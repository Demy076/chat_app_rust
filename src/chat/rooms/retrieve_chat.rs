use crate::{
    prisma_client::client::{
        rooms::{self, Data as Room},
        user::Data as User,
        users_rooms,
    },
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use serde::Serialize;

use super::interfaces::params_chat::RetrieveChatParams;

#[derive(Serialize)]
pub struct ParticipantChat {
    pub id: u64,
    pub username: String,
    pub email: String,
}

impl From<User> for ParticipantChat {
    fn from(value: User) -> Self {
        ParticipantChat {
            id: value.id.try_into().unwrap(),
            username: value.username,
            email: value.email,
        }
    }
}

impl From<Room> for Chat {
    fn from(value: Room) -> Self {
        let participants: Vec<ParticipantChat> = value
            .users_rooms()
            .unwrap()
            .into_iter()
            .map(|participant| participant.user().unwrap().to_owned().into())
            .collect();
        return Chat {
            name: value.name,
            capacity: value.capacity.try_into().unwrap(),
            users: participants,
        };
    }
}

#[derive(Serialize)]
pub struct Chat {
    pub name: String,
    pub capacity: u8,
    pub users: Vec<ParticipantChat>,
}

#[derive(Serialize)]
pub struct RetrieveChatResonse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn retrieve_chat(
    State(state): State<AppState>,
    WithRejection(Path(chat_param), _): WithRejection<
        Path<RetrieveChatParams>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Json<RetrieveChatResonse>) {
    let chat_id = chat_param.id;
    let chat = state
        .prisma_client
        .rooms()
        .find_unique(rooms::UniqueWhereParam::IdEquals(
            chat_id.try_into().unwrap(),
        ))
        .with(rooms::users_rooms::fetch(vec![]).with(users_rooms::user::fetch()))
        .exec()
        .await;
    let chat = match chat {
        Ok(chat) => {
            if chat.is_none() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(RetrieveChatResonse {
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
                Json(RetrieveChatResonse {
                    success: false,
                    http_code: 500,
                    chat: None,
                    error: Some("Internal server error".to_string()),
                }),
            );
        }
    };
    (
        StatusCode::OK,
        Json(RetrieveChatResonse {
            success: true,
            http_code: 200,
            chat: Some(chat.into()),
            error: None,
        }),
    )
}
