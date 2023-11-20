use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::Serialize;

use crate::{
    prisma_client::client::{user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
};

use super::interfaces::params_chat::RetrieveChatParams;

#[derive(Serialize)]
pub struct LeaveChatErrorResponse {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

pub async fn leave_chat(
    State(state): State<AppState>,
    Extension(user): Extension<user::Data>,
    WithRejection(Path(chat_params), _): WithRejection<
        Path<RetrieveChatParams>,
        CustomPathDataRejection,
    >,
) -> Result<StatusCode, (StatusCode, Json<LeaveChatErrorResponse>)> {
    if chat_params.id > i32::MAX as u64 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(LeaveChatErrorResponse {
                success: false,
                http_code: 400,
                error: "Chat id cannot exceed 32 bits signed".to_string(),
            }),
        ));
    }
    let is_participant = state
        .prisma_client
        .users_rooms()
        .find_first(vec![
            users_rooms::user_id::equals(user.id),
            users_rooms::room_id::equals(chat_params.id as i32),
        ])
        .with(users_rooms::room::fetch())
        .exec()
        .await;
    let is_participant = match is_participant {
        Ok(participant) => {
            if let Some(participant) = participant {
                participant
            } else {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(LeaveChatErrorResponse {
                        success: false,
                        http_code: 404,
                        error: "User is not a participant of this chat or transferring ownership"
                            .to_string(),
                    }),
                ));
            }
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LeaveChatErrorResponse {
                    success: false,
                    http_code: 500,
                    error: "Internal server error".to_string(),
                }),
            ));
        }
    };
    let is_owner = is_participant.room.unwrap().user_id == user.id;
    if is_owner {
        return Err((
            StatusCode::FORBIDDEN,
            Json(LeaveChatErrorResponse {
                success: false,
                http_code: 403,
                error: "Owner cannot leave chat consider deleting the chat".to_string(),
            }),
        ));
    }
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
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LeaveChatErrorResponse {
                    success: false,
                    http_code: 500,
                    error: "Internal server error".to_string(),
                }),
            ));
        }
    }
    state
        .redis_client
        .publish(
            format!("priv_user:{}", user.id),
            serde_json::to_string(
                &crate::socket::interfaces::websocket_message::WebSocketMessage {
                    record: crate::socket::interfaces::websocket_message::Records::LeftQueue,
                    queue: format!("chat:{}", chat_params.id),
                    data: serde_json::json!({}),
                },
            )
            .ok(),
        )
        .await
        .ok();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        state
            .redis_client
            .publish(
                format!("chat:{}", chat_params.id),
                serde_json::to_string(
                    &crate::socket::interfaces::websocket_message::WebSocketMessage {
                        record:
                            crate::socket::interfaces::websocket_message::Records::ParticipantLeft,
                        queue: format!("chat:{}", chat_params.id),
                        data: serde_json::json!({
                            "user_id": user.id,
                        }),
                    },
                )
                .ok(),
            )
            .await
            .ok();
    });
    Ok(StatusCode::NO_CONTENT)
}
