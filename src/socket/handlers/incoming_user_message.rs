use std::{fmt::Display, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use futures_util::stream::SplitSink;
use prisma_client_rust::operator::and;
use rustis::client::PubSubStream;

use crate::{
    prisma_client::client::{banned_users_room, rooms, user, PrismaClient},
    socket::interfaces::websocket_incoming_message::IncomingWebsocketMessage,
};

pub enum MessageHandlerError {
    RedisError(rustis::Error),
    JsonError(serde_json::Error),
    AxumError(axum::Error),
    InvalidMessage(String),
    ServerError(prisma_client_rust::QueryError),
}

impl Display for MessageHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageHandlerError::RedisError(e) => write!(f, "Redis error: {}", e),
            MessageHandlerError::JsonError(e) => write!(f, "Json error: {}", e),
            MessageHandlerError::AxumError(e) => write!(f, "Axum error: {}", e),
            MessageHandlerError::ServerError(e) => write!(f, "Server error: {}", e),
            MessageHandlerError::InvalidMessage(e) => write!(f, "Invalid message: {}", e),
        }
    }
}

pub async fn handle_incoming_message(
    message: Message,
    user_id: &u64,
    pubsub: &mut PubSubStream,
    sender: &mut SplitSink<WebSocket, Message>,
    prisma_client: Arc<PrismaClient>,
) -> Result<(), MessageHandlerError> {
    let text = message.to_text().map_err(MessageHandlerError::AxumError)?;
    let message: IncomingWebsocketMessage =
        serde_json::from_str(text).map_err(MessageHandlerError::JsonError)?;
    match message.record {
        crate::socket::interfaces::websocket_message::Records::JoinedQueue => match message.mount {
            crate::socket::interfaces::websocket_incoming_message::Mounts::Chat => {
                let queue = message.queue.parse::<i32>();
                let queue = match queue {
                    Ok(queue) => queue,
                    Err(_) => {
                        return Err(MessageHandlerError::InvalidMessage(
                            "Invalid queue id".to_string(),
                        ))
                    }
                };
                let chat = prisma_client
                    .rooms()
                    .find_unique(rooms::UniqueWhereParam::IdEquals(queue))
                    .with(rooms::users_rooms::fetch(vec![]))
                    .exec()
                    .await
                    .map_err(MessageHandlerError::ServerError)?;
                let chat = match chat {
                    Some(chat) => chat,
                    None => {
                        return Err(MessageHandlerError::InvalidMessage(
                            "Invalid queue id".to_string(),
                        ))
                    }
                };
                let count_chat = chat.users_rooms.unwrap().len();
                let count_chat = count_chat as i32;
                if count_chat > chat.capacity {
                    return Err(MessageHandlerError::InvalidMessage(
                        "Queue is full".to_string(),
                    ));
                }
                let is_banned = prisma_client
                    .banned_users_room()
                    .find_first(vec![and(vec![
                        banned_users_room::user_id::equals(*user_id as i32),
                        banned_users_room::room_id::equals(queue as i32),
                    ])])
                    .exec()
                    .await
                    .map_err(MessageHandlerError::ServerError)?;
                if is_banned.is_some() {
                    return Err(MessageHandlerError::InvalidMessage(
                        "You are banned from this queue".to_string(),
                    ));
                }
                // Insert participant of queue
                prisma_client
                    .users_rooms()
                    .create(
                        user::UniqueWhereParam::IdEquals(*user_id as i32),
                        rooms::UniqueWhereParam::IdEquals(queue as i32),
                        vec![],
                    )
                    .exec()
                    .await
                    .map_err(MessageHandlerError::ServerError)?;
                pubsub
                    .subscribe(format!("chat:{}", queue))
                    .await
                    .map_err(MessageHandlerError::RedisError)?;
                let serialize_joined_queue =
                    crate::socket::interfaces::websocket_message::WebSocketMessage {
                        record: crate::socket::interfaces::websocket_message::Records::JoinedQueue,
                        queue: message.queue,
                        message: serde_json::json!({
                            "message": "Joined queue",
                            "code": 200,
                        }),
                    };
                let serialized_joined_queue = serde_json::to_string(&serialize_joined_queue)
                    .map_err(MessageHandlerError::JsonError)?;
                sender
                    .send(Message::Text(serialized_joined_queue))
                    .await
                    .map_err(MessageHandlerError::AxumError)?;
                Ok(())
            }
            crate::socket::interfaces::websocket_incoming_message::Mounts::User => Ok(()),
        },
        crate::socket::interfaces::websocket_message::Records::LeftQueue => Ok(()),
        crate::socket::interfaces::websocket_message::Records::Message => Ok(()),
        crate::socket::interfaces::websocket_message::Records::RateLimit => Ok(()),
    }
}
