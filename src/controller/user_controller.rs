use std::sync::Arc;

use crate::models::user::{ErrorRequest, LoginUser, Payload, RegisterUser};
use crate::services::user::{login, register};
use crate::utils::responses::ApiResponse;
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, Form};
use sqlx::PgPool;

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
    Extension(payload): Extension<Payload>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("User ID: {}", payload.id);
    println!("User name: {}", payload.name);

    Ok(ApiResponse::success("Usuario verificado correctamente"))
}
