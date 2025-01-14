use std::sync::Arc;

use crate::controller::user_controller::*;
use crate::middlewares::auth::auth;
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;

pub fn user_router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/register",
            post({
                let pool = pool.clone();
                move |data| user_register(data, pool)
            }),
        )
        .route("/login", post(move |data| user_login(data, pool.clone())))
        .route(
            "/logout",
            post(user_logout).route_layer(axum::middleware::from_fn(auth)),
        )
        .route(
            "/info",
            get(user_info).route_layer(axum::middleware::from_fn(auth)),
        )
        .route("/upload", post(upload_image))
}
