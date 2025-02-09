use axum::{routing::get, Router};
use axum_server::{
    db::db::init_db_pool, routes::main_router::main_router, utils::cors::create_cors_layer,
};

#[tokio::main]
async fn main() {
    let pool = init_db_pool().await;
    // let _ = delete_all_db(&pool).await;

    let cors = create_cors_layer();
    // build our application with a single route
    let app = Router::new()
        .nest("/application", main_router(pool))
        .route("/", get(|| async { "Welcome to the API" }))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
