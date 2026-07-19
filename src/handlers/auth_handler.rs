use crate::models::{auth::AuthUser, user::User};
use crate::services::auth_service::{self, AuthError};
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
) -> Result<impl IntoResponse, AuthError> {
    req.validate()
        .map_err(|e| AuthError::Validation(e.to_string()))?;

    let (user, token) = auth_service::signup(&state.pool, req, &state.jwt_secret).await?;
    Ok((StatusCode::CREATED, Json(AuthResponse { token, user })))
}

pub async fn signin(
    State(state): State<AppState>,
    Json(req): Json<SigninRequest>,
) -> Result<impl IntoResponse, AuthError> {
    req.validate()
        .map_err(|e| AuthError::Validation(e.to_string()))?;

    let (user, token) = auth_service::signin(&state.pool, req, &state.jwt_secret).await?;
    Ok(Json(AuthResponse { token, user }))
}

pub async fn update_profile(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AuthError> {
    req.validate()
        .map_err(|e| AuthError::Validation(e.to_string()))?;

    let user = auth_service::update_profile(&state.pool, user_id, req).await?;
    Ok(Json(user))
}
