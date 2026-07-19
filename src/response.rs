use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use validator::ValidationErrors;

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
    Validation(HashMap<String, String>),
    Internal(String),
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let details = errors
            .field_errors()
            .into_iter()
            .map(|(field, errs)| {
                let message = errs
                    .first()
                    .and_then(|e| e.message.clone())
                    .unwrap_or_else(|| errs.first().map(|e| e.code.clone()).unwrap_or_default());
                (field.to_string(), message.to_string())
            })
            .collect();
        AppError::Validation(details)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg, None),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg, None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg, None),
            AppError::Validation(details) => (
                StatusCode::BAD_REQUEST,
                "validation failed".to_string(),
                Some(details),
            ),
            AppError::Internal(msg) => {
                tracing::error!("{msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                    None,
                )
            }
        };

        let mut body = json!({ "success": false, "error": message });
        if let Some(details) = details {
            body["details"] = json!(details);
        }
        (status, Json(body)).into_response()
    }
}
