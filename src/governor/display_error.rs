use http::{Response, StatusCode};
use serde::Serialize;
use serde_json::to_string;
use tower::BoxError;
use tower_governor::GovernorError;

#[derive(Serialize)]
pub struct ErrorHandler {
    pub http_code: u16,
    pub error: String,
}

pub fn display_error(e: BoxError) -> Response<String> {
    let error_handler: ErrorHandler;

    if e.is::<GovernorError>() {
        let error = e.downcast_ref::<GovernorError>().unwrap().to_owned();
        match error {
            GovernorError::TooManyRequests { headers, .. } => {
                error_handler = ErrorHandler {
                    http_code: 429,
                    error: "Too Many Requests!".to_string(),
                };
                let response = Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(to_string(&error_handler).unwrap())
                    .unwrap();
                let (mut parts, body) = response.into_parts();
                if let Some(headers) = headers {
                    parts.headers = headers;
                    // Apply content type header
                    parts
                        .headers
                        .insert("Content-Type", "application/json".parse().unwrap());
                }
                Response::from_parts(parts, body)
            }
            GovernorError::UnableToExtractKey => {
                error_handler = ErrorHandler {
                    http_code: 500,
                    error: "Unable To Extract Key!".to_string(),
                };
                let json = to_string(&error_handler).unwrap();
                let response = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "application/json")
                    .body(json)
                    .unwrap();
                Response::from(response)
            }
            GovernorError::Other { code, msg, .. } => {
                error_handler = ErrorHandler {
                    http_code: code.as_u16(),
                    error: msg.unwrap_or_else(|| "Other Error!".to_string()),
                };
                let json = to_string(&error_handler).unwrap();
                let response = Response::builder()
                    .status(code)
                    .header("Content-Type", "application/json")
                    .body(json)
                    .unwrap();
                Response::from(response)
            }
        }
    } else {
        error_handler = ErrorHandler {
            http_code: 500,
            error: "Internal Server Error!".to_string(),
        };
        let response = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(to_string(&error_handler).unwrap())
            .unwrap();
        Response::from(response)
    }
}
