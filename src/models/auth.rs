use crate::response::AppError;
use crate::state::AppState;
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Extractor that pulls the authenticated user's id out of a `Bearer` JWT.
pub struct AuthUser(pub Uuid);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let unauthorized =
            || AppError::Unauthorized("missing or invalid authorization token".into());

        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(unauthorized)?;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| unauthorized())?;

        let user_id = Uuid::parse_str(&data.claims.sub).map_err(|_| unauthorized())?;
        Ok(AuthUser(user_id))
    }
}
