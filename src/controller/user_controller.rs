use crate::models::user::{LoginUser, RegisterUser};
use crate::services::user::register;
use axum::{http::StatusCode, response::IntoResponse, Form, Json};

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
    let login_user = LoginUser::new(username.to_string(), password.to_string());
    Ok((
        StatusCode::OK,
        Json("Login route: ".to_string() + &login_user.username),
    ))
}
