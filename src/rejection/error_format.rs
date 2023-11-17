use serde::Serialize;

#[derive(Serialize)]
pub struct RejectionResponseError {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}
