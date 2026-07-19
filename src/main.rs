mod db;
mod handlers;
mod models;
mod repositories;
mod response;
mod routes;
mod services;
mod state;
mod validation;

use state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::connect().await;
    let jwt_secret: Arc<str> = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET not set")
        .into();

    let state = AppState { pool, jwt_secret };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = routes::routes().with_state(state).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
