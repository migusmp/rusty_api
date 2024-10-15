use crate::controller::user_controller::*;
use crate::middlewares::auth::auth;
use axum::routing::{get, post};
use axum::Router;

pub fn user_router() -> Router {
    Router::new()
        .route("/register", post(user_register))
        .route("/login", post(user_login))
        .route(
            "/info",
            get(user_info).route_layer(axum::middleware::from_fn(auth)),
        )
}
