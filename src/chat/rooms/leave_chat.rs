use axum::{
    extract::{Path, State},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use http::StatusCode;
use rustis::commands::PubSubCommands;
use serde::Serialize;

use crate::{
    prisma_client::client::{user, users_rooms},
    rejection::path::CustomPathDataRejection,
};

use super::interfaces::params_chat::RetrieveChatParams;

#[derive(Serialize)]
pub struct LeaveChatErrorResponse {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

pub async fn leave_chat(
    State(state): State<crate::shared::arc_clients::State>,
    Extension(user): Extension<user::Data>,
    WithRejection(Path(chat_params), _): WithRejection<
        Path<RetrieveChatParams>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Option<Json<LeaveChatErrorResponse>>) {
    if chat_params.id > i32::MAX as u64 {
        return (
            StatusCode::BAD_REQUEST,
            Some(Json(LeaveChatErrorResponse {
                success: false,
                http_code: 400,
                error: "Chat id cannot exceed 32 bits signed".to_string(),
            })),
        );
    }
    let is_participant = state
        .prisma_client
        .users_rooms()
        .find_first(vec![
            users_rooms::user_id::equals(user.id),
            users_rooms::room_id::equals(chat_params.id as i32),
        ])
        .exec()
        .await;
    let is_participant = match is_participant {
        Ok(participant) => {
            if let Some(participant) = participant {
                participant
            } else {
                return (
                    StatusCode::NOT_FOUND,
                    Some(Json(LeaveChatErrorResponse {
                        success: false,
                        http_code: 404,
                        error: "Not a participant of this chat".to_string(),
                    })),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(Json(LeaveChatErrorResponse {
                    success: false,
                    http_code: 500,
                    error: "Internal server error".to_string(),
                })),
            )
        }
    };
    // Remove participant from chat
    let remove_participant = state
        .prisma_client
        .users_rooms()
        .delete(users_rooms::UniqueWhereParam::IdEquals(is_participant.id))
        .exec()
        .await;
    match remove_participant {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(Json(LeaveChatErrorResponse {
                    success: false,
                    http_code: 500,
                    error: "Internal server error".to_string(),
                })),
            )
        }
    }
    state.redis_client.publish(
        format!("chat:{}", chat_params.id),
        serde_json::to_string(
            &crate::socket::interfaces::websocket_message::WebSocketMessage {
                record: crate::socket::interfaces::websocket_message::Records::ParticipantLeft,
                queue: chat_params.id.to_string(),
                data: serde_json::json!({
                    "user_id": user.id,
                }),
            },
        )
        .unwrap(),
    );
    state.redis_client.publish(
        format!("priv_user:{}", user.id),
        serde_json::to_string(
            &crate::socket::interfaces::websocket_message::WebSocketMessage {
                record: crate::socket::interfaces::websocket_message::Records::ParticipantLeft,
                queue: chat_params.id.to_string(),
                data: serde_json::json!({
                    "user_id": user.id,
                }),
            },
        )
        .unwrap(),
    );
    (StatusCode::NO_CONTENT, None)
}
