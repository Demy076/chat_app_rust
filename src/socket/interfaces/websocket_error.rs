use serde::Serialize;

#[derive(Serialize)]
pub struct WebSocketError {
    pub record: String,
    pub message: String,
}
