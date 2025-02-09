use super::user::user_router;
use axum::Router;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub fn main_router(pool: Arc<Pool<Postgres>>) -> Router {
    Router::new().nest("/user", user_router(pool))
}
