use crate::controller::user_controller::*;
use crate::middlewares::auth::auth;
use axum::routing::{get, post, put};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub fn user_router(pool: Arc<PgPool>) -> Router {
    let pool_login = pool.clone();
    let pool_get_friends = pool.clone();
    Router::new()
        .route(
            "/register",
            post({
                let pool = pool.clone();
                move |data| user_register(data, pool)
            }),
        )
        .route("/login", post(move |data| user_login(data, pool_login)))
        .route(
            "/logout",
            post(user_logout).route_layer(axum::middleware::from_fn(auth)),
        )
        .route(
            "/info",
            get(user_info).route_layer(axum::middleware::from_fn(auth)),
        )
        .route(
            "/get-friends",
            get(move |payload| get_friends(payload, pool_get_friends))
                .route_layer(axum::middleware::from_fn(auth)),
        )
        .route(
            "/update",
            put({
                let pool = pool.clone();
                move |payload, path| user_update(payload, path, pool)
            })
            .route_layer(axum::middleware::from_fn(auth)),
        )
        .route("/upload", post(upload_image))
}
