use axum::{extract::rejection::PathRejection, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use super::error_format::RejectionResponseError;

#[derive(Serialize)]
pub struct CustomPathDataRejection {
    pub message: String,
}

impl From<PathRejection> for CustomPathDataRejection {
    fn from(value: PathRejection) -> Self {
        match value {
            PathRejection::FailedToDeserializePathParams(err) => CustomPathDataRejection {
                message: err.to_string(),
            },
            PathRejection::MissingPathParams(err) => CustomPathDataRejection {
                message: err.to_string(),
            },
            _ => CustomPathDataRejection {
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl IntoResponse for CustomPathDataRejection {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            axum::Json(RejectionResponseError {
                http_code: 400,
                error: self.message,
            }),
        )
            .into_response()
    }
}
