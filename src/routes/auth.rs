use crate::handlers::auth_handler;
use crate::state::AppState;
use axum::{
    routing::{post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/signup", post(auth_handler::signup))
        .route("/auth/signin", post(auth_handler::signin))
        .route("/auth/profile", put(auth_handler::update_profile))
}
