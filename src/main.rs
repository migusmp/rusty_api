use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use axum_server::{
    models::chat::ChatState,
    routes::{chat::chat_router, user::user_router},
};

#[tokio::main]
async fn main() {
    let chat_state = Arc::new(Mutex::new(ChatState::default()));

    // build our application with a single route
    let app = Router::new()
        .nest("/user", user_router())
        .nest("/chat", chat_router(chat_state.clone()))
        .route("/", get(|| async { "Welcome to the API" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
