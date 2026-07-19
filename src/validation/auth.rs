use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(length(
        min = 3,
        max = 32,
        message = "username must be between 3 and 32 characters"
    ))]
    pub username: String,
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SigninRequest {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(
        min = 3,
        max = 32,
        message = "username must be between 3 and 32 characters"
    ))]
    pub username: Option<String>,
    #[validate(email(message = "must be a valid email address"))]
    pub email: Option<String>,
}
