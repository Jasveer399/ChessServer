use crate::models::{auth::AuthUser, user::User};
use crate::response::{ApiResponse, AppError};
use crate::services::auth_service;
use crate::state::AppState;
use crate::validation::auth::{SigninRequest, SignupRequest, UpdateProfileRequest};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use validator::Validate;

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()?;

    let (user, token) = auth_service::signup(&state.pool, req, &state.jwt_secret).await?;
    Ok((
        StatusCode::CREATED,
        ApiResponse::ok(AuthResponse { token, user }),
    ))
}

pub async fn signin(
    State(state): State<AppState>,
    Json(req): Json<SigninRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()?;

    let (user, token) = auth_service::signin(&state.pool, req, &state.jwt_secret).await?;
    Ok(ApiResponse::ok(AuthResponse { token, user }))
}

pub async fn update_profile(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()?;

    let user = auth_service::update_profile(&state.pool, user_id, req).await?;
    Ok(ApiResponse::ok(user))
}
