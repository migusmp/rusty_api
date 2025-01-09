use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{routing::get, Router};
use axum_server::{
    db::db::{delete_all_users, init_db_pool},
    models::chat::ChatState,
    routes::{chat::chat_router, user::user_router},
    utils::cors::create_cors_layer,
};

#[tokio::main]
async fn main() {
    let pool = init_db_pool().await;
    let _ = delete_all_users(&pool).await;

    let chat_state = Arc::new(RwLock::new(ChatState::default()));

    let cors = create_cors_layer();
    // build our application with a single route
    let app = Router::new()
        .nest("/user", user_router(pool.clone()))
        .nest("/chat", chat_router(chat_state.clone()))
        .route("/", get(|| async { "Welcome to the API" }))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
