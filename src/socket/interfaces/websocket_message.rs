use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
    Serialize as SerializeM,
};

pub enum Records {
    Message,
    JoinedQueue,
    LeftQueue,
    RateLimit,
}

impl Serialize for Records {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Records::RateLimit => serializer.serialize_str("msg_g2c_ratelimit"),
            Records::Message => serializer.serialize_str("msg_g2c_send_message"),
            Records::JoinedQueue => serializer.serialize_str("msg_g2c_joined_queue"),
            Records::LeftQueue => serializer.serialize_str("msg_g2c_left_queue"),
        }
    }
}

impl<'de> Deserialize<'de> for Records {
    fn deserialize<D>(deserializer: D) -> Result<Records, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "msg_c2g_send_message" => Ok(Records::Message),
            "msg_c2g_subscribe_queue" => Ok(Records::JoinedQueue),
            "msg_c2g_unsubscribe_queue" => Ok(Records::LeftQueue),
            _ => Err(serde::de::Error::custom("expected a valid record")),
        }
    }
}

#[derive(SerializeM)]
pub struct WebSocketMessage {
    pub record: Records,
    pub queue: String,
    pub message: serde_json::Value,
}
