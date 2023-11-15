use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};

use crate::socket::interfaces::{
    websocket_error::WebSocketError, websocket_message::WebSocketMessage,
};

use super::incoming_user_message::handle_incoming_message;

use futures::stream::StreamExt;
use futures::SinkExt;
use rustis::{client::Client, commands::PubSubCommands};

use crate::{
    prisma_client::client::user, shared::arc_clients::State as app_state,
    socket::handlers::ratelimit::check_ratelimit,
};

pub async fn handle_websocket(
    ws: WebSocket,
    uuid: String,
    _: SocketAddr,
    user_id: u64,
    _: user::Data,
    state: app_state,
) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let redis_connection = Client::connect("redis://localhost:6379").await;
    match redis_connection {
        Ok(connection) => {
            let pubsub = connection.subscribe(&user_id.to_string()).await;
            match pubsub {
                Ok(mut pubsub) => loop {
                    tokio::select! {
                        next_msg = ws_receiver.next() => {
                            let ratelimit_check = check_ratelimit(user_id as i64, uuid.clone(), state.redis_client.clone()).await;
                            match ratelimit_check {
                                Ok(true) => {}
                                Ok(false) => {
                                    continue;
                                }
                                Err(_) => {
                                    let serialize_ratelimit: WebSocketError = WebSocketError {
                                        record: crate::socket::interfaces::websocket_message::Records::RateLimit,
                                        message: serde_json::json!({
                                            "message": "You are being ratelimited",
                                            "code": 429,
                                        }),
                                    };
                                    let serialized_ratelimit = serde_json::to_string(&serialize_ratelimit).unwrap();
                                    ws_sender.send(Message::Text(serialized_ratelimit)).await.unwrap();
                                    break;
                                }
                            }
                            match next_msg {
                                Some(Ok(msg)) => {
                                    let handler = handle_incoming_message(
                                        msg,
                                        &user_id,
                                        &mut pubsub,
                                        &mut ws_sender,
                                        state.prisma_client.clone(),
                                    ).await;
                                    match handler {
                                        Ok(_) => {}
                                        Err(e) => {
                                            // Send a message to the client
                                            let serialize_error: WebSocketError = WebSocketError {
                                                record: crate::socket::interfaces::websocket_message::Records::Message,
                                                message: serde_json::json!({
                                                    "message": e.to_string(),
                                                    "code": 500,
                                                }),
                                            };
                                            let serialized_error = serde_json::to_string(&serialize_error).unwrap();
                                            ws_sender.send(Message::Text(serialized_error)).await.unwrap();
                                        }
                                    }
                                }
                                Some(Err(e)) => {
                                    println!("Error from websocket: {:?}", e);
                                }
                                None => {
                                    println!("Websocket connection closed");
                                    break;
                                }
                            }
                        }
                        next_msg = pubsub.next() => {
                            match next_msg {
                                Some(Ok(msg)) => {
                                    let channel = msg.channel;
                                    let channel = String::from_utf8(channel).unwrap();
                                    let msg = msg.payload;
                                    let msg = String::from_utf8(msg).unwrap();
                                    let message_to_send: WebSocketMessage = WebSocketMessage {
                                        record: crate::socket::interfaces::websocket_message::Records::Message,
                                        queue: channel,
                                        message: serde_json::json!({
                                            "data": msg,
                                            "code": 200,
                                        }),
                                    };
                                    let serialized_message = serde_json::to_string(&message_to_send).unwrap();
                                    ws_sender.send(Message::Text(serialized_message)).await.unwrap();

                                }
                                Some(Err(e)) => {
                                    println!("Error from redis: {:?}", e);
                                }
                                None => {
                                    println!("Redis connection closed");
                                    break;
                                }
                            }
                        }
                    }
                },
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}

pub async fn websocket_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<app_state>,
    Extension(user): Extension<user::Data>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let uuid = uuid::Uuid::new_v4().to_string();
    ws.on_upgrade(move |socket| {
        handle_websocket(
            socket,
            uuid,
            client_addr,
            user.id.clone().try_into().unwrap(),
            user,
            state,
        )
    })
}
