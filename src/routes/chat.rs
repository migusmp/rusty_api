use std::sync::{Arc, Mutex};

use axum::{middleware::from_fn, routing::post, Extension, Router};

use crate::middlewares::auth::auth;
use crate::{controller::chat_controller::create_chat, models::chat::ChatState};

pub fn chat_router(state: Arc<Mutex<ChatState>>) -> Router {
    Router::new()
        .route(
            "/create/:room_id",
            post(create_chat).route_layer(from_fn(auth)),
        )
        .layer(Extension(state))
}
