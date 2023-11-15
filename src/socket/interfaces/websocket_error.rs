use serde::Serialize;

use super::websocket_message::Records;

#[derive(Serialize)]
pub struct WebSocketError {
    pub record: Records,
    pub message: serde_json::Value,
}
