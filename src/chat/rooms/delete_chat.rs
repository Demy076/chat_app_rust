use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::Serialize;

use crate::{
    prisma_client::client::rooms,
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::{Records, WebSocketMessage},
};

use super::interfaces::params_chat::RetrieveChatParams;

#[derive(Serialize)]
pub struct DeleteChatError {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

pub async fn delete_chat(
    State(state): State<AppState>,
    WithRejection(Path(chat_params), _): WithRejection<
        Path<RetrieveChatParams>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Option<Json<DeleteChatError>>) {
    let chat_id = chat_params.id;
    let removal_chat = state
        .prisma_client
        .rooms()
        .delete(rooms::UniqueWhereParam::IdEquals(
            chat_id.try_into().unwrap(),
        ))
        .exec()
        .await;
    match removal_chat {
        Ok(_) => {}
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(Json(DeleteChatError {
                    success: false,
                    http_code: 500,
                    error: e.to_string(),
                })),
            );
        }
    };
    let websocket_message = serde_json::to_string(&WebSocketMessage {
        record: Records::Message,
        queue: format!("chat:{}", chat_id),
        data: serde_json::json!(
            {
                "chat_id": chat_id,
                "action": "delete"

            }
        ),
    })
    .unwrap();
    state
        .redis_client
        .publish(format!("chat:{}", chat_id), websocket_message)
        .await
        .ok();
    (StatusCode::NO_CONTENT, None)
}
