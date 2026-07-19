use crate::models::{auth::Claims, user::User};
use crate::repositories::user_repository;
use crate::response::AppError;
use crate::validation::auth::{SigninRequest, SignupRequest, UpdateProfileRequest};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

fn map_db_error(err: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db_err) = &err {
        if db_err.is_unique_violation() {
            return match db_err.constraint() {
                Some("users_username_key") => AppError::Conflict("username already in use".into()),
                _ => AppError::Conflict("email already in use".into()),
            };
        }
    }
    AppError::Internal(err.to_string())
}

fn generate_token(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub async fn signup(
    pool: &PgPool,
    req: SignupRequest,
    jwt_secret: &str,
) -> Result<(User, String), AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();

    let user = user_repository::create_user(pool, &req.username, &req.email, &password_hash)
        .await
        .map_err(map_db_error)?;

    let token = generate_token(user.id, jwt_secret)?;
    Ok((user, token))
}

pub async fn signin(
    pool: &PgPool,
    req: SigninRequest,
    jwt_secret: &str,
) -> Result<(User, String), AppError> {
    let user = user_repository::find_by_email(pool, &req.email)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("invalid email or password".into()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("invalid email or password".into()))?;

    let token = generate_token(user.id, jwt_secret)?;
    Ok((user, token))
}

pub async fn update_profile(
    pool: &PgPool,
    user_id: Uuid,
    req: UpdateProfileRequest,
) -> Result<User, AppError> {
    user_repository::update_profile(pool, user_id, req.username.as_deref(), req.email.as_deref())
        .await
        .map_err(map_db_error)
}
