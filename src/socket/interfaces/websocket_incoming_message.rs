use serde::{
    de::{Deserialize, Deserializer},
    Deserialize as DeserializeM,
};

use super::websocket_message::Records;

pub enum Mounts {
    Chat,
    User,
}

impl<'de> Deserialize<'de> for Mounts {
    fn deserialize<D>(deserializer: D) -> Result<Mounts, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "chat" => Ok(Mounts::Chat),
            "user" => Ok(Mounts::User),
            _ => Err(serde::de::Error::custom("expected a valid mount")),
        }
    }
}

#[derive(DeserializeM)]
pub struct IncomingWebsocketMessage {
    pub record: Records,
    pub mount: Mounts,
    pub queue: String,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
