use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;

use crate::{db::db::init_db_pool, models::chat::ChatState, state::app_state::AppState};

pub mod cors;
pub mod jwt;
pub mod responses;
pub mod user_utils;

pub struct SharedState {
    pub pool: Arc<Pool<Postgres>>,
    pub chat_state: Arc<RwLock<ChatState>>,
    pub app_state: Arc<AppState>,
}

impl SharedState {
    pub async fn new() -> Self {
        let chat_state = Arc::new(RwLock::new(ChatState::default()));
        let app_state = Arc::new(AppState::new());
        let pool = init_db_pool().await;

        Self {
            chat_state,
            app_state,
            pool,
        }
    }
}
