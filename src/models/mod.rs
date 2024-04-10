use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::error::CarSharingError;

pub mod car;
pub mod order;
pub mod session_token;
pub mod user;

#[derive(Debug)]
pub enum AuthError {
    TelegramHashProblem,
    InternalServerError,
    CarSharingError(CarSharingError),
    AuthFailNoUserDataInRequest,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::CarSharingError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };

        (status,
         Json(
             json!({"resource":"PostModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
         ),
        )
            .into_response()
    }
}
