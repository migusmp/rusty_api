use std::borrow::Cow;

use axum::{http::StatusCode, response::Response};
use bcrypt::BcryptError;
use chrono::{Duration, Utc};
use cookie::Cookie;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use rusqlite::{params, Connection};

use crate::models::user::{LoginUser, Payload, RegisterUser, User};

// Verificamos que el usuario exista
pub fn verify_user_exists(user: &RegisterUser, conn: &Connection) -> Result<bool, rusqlite::Error> {
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

// Función para verificar que el usuario y la contraseña son correctos
pub fn verify_user_login(
    user_login: &LoginUser,
    conn: &Connection,
) -> Result<bool, rusqlite::Error> {
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

pub fn hashed_pwd(pwd: &String, hashed_pwd: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(pwd, hashed_pwd)
}

// Guardamos al usuario en la BBDD.
pub fn insert_user(
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

// Función para obtener todos los datos de un determinado usuario en la BBDD.
pub fn get_user_full_data(user: &LoginUser, conn: &Connection) -> Result<User, rusqlite::Error> {
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

pub fn create_payload(user_data: User) -> Result<String, jsonwebtoken::errors::Error> {
    let iat = Utc::now().timestamp(); // tiempo actual
    let exp = (Utc::now() + Duration::hours(1)).timestamp(); // 1 hora de tiempo de expiración.
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
}

pub fn create_token_cookie<'a>(name_cookie: &'a str, token: Cow<'a, str>) -> Cookie<'a> {
    Cookie::build((name_cookie, token))
        .http_only(true) // Evita que sea accesible desde JavaScript.
        .path("/")
        .build()
}

pub fn append_cookie_to_response(res: &mut Response, cookie: Cookie) {
    res.headers_mut().append(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
}

pub async fn decode_token(auth_token: String) -> Result<Payload, StatusCode> {
    let token_data: TokenData<Payload> = jsonwebtoken::decode(
        &auth_token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        eprintln!("Error al decodificar el token: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(token_data.claims)
}
