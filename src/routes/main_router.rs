use super::{chat::chat_router, friend::friend_router, user::user_router};
use crate::{models::chat::ChatState, state::app_state::AppState};
use axum::Router;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::RwLock;

pub fn main_router(
    pool: Arc<Pool<Postgres>>,
    chat_state: Arc<RwLock<ChatState>>,
    app_state: Arc<AppState>,
) -> Router {
    Router::new()
        // Poner ruta para manejar la conexión websocket con el cliente y registrarlo en todos los
        // canales globales del AppState.
        .nest("/user", user_router(pool.clone()))
        .nest("/chat", chat_router(chat_state.clone(), pool.clone()))
        .nest("/friend", friend_router(pool.clone(), app_state.clone()))
}
