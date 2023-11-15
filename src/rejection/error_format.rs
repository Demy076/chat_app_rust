use serde::Serialize;

#[derive(Serialize)]
pub struct RejectionResponseError {
    pub http_code: u16,
    pub error: String,
}
