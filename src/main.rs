use axum::{routing::get, Router};
use axum_server::routes::user::user_router;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest("/user", user_router())
        .route("/", get(|| async { "Welcome to the API" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
