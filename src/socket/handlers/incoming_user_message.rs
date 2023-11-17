use std::{collections::HashSet, fmt::Display, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use futures_util::stream::SplitSink;
use rustis::client::PubSubStream;

use crate::{
    prisma_client::client::{rooms, user, PrismaClient},
    socket::interfaces::{
        websocket_incoming_message::IncomingWebsocketMessage, websocket_message::WebSocketMessage,
    },
};

pub enum MessageHandlerError {
    RedisError(rustis::Error),
    JsonError(serde_json::Error),
    AxumError(axum::Error),
    InvalidMessage(String),
    ValidationError(String),
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
            MessageHandlerError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

pub async fn handle_incoming_message(
    message: Message,
    user_id: &u64,
    pubsub: &mut PubSubStream,
    sender: &mut SplitSink<WebSocket, Message>,
    prisma_client: &PrismaClient,
    subbed_channels: &mut HashSet<String>,
) -> Result<(), MessageHandlerError> {
    let text = message.to_text().map_err(MessageHandlerError::AxumError)?;
    let message: IncomingWebsocketMessage =
        serde_json::from_str(text).map_err(MessageHandlerError::JsonError)?;
    match message.record {
        crate::socket::interfaces::websocket_message::Records::JoinedQueue => match message.mount {
            crate::socket::interfaces::websocket_incoming_message::Mounts::Chat => {
                if subbed_channels.contains(&message.queue) {
                    return Ok(());
                }
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
                        return Err(MessageHandlerError::ValidationError(
                            "Invalid queue id".to_string(),
                        ))
                    }
                };
                let is_participant = chat
                    .users_rooms()
                    .unwrap()
                    .iter()
                    .find(|user_room| user_room.user_id == *user_id as i32);
                if is_participant.is_some() {
                    pubsub
                        .subscribe(format!("chat:{}", chat.id))
                        .await
                        .map_err(MessageHandlerError::RedisError)?;
                    subbed_channels.insert(format!("chat:{}", chat.id));
                    // Send success message
                    let joined_queue = WebSocketMessage {
                        record: crate::socket::interfaces::websocket_message::Records::JoinedQueue,
                        queue: message.queue,
                        data: serde_json::json!({
                            "data": "You joined the queue",
                            "code": 200,
                        }),
                    };
                    let serialized_message = serde_json::to_string(&joined_queue)
                        .map_err(MessageHandlerError::JsonError)?;
                    sender
                        .send(Message::Text(serialized_message))
                        .await
                        .map_err(MessageHandlerError::AxumError)?;
                }

                Ok(())
            }
            crate::socket::interfaces::websocket_incoming_message::Mounts::User => {
                if subbed_channels.contains(&message.queue) {
                    return Ok(());
                }
                let user = prisma_client
                    .user()
                    .find_unique(user::UniqueWhereParam::IdEquals(*user_id as i32))
                    .exec()
                    .await
                    .map_err(MessageHandlerError::ServerError)?;
                let user = match user {
                    Some(user) => user,
                    None => {
                        return Err(MessageHandlerError::ValidationError(
                            "Invalid user id".to_string(),
                        ))
                    }
                };
                pubsub
                    .subscribe(&user.id.to_string())
                    .await
                    .map_err(MessageHandlerError::RedisError)?;
                let joined_queue = serde_json::to_string(&WebSocketMessage {
                    record: crate::socket::interfaces::websocket_message::Records::JoinedQueue,
                    queue: user.id.to_string(),
                    data: serde_json::json!({
                        "data": "You joined the queue",
                        "code": 200,
                    }),
                })
                .map_err(MessageHandlerError::JsonError)?;
                sender
                    .send(Message::Text(joined_queue))
                    .await
                    .map_err(MessageHandlerError::AxumError)?;

                Ok(())
            }
        },
        crate::socket::interfaces::websocket_message::Records::LeftQueue => match message.mount {
            crate::socket::interfaces::websocket_incoming_message::Mounts::Chat => {
                if !subbed_channels.contains(&message.queue) {
                    return Ok(());
                }
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
                        return Err(MessageHandlerError::ValidationError(
                            "Invalid queue id".to_string(),
                        ))
                    }
                };
                let is_participant = chat
                    .users_rooms()
                    .unwrap()
                    .iter()
                    .find(|user_room| user_room.user_id == *user_id as i32);
                if is_participant.is_some() {
                    pubsub.unsubscribe(&message.queue).await.ok();
                    let left_queue = WebSocketMessage {
                        record: crate::socket::interfaces::websocket_message::Records::LeftQueue,
                        queue: message.queue,
                        data: serde_json::json!({
                            "data": "You left the queue",
                            "code": 200,
                        }),
                    };
                    let serialized_message = serde_json::to_string(&left_queue)
                        .map_err(MessageHandlerError::JsonError)?;
                    sender
                        .send(Message::Text(serialized_message))
                        .await
                        .map_err(MessageHandlerError::AxumError)?;
                }
                Ok(())
            }
            crate::socket::interfaces::websocket_incoming_message::Mounts::User => {
                if !subbed_channels.contains(&message.queue) {
                    return Ok(());
                }
                let user = prisma_client
                    .user()
                    .find_unique(user::UniqueWhereParam::IdEquals(*user_id as i32))
                    .exec()
                    .await
                    .map_err(MessageHandlerError::ServerError)?;
                let user = match user {
                    Some(user) => user,
                    None => {
                        return Err(MessageHandlerError::ValidationError(
                            "Invalid user id".to_string(),
                        ))
                    }
                };
                pubsub.unsubscribe(&user.id.to_string()).await.ok();
                let left_queue = serde_json::to_string(&WebSocketMessage {
                    record: crate::socket::interfaces::websocket_message::Records::LeftQueue,
                    queue: user.id.to_string(),
                    data: serde_json::json!({
                        "data": "You left the queue",
                        "code": 200,
                    }),
                })
                .map_err(MessageHandlerError::JsonError)?;
                sender
                    .send(Message::Text(left_queue))
                    .await
                    .map_err(MessageHandlerError::AxumError)?;
                Ok(())
            }
        },

        crate::socket::interfaces::websocket_message::Records::Message => Ok(()),
        crate::socket::interfaces::websocket_message::Records::RateLimit => Ok(()),
        crate::socket::interfaces::websocket_message::Records::ParticipantJoined => Ok(()),
        crate::socket::interfaces::websocket_message::Records::ParticipantLeft => Ok(()),
    }
}
