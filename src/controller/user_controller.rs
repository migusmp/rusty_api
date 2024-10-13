use crate::models::user::{LoginUser, RegisterUser};
use crate::services::user::{login, register};
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
