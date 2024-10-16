use crate::models::user::{LoginUser, RegisterUser};
use crate::services::user::{info, login, register};
use crate::utils::responses::ApiResponse;
use axum::Extension;
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
pub async fn user_info(
    Extension(auth_token): Extension<String>,
) -> Result<impl IntoResponse, StatusCode> {
    if info(auth_token).await.is_ok() {
        Ok(ApiResponse::success("Usuario verificado correctamente"))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
