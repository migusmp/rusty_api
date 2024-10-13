use crate::controller::user_controller::*;
use axum::{routing::post, Router};

pub fn user_router() -> Router {
    Router::new()
        .route("/register", post(user_register))
        .route("/login", post(user_login))
}
