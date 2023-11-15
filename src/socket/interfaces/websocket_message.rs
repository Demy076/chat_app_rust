use serde::{
    ser::{Serialize, Serializer},
    Serialize as SerializeM,
};

pub enum Records {
    Message,
    JoinedQueue,
    LeftQueue,
}

impl Serialize for Records {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Records::Message => serializer.serialize_str("msg_g2c_send_message"),
            Records::JoinedQueue => serializer.serialize_str("msg_g2c_joined_queue"),
            Records::LeftQueue => serializer.serialize_str("msg_g2c_left_queue"),
        }
    }
}

#[derive(SerializeM)]
pub struct WebSocketMessage {
    pub record: Records,
    pub queue: String,
    pub message: serde_json::Value,
}
