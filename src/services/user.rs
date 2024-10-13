use crate::models::user::LoginUser;
use crate::utils::responses::{error_response, success_response};
use crate::{db::connection::open_users_db, models::user::RegisterUser};
use axum::{http::StatusCode, response::IntoResponse};
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

    // validamos que el usuario exista
    if !verify_user_login(&user, &conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Ok(error_response(
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password",
        ));
    }

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

fn verify_user_login(user: &LoginUser, conn: &Connection) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT COUNT (*) FROM users WHERE name = ?1 AND password = ?2")
        .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;
    let user_exists: i32 = stmt
        .query_row(params![&user.username, &user.password], |row| row.get(0))
        .map_err(|e| {
            eprintln!("Error al verificar usuario: {}", e);
            rusqlite::Error::QueryReturnedNoRows
        })?;

    Ok(user_exists > 0)
}
