use std::collections::HashSet;

use rustis::client::PubSubStream;

use crate::socket::interfaces::websocket_message::WebSocketMessage;

use super::incoming_user_message::MessageHandlerError;

pub async fn handle_private_pubsub_message(
    incoming_message: WebSocketMessage,
    pubsub: &mut PubSubStream,
    subbed_channels: &mut HashSet<String>,
) -> Result<(), MessageHandlerError> {
    match incoming_message.record {
        crate::socket::interfaces::websocket_message::Records::LeftQueue => {
            if subbed_channels.contains(&incoming_message.queue) {
                subbed_channels.remove(&incoming_message.queue);
                pubsub
                    .unsubscribe(&incoming_message.queue)
                    .await
                    .map_err(MessageHandlerError::RedisError)?;
                return Ok(());
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
