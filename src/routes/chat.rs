use axum::routing::get;
use axum::{middleware::from_fn, routing::post, Extension, Router};
use std::sync::{Arc, Mutex};

use crate::controller::chat_controller::*;
use crate::middlewares::auth::auth;
use crate::models::chat::ChatState;

pub fn chat_router(state: Arc<Mutex<ChatState>>) -> Router {
    Router::new()
        .route(
            "/create/:room_id",
            post(create_chat).route_layer(from_fn(auth)),
        )
        .route("/join/:room_id", get(join_chat).route_layer(from_fn(auth)))
        //.route("/join/:room_id", post(join_chat))
        .layer(Extension(state))
}
