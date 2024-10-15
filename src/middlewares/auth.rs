use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::services::user::info;

pub async fn auth<B>(req: Request<B>, next: Next) -> impl IntoResponse {
    let headers = req.headers().clone();
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(";") {
                let cookie = cookie.trim();
                if let Some(auth_token) = cookie.strip_prefix("auth=") {
                    match info(auth_token.to_string()).await {
                        Ok(_) => {
                            let mut req = req.map(|_body| Body::empty());
                            req.extensions_mut().insert(auth_token.to_string());
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
