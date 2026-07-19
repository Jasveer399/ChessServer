mod auth;
mod health;

use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().merge(health::routes()).merge(auth::routes())
}
