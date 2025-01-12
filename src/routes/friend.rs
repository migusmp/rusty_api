use std::sync::Arc;

use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::{
    controller::friend_controller::send_friend_request, middlewares::auth::auth,
    state::app_state::AppState,
};

pub fn friend_router(pool: Arc<PgPool>, app_state: Arc<AppState>) -> Router {
    Router::new().route(
        "/add/:friend_id",
        post({
            let pool_for_friend_add = pool.clone();
            move |friend_id, data| {
                send_friend_request(pool_for_friend_add, friend_id, data, app_state)
            }
        })
        .route_layer(axum::middleware::from_fn(auth)),
    )
}
