use crate::models::user::{LoginUser, RegisterUser};
use crate::services::user::{info, login, register};
use axum::http::HeaderMap;
use axum::{http::StatusCode, response::IntoResponse, Form};

pub async fn user_register(
    Form(data): Form<RegisterUser>,
) -> Result<impl IntoResponse, StatusCode> {
    // Accedemos a los datos del usuario
    let username = &data.username;
    let email = &data.email;
    let password = &data.password;

    // Llamamos al servicio de registro
    let new_user = RegisterUser::new(
        username.to_string(),
        email.to_string(),
        password.to_string(),
    );

    register(new_user).await
}

pub async fn user_login(Form(data): Form<LoginUser>) -> Result<impl IntoResponse, StatusCode> {
    // Accedemos a los datos del usuario
    let username = &data.username;
    let password = &data.password;

    // Llamamos al servicio de registro
    let user = LoginUser::new(username.to_string(), password.to_string());

    login(user).await
}

// Hacer ruta de logout

// Ruta de informacion de usuario (Probar decodear el payload)
pub async fn user_info(headers: HeaderMap) -> Result<impl IntoResponse, StatusCode> {
    // Intentar obtener la cookie "auth" desde los headers
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Buscar la cookie "auth" en la cadena de cookies
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(auth_token) = cookie.strip_prefix("auth=") {
                    return info(auth_token.to_string()).await;
                }
            }
        }
    }

    // Si no se encuentra la cookie "auth"
    Err(StatusCode::UNAUTHORIZED)
}
