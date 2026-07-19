use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Json<Self> {
        Json(Self {
            success: true,
            data,
        })
    }
}

pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(msg) => {
                tracing::error!("{msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };
        (status, Json(json!({ "success": false, "error": message }))).into_response()
    }
}
