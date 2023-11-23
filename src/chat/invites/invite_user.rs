use crate::{
    chat::interfaces::single_user_param::SingleUserParam,
    prisma_client::client::{banned_users_room, rooms, user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::WebSocketMessage,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::Serialize;

#[derive(Serialize)]
pub struct InviteUserResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn invite_user(
    State(state): State<AppState>,
    Extension(participant): Extension<users_rooms::Data>,
    WithRejection(Path(SingleUserParam { user_id }), _): WithRejection<
        axum::extract::Path<SingleUserParam>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Json<InviteUserResponse>) {
    if participant.user_id as u32 == user_id {
        return (
            StatusCode::BAD_REQUEST,
            Json(InviteUserResponse {
                success: false,
                error: Some("You can't invite yourself".to_string()),
                http_code: 400,
            }),
        );
    }
    let is_participant = state
        .prisma_client
        .users_rooms()
        .find_unique(users_rooms::UniqueWhereParam::IdEquals(user_id as i32))
        .exec()
        .await;
    let is_participant = match is_participant {
        Ok(is_participant) => is_participant.is_some(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InviteUserResponse {
                    success: false,
                    http_code: 500,
                    error: Some("Internal server error".to_string()),
                }),
            )
        }
    };
    if is_participant {
        return (
            StatusCode::BAD_REQUEST,
            Json(InviteUserResponse {
                success: false,
                http_code: 400,
                error: Some("User is already a participant".to_string()),
            }),
        );
    }
    let is_banned = state
        .prisma_client
        .banned_users_room()
        .find_first(vec![
            banned_users_room::user_id::equals(user_id as i32),
            banned_users_room::room_id::equals(participant.room_id),
        ])
        .exec()
        .await;
    let is_banned = match is_banned {
        Ok(is_banned) => is_banned.is_some(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InviteUserResponse {
                    success: false,
                    http_code: 500,
                    error: Some("Internal server error".to_string()),
                }),
            )
        }
    };
    if is_banned {
        return (
            StatusCode::BAD_REQUEST,
            Json(InviteUserResponse {
                success: false,
                http_code: 400,
                error: Some("User is banned from this room".to_string()),
            }),
        );
    }
    let invite = state
        .prisma_client
        .invites()
        .create(
            user::UniqueWhereParam::IdEquals(user_id as i32),
            rooms::UniqueWhereParam::IdEquals(participant.room_id),
            vec![],
        )
        .exec()
        .await;
    match invite {
        Ok(invite) => {
            state
                .redis_client
                .publish(
                    format!("priv_user:{}", user_id),
                    serde_json::to_string(&WebSocketMessage {
                        record: crate::socket::interfaces::websocket_message::Records::Message,
                        queue: format!("chat-{}", participant.room_id),
                        data: serde_json::json!({
                            "invite_id": invite.id,
                            "type": "invite",
                        }),
                    })
                    .unwrap(),
                )
                .await
                .ok();
            (
                StatusCode::CREATED,
                Json(InviteUserResponse {
                    success: true,
                    http_code: 201,
                    error: None,
                }),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InviteUserResponse {
                    success: false,
                    http_code: 500,
                    error: Some("Internal server error".to_string()),
                }),
            )
        }
    }
}
