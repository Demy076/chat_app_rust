use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::Serialize;

use crate::{
    prisma_client::client::{banned_users_room, rooms, user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::WebSocketMessage,
};

use super::{interfaces::params_chat::RetrieveChatParams, retrieve_chat::Chat};

#[derive(Serialize)]
pub struct JoinChatResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn join_chat(
    State(state): State<AppState>,
    Extension(user): Extension<user::Data>,
    WithRejection(Path(chat_param), _): WithRejection<
        Path<RetrieveChatParams>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Json<JoinChatResponse>) {
    if chat_param.id > i32::MAX as u64 {
        return (
            StatusCode::BAD_REQUEST,
            Json(JoinChatResponse {
                success: false,
                http_code: 400,
                chat: None,
                error: Some("Chat id cannot exceed 32 bits signed".to_string()),
            }),
        );
    }
    let chat = state
        .prisma_client
        .rooms()
        .find_unique(rooms::UniqueWhereParam::IdEquals(
            chat_param.id.try_into().unwrap(),
        ))
        .with(rooms::users_rooms::fetch(vec![]))
        .exec()
        .await;
    let chat = match chat {
        Ok(chat) => {
            if chat.is_none() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(JoinChatResponse {
                        success: false,
                        http_code: 404,
                        chat: None,
                        error: Some("Chat not found".to_string()),
                    }),
                );
            }
            chat.unwrap()
        }
        Err(e) => {
            println!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JoinChatResponse {
                    success: false,
                    http_code: 500,
                    chat: None,
                    error: Some("Internal Server Error".to_string()),
                }),
            );
        }
    };
    let is_banned = state
        .prisma_client
        .banned_users_room()
        .find_first(vec![
            banned_users_room::user_id::equals(user.id),
            banned_users_room::room_id::equals(chat.id),
        ])
        .exec()
        .await;
    let is_banned = match is_banned {
        Ok(banned) => banned.is_some(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JoinChatResponse {
                    success: false,
                    http_code: 500,
                    chat: None,
                    error: Some("Internal Server Error".to_string()),
                }),
            );
        }
    };
    if is_banned {
        return (
            StatusCode::FORBIDDEN,
            Json(JoinChatResponse {
                success: false,
                http_code: 403,
                chat: None,
                error: Some("You are banned from this room.".to_string()),
            }),
        );
    }
    let participants = chat.users_rooms.unwrap();
    let is_already_participant: bool = participants
        .iter()
        .any(|participant| participant.user_id == user.id);
    if is_already_participant {
        return (
            StatusCode::CONFLICT,
            Json(JoinChatResponse {
                success: false,
                http_code: 409,
                chat: None,
                error: Some("Already participant".to_string()),
            }),
        );
    }
    if participants.len() >= chat.capacity as usize {
        return (
            StatusCode::BAD_REQUEST,
            Json(JoinChatResponse {
                success: false,
                http_code: 400,
                chat: None,
                error: Some("Chat is full".to_string()),
            }),
        );
    }
    println!("{}", chat.id);
    let participant_creation = state
        .prisma_client
        .users_rooms()
        .create(
            user::UniqueWhereParam::IdEquals(user.id),
            rooms::UniqueWhereParam::IdEquals(chat.id),
            vec![],
        )
        .exec()
        .await;
    match participant_creation {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JoinChatResponse {
                    success: false,
                    http_code: 500,
                    chat: None,
                    error: Some("Internal Server Error".to_string()),
                }),
            );
        }
    };

    let chat = state
        .prisma_client
        .rooms()
        .find_unique(rooms::UniqueWhereParam::IdEquals(
            chat_param.id.try_into().unwrap(),
        ))
        .with(rooms::users_rooms::fetch(vec![]).with(users_rooms::user::fetch()))
        .exec()
        .await;

    let chat = match chat {
        Ok(chat) => {
            if chat.is_none() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(JoinChatResponse {
                        success: false,
                        http_code: 404,
                        chat: None,
                        error: Some("Chat not found".to_string()),
                    }),
                );
            }
            chat.unwrap()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JoinChatResponse {
                    success: false,
                    http_code: 500,
                    chat: None,
                    error: Some("Internal Server Error".to_string()),
                }),
            );
        }
    };
    state
        .redis_client
        .publish(
            format!("chat:{}", chat.id),
            serde_json::to_string(&WebSocketMessage {
                record: crate::socket::interfaces::websocket_message::Records::ParticipantJoined,
                data: serde_json::json!({}),
                queue: chat.id.to_string(),
            })
            .unwrap(),
        )
        .await
        .unwrap();

    // Send message to chat it is
    (
        StatusCode::CREATED,
        Json(JoinChatResponse {
            success: true,
            http_code: 201,
            chat: Some(chat.into()),
            error: None,
        }),
    )
}
