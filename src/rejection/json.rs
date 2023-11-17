use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};

use super::error_format::RejectionResponseError;

pub struct CustomJsonDataRejection {
    pub message: String,
}

impl From<JsonRejection> for CustomJsonDataRejection {
    fn from(value: JsonRejection) -> Self {
        // Just convert to_string() and return
        // the error message
        match value {
            JsonRejection::BytesRejection(err) => CustomJsonDataRejection {
                message: err.to_string(),
            },
            JsonRejection::JsonSyntaxError(err) => CustomJsonDataRejection {
                message: err.to_string(),
            },
            JsonRejection::JsonDataError(err) => CustomJsonDataRejection {
                message: err.to_string(),
            },
            JsonRejection::MissingJsonContentType(err) => CustomJsonDataRejection {
                message: err.to_string(),
            },
            _ => CustomJsonDataRejection {
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl IntoResponse for CustomJsonDataRejection {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            axum::Json(RejectionResponseError {
                success: false,
                http_code: 422,
                error: self.message,
            }),
        )
            .into_response()
    }
}
