use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::services::user::decode_token;
// B es un tipo genérico, ya que la request nos puede devolver cualquier valor
pub async fn auth<B>(req: Request<B>, next: Next) -> impl IntoResponse {
    // Obtenemos todos los headers de la petición.
    let headers = req.headers().clone();

    // Si esta el header 'cookie' ejecutamos el siguiente codigo
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Spliteamos todas las cookies y obtenemos la cookie 'auth='
            for cookie in cookie_str.split(";") {
                let cookie = cookie.trim();
                if let Some(auth_token) = cookie.strip_prefix("auth=") {
                    // Si hay algún 'auth=' lo decodificamos y verificamos que sea correcto.
                    match decode_token(auth_token.to_string()).await {
                        Ok(payload) => {
                            let mut req = req.map(|_body| Body::empty());
                            // Insertamos el payload como extension en los headers.
                            req.extensions_mut().insert(payload);
                            return next.run(req).await;
                        }
                        Err(_) => {
                            return StatusCode::UNAUTHORIZED.into_response();
                        }
                    };
                };
            }
        }
    }

    StatusCode::UNAUTHORIZED.into_response()
}
