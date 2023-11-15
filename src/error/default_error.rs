use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct MethodError {
    pub http_code: u16,
    pub error: String,
}

pub async fn default_error() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(MethodError {
            http_code: 404,
            error: "Not Found".to_string(),
        }),
    )
}
