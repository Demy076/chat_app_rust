use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::Serialize;

use crate::{
    prisma_client::client::{messages, user},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::WebSocketMessage,
};

use super::interfaces::retrieve_message_params::RetrieveSingleMessageParam;

#[derive(Serialize)]
pub struct DeleteMessageErrorResponse {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

#[derive(Serialize)]
pub struct WebsocketDeleteMessageData {
    pub message_id: i32,
    pub action: String,
}

pub async fn delete_message(
    State(state): State<AppState>,
    Extension(user): Extension<user::Data>,
    WithRejection(Path(params), _): WithRejection<
        Path<RetrieveSingleMessageParam>,
        CustomPathDataRejection,
    >,
) -> Result<StatusCode, (StatusCode, Json<DeleteMessageErrorResponse>)> {
    let message_id = params.message_id;
    let message = state
        .prisma_client
        .messages()
        .find_unique(messages::UniqueWhereParam::IdEquals(message_id))
        .with(messages::room::fetch())
        .exec()
        .await;
    match message {
        Ok(message) => {
            if message.is_none() {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(DeleteMessageErrorResponse {
                        success: false,
                        http_code: 404,
                        error: "Message not found".to_string(),
                    }),
                ));
            }
            let message = message.unwrap();
            if message.room.is_none() {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(DeleteMessageErrorResponse {
                        success: false,
                        http_code: 500,
                        error: "Internal server error".to_string(),
                    }),
                ));
            }
            let room = message.clone().room.unwrap();
            if message.user_id != user.id {
                if room.user_id != user.id {
                    return Err((
                        StatusCode::FORBIDDEN,
                        Json(DeleteMessageErrorResponse {
                            success: false,
                            http_code: 403,
                            error: "You are not allowed to delete this message".to_string(),
                        }),
                    ));
                }
            }
            let delete_message = state
                .prisma_client
                .messages()
                .delete(messages::UniqueWhereParam::IdEquals(message_id))
                .exec()
                .await;
            match delete_message {
                Ok(_) => {
                    let prefix = format!("chat:{}", room.id);
                    state
                        .redis_client
                        .publish(
                            prefix.clone(),
                            serde_json::to_string(&WebSocketMessage {
                                record:
                                    crate::socket::interfaces::websocket_message::Records::Message,
                                queue: prefix,
                                data: serde_json::json!(WebsocketDeleteMessageData {
                                    message_id: message_id,
                                    action: "delete".to_string(),
                                }),
                            })
                            .unwrap(),
                        )
                        .await
                        .ok();
                    return Ok(StatusCode::NO_CONTENT);
                }
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(DeleteMessageErrorResponse {
                            success: false,
                            http_code: 500,
                            error: "Internal server error".to_string(),
                        }),
                    ));
                }
            }
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteMessageErrorResponse {
                    success: false,
                    http_code: 500,
                    error: "Internal server error".to_string(),
                }),
            ));
        }
    }
}
