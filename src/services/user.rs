use crate::models::user::{LoginUser, Payload};
use crate::utils::responses::ApiResponse;
use crate::utils::user_utils::{
    create_payload, create_token_cookie, get_user_full_data, insert_user, verify_user_exists,
    verify_user_login,
};
use crate::{db::connection::open_users_db, models::user::RegisterUser};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use jsonwebtoken::TokenData;
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::json;

pub async fn register(user: RegisterUser) -> Result<impl IntoResponse, StatusCode> {
    // Establecemos la conexión con la base de datos
    let conn = open_users_db().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // validamos que el usuario no exista
    if verify_user_exists(&user, &conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Ok(ApiResponse::error(
            StatusCode::BAD_REQUEST,
            "User already exists",
        ));
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
    Ok(ApiResponse::success("User created successfully"))
}

pub async fn login(user: LoginUser) -> Result<impl IntoResponse, StatusCode> {
    let conn = open_users_db().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verificamos que el usuario y la contraseña son correctos.
    if !verify_user_login(&user, &conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Ok(ApiResponse::error(
            StatusCode::UNAUTHORIZED,
            "Invalid credentials",
        ));
    }
    // Recogemos la información de la BBDD del usuario.
    let user_data = tokio::task::spawn_blocking(move || get_user_full_data(&user, &conn))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Creamos el payload.
    let token = tokio::task::spawn_blocking(move || create_payload(user_data))
        .await
        .map_err(|_| {
            eprintln!("Error al crear el token");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map_err(|_| {
            eprintln!("Error al crear el payload");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Creamos la cookie con el token.
    let token_cookie = create_token_cookie("auth", std::borrow::Cow::Owned(token)); // Si la contraseña y el usuario son correctos creamos el token de seguridad.
    Ok(ApiResponse::SuccessWithCookie(
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Login successful",
        })),
        token_cookie,
    ))
}

pub async fn info(auth_token: String) -> Result<impl IntoResponse, StatusCode> {
    let token_data: TokenData<Payload> = jsonwebtoken::decode(
        &auth_token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        eprintln!("Error al decodificar el token: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    println!("Token: {:?}", token_data.claims);

    //let user_data = tokio::task::spawn_blocking(move || get_user_full_data(&auth_token, &conn))
    //    .await
    //    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    //    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::success("User info"))
}
