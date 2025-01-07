use axum::routing::get;
use axum::{middleware::from_fn, routing::post, Extension, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::controller::chat_controller::*;
use crate::middlewares::auth::auth;
use crate::models::chat::ChatState;

pub fn chat_router(state: Arc<RwLock<ChatState>>) -> Router {
    Router::new()
        .route(
            "/create/:room_id",
            post(create_chat).route_layer(from_fn(auth)),
        )
        .route("/join/:room_id", get(join_chat).route_layer(from_fn(auth)))
        .route("/stats/:room_id", get(get_room_stats).route_layer(from_fn(auth)))
        .route("/active-rooms", get(get_active_rooms).route_layer(from_fn(auth)))
        .layer(Extension(state))
}
