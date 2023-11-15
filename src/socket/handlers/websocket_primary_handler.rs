use std::net::SocketAddr;

use axum::extract::ws::WebSocket;

use futures::stream::StreamExt;
use futures_util::SinkExt;
use rustis::{client::Client, commands::PubSubCommands};

use crate::prisma_client::client::user;

pub async fn handle_websocket(ws: WebSocket, _: SocketAddr, user_id: u64, _: user::Data) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let redis_connection = Client::connect("redis://localhost:6379").await;
    match redis_connection {
        Ok(connection) => {
            let pubsub = connection.subscribe(&user_id.to_string()).await;
            match pubsub {
                Ok(mut pubsub) => {}
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}
