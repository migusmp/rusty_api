use std::sync::Arc;

use crate::db::db::{update_user_name, update_user_pwd, UpdatePassword, UpdateUserName};
use crate::models::user::{ErrorRequest, LoginUser, Payload, RegisterUser, UpdateData};
use crate::services::user::{login, register};
use crate::utils::responses::ApiResponse;
use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, Form};
use sqlx::PgPool;
use tokio::fs;
use uuid::Uuid;

const MAX_CONTENT_LENGTH: u64 = 10 * 1024 * 1024; // 10 MB

// Ruta de registro de usuarios.
pub async fn user_register(
    Form(data): Form<RegisterUser>,
    pool: Arc<PgPool>,
) -> Result<impl IntoResponse, ErrorRequest> {
    // Accedemos a los datos del usuario
    let username = &data.username;
    let email = &data.email;
    let password = &data.password;

    if username.trim().is_empty() {
        return Err(ErrorRequest::UsernameEmpty);
    }

    if username.len() < 3 {
        return Err(ErrorRequest::UsernameInvalid);
    }

    if !email.contains("@") {
        return Err(ErrorRequest::EmailInvalid);
    }

    if password.len() < 4 {
        return Err(ErrorRequest::PasswordInvalid);
    }

    // Llamamos al servicio de registro
    let new_user = RegisterUser::new(
        username.to_string(),
        email.to_string(),
        password.to_string(),
    );

    Ok(register(new_user, &pool).await)
}

// Ruta de inicio de sesión de usuarios.
pub async fn user_login(
    Form(data): Form<LoginUser>,
    pool: Arc<PgPool>,
) -> Result<impl IntoResponse, ErrorRequest> {
    // Accedemos a los datos del usuario
    let username = &data.username;
    let password = &data.password;

    // Llamamos al servicio de registro
    let user = LoginUser::new(username.to_string(), password.to_string());

    Ok(login(user, &pool).await)
}

// Hacer ruta de logout
pub async fn user_logout() -> Result<impl IntoResponse, StatusCode> {
    let response = (
        StatusCode::OK,
        [("Set-Cookie", "auth=; Max-Age=0; Path=/; HttpOnly")],
    );
    Ok(response)
}

// Ruta de informacion de usuario (Probar decodear el payload)
pub async fn user_info(
    Extension(_payload): Extension<Payload>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(ApiResponse::success("Usuario verificado correctamente"))
}

pub async fn user_update(
    Extension(payload): Extension<Payload>,
    Form(update_info): Form<UpdateData>,
    pool: Arc<PgPool>,
) -> Result<impl IntoResponse, ErrorRequest> {
    if let Some(username) = update_info.username {
        match update_user_name(username, payload.id, &pool).await {
            UpdateUserName::UserNameUpdated => {}
            UpdateUserName::UserExists => {
                return Err(ErrorRequest::UserAlreadyExists);
            }
            UpdateUserName::ConsultError => {
                return Err(ErrorRequest::InternalError);
            }
        }
    }

    // TODO
    if let Some(pwd) = update_info.password {
        match update_user_pwd(pwd, payload.id, &pool).await {
            UpdatePassword::PasswordUpdated => println!("Password updated"),
            UpdatePassword::ErrorPasswordUpdate => return Err(ErrorRequest::ErrorPasswordUpdate),
        }
    }

    match update_info.email {
        Some(email) => {
            println!("new email: {}", email);
        }
        None => println!("No email in request"),
    }

    Ok(ApiResponse::success("update endpoint is working"))
}

pub async fn upload_image(
    headers: HeaderMap,
    multipart: Multipart,
) -> Result<impl IntoResponse, ErrorRequest> {
    if let Some(content_length) = headers.get("content-length") {
        let content_length = content_length
            .to_str()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        if content_length > MAX_CONTENT_LENGTH {
            return Err(ErrorRequest::UsernameEmpty);
        }
    }

    let mut file_name = String::new();

    let mut fields = multipart; // Procesamos el contenido del multipart.

    while let Some(field) = fields.next_field().await.map_err(|e| {
        eprintln!("Error: {:?}", e);
        ErrorRequest::InternalError
    })? {
        let _name = field.name().unwrap_or("file");
        let content_type = field.content_type().unwrap_or("application/octet-stream");

        // Check this file is an image.
        if !content_type.starts_with("image/") {
            return Err(ErrorRequest::InvalidImageFormat);
        }

        let file_extension = match content_type.split('/').last() {
            Some(ext) => ext,
            None => return Err(ErrorRequest::InvalidImageFormat),
        };

        file_name = format!("{}.{}", Uuid::new_v4(), file_extension);

        // Guardar el archivo.
        let data = field.bytes().await.map_err(|e| {
            eprintln!("Error to save uploaded file: {:?}", e);
            ErrorRequest::InternalError
        })?;

        let path = format!("./uploads/user/{}", file_name);
        fs::write(&path, &data).await.map_err(|e| {
            eprintln!("Error al guardar el archivo: {:?}", e);
            ErrorRequest::InternalError
        })?;
    }
    println!("{:?}", file_name);
    Ok(ApiResponse::success("Image uploaded successfully"))
}
