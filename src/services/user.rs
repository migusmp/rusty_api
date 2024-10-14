use crate::models::user::{LoginUser, Payload, User};
use crate::utils::responses::{error_response, success_response};
use crate::{db::connection::open_users_db, models::user::RegisterUser};
use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use chrono::{Duration, Utc};
use rusqlite::{params, Connection};

pub async fn register(user: RegisterUser) -> Result<impl IntoResponse, StatusCode> {
    // Establecemos la conexión con la base de datos
    let conn = open_users_db().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // validamos que el usuario no exista
    if verify_user_exists(&user, &conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Ok(error_response(StatusCode::CONFLICT, "User already exists"));
    }

    // hasheamos la contraseña en otro hilo asincrono para mejorar el rendimiento y que el hashing
    // no bloquee otras acciones del enpoint.
    let hashed_pwd = tokio::task::spawn_blocking(move || bcrypt::hash(&user.password, 4))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Error en la tarea asíncrona
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Error en el proceso de hash

    // insertamos el usuario
    insert_user(&user.username, &user.email, &conn, hashed_pwd).map_err(|err| {
        eprintln!("Error al insertar: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Devolvemos la success response.
    Ok(success_response(
        StatusCode::CREATED,
        "User created successfully",
    ))
}

pub async fn login(user: LoginUser) -> Result<impl IntoResponse, StatusCode> {
    let conn = open_users_db().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verificamos que el usuario y la contraseña son correctos.
    if !verify_user_login(&user, &conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Ok(error_response(
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password",
        ));
    }
    // Recogemos la información de la BBDD del usuario.
    let user_data = tokio::task::spawn_blocking(move || get_user_full_data(&user, &conn))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Creamos el payload.
    let payload = match tokio::task::spawn_blocking(move || {
        let iat = Utc::now().timestamp().to_string(); // Tiempo actual
        let exp = (Utc::now() + Duration::hours(1)).timestamp().to_string(); // 1 hora de tiempo de expiración.
        let user_payload = Payload::new(
            user_data.id,
            user_data.name,
            user_data.email,
            user_data.password,
            user_data.created_at,
            exp,
            iat,
        );
        user_payload.token()
    })
    .await
    {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error al generar el token");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    eprintln!("{:?}", payload);
    // Si la contraseña y el usuario son correctos creamos el token de seguridad.
    Ok(success_response(StatusCode::OK, "User logged successfully"))
}

// Verificamos que el usuario exista
fn verify_user_exists(user: &RegisterUser, conn: &Connection) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT COUNT (*) FROM users WHERE email = ?1 OR name = ?2")
        .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;
    let user_exists: i32 = stmt
        .query_row(params![&user.email, &user.username], |row| row.get(0))
        .map_err(|e| {
            eprintln!("Error al verificar usuario: {}", e);
            rusqlite::Error::QueryReturnedNoRows
        })?;

    Ok(user_exists > 0)
}

// Guardamos al usuario en la BBDD.
fn insert_user(
    username: &String,
    email: &String,
    conn: &Connection,
    hashed_pwd: String,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO users (name, email, password) VALUES (?1, ?2, ?3)",
        params![username, email, hashed_pwd],
    )
    .map_err(|e| {
        eprintln!("Error al insertar usuario: {}", e);
        e
    })?;

    Ok(())
}

// Función para verificar que el usuario y la contraseña son correctos
fn verify_user_login(user_login: &LoginUser, conn: &Connection) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT password FROM users WHERE name = ?1")
        .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;

    let stored_password: String = stmt
        .query_row(params![&user_login.username], |row| row.get(0))
        .map_err(|e| {
            eprintln!("Error al verificar usuario: {}", e);
            rusqlite::Error::QueryReturnedNoRows
        })?;

    if hashed_pwd(&user_login.password, &stored_password).unwrap() {
        Ok(true)
    } else {
        Ok(false)
    }
}
fn hashed_pwd(pwd: &String, hashed_pwd: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(pwd, hashed_pwd)
}

fn get_user_full_data(user: &LoginUser, conn: &Connection) -> Result<User, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT id, name, email, password, created_at FROM users WHERE name = ?1")
        .map_err(|e| {
            eprintln!("Error al preparar la consulta: {}", e);
            e
        })?;

    let user_data = stmt
        .query_row(params![&user.username], |row| {
            Ok(User {
                id: row.get::<_, i64>(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                password: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| {
            eprintln!("Error al obtener los datos del usuario: {}", e);
            e
        })?;

    Ok(user_data)
}
