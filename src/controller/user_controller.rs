use crate::models::user::{LoginUser, Payload, RegisterUser};
use crate::services::user::{login, register};
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
