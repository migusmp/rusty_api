use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://127.0.0.1:5500".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, COOKIE])
}
