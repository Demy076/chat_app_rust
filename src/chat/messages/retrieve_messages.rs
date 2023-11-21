use std::collections::HashSet;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use prisma_client_rust::QueryError;
use serde::Serialize;
use validator::Validate;

use crate::{
    error::validation_error::ValidationError,
    prisma_client::client::{messages, user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
};

use super::interfaces::retrieve_message_params::RetrieveMessageParams;

#[derive(Serialize)]
pub struct Sender {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct MessageReponse {
    pub message_id: i32,
    pub message: String,
    pub sender: Sender,
}

impl From<messages::Data> for MessageReponse {
    fn from(value: messages::Data) -> Self {
        let user = value.user.unwrap();
        Self {
            message_id: value.id,
            message: value.message,
            sender: Sender {
                id: user.id,
                username: user.username,
            },
        }
    }
}

#[derive(Serialize)]
pub struct MessagesResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageReponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn retrieve_messages(
    State(state): State<AppState>,
    Extension(participant): Extension<users_rooms::Data>,
    WithRejection(Path(params), _): WithRejection<
        Path<RetrieveMessageParams>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Json<MessagesResponse>) {
    match params.validate() {
        Ok(_) => {
            let (limit, message_id) = (params.limit, params.message_id);
            let messages: Result<Vec<messages::Data>, QueryError>;

            if message_id == 0 {
                // Init curso
                messages = state
                    .prisma_client
                    .messages()
                    .find_many(vec![messages::room_id::equals(participant.room_id)])
                    .order_by(messages::OrderByParam::CreatedAt(
                        prisma_client_rust::Direction::Desc,
                    ))
                    .take(limit as i64)
                    .exec()
                    .await;
            } else {
                messages = state
                    .prisma_client
                    .messages()
                    .find_many(vec![messages::room_id::equals(participant.room_id)])
                    .skip(1)
                    .cursor(messages::id::equals(message_id))
                    .take(limit as i64)
                    .order_by(messages::OrderByParam::CreatedAt(
                        prisma_client_rust::Direction::Desc,
                    ))
                    .exec()
                    .await
            }
            let messages = match messages {
                Ok(messages) => {
                    if messages.len() == 0 {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(MessagesResponse {
                                success: false,
                                http_code: 404,
                                error: Some("No messages found".to_string()),
                                latest_id: None,
                                validation_errors: None,
                                messages: None,
                            }),
                        );
                    }
                    messages
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(MessagesResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Internal server error".to_string()),
                            latest_id: None,
                            validation_errors: None,
                            messages: None,
                        }),
                    )
                }
            };

            let mut unique_user_ids = HashSet::new();
            for message in &messages {
                unique_user_ids.insert(message.user_id);
            }

            let users = state
                .prisma_client
                .user()
                .find_many(vec![user::id::in_vec(
                    unique_user_ids.into_iter().collect(),
                )])
                .exec()
                .await;
            let users = match users {
                Ok(users) => {
                    // Box every data
                    let users = users
                        .into_iter()
                        .map(|u| Box::new(u))
                        .collect::<Vec<Box<user::Data>>>();
                    users
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(MessagesResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Internal server error".to_string()),
                            latest_id: None,
                            validation_errors: None,
                            messages: None,
                        }),
                    )
                }
            };
            // Put user into messages

            let latest_id = messages.iter().last().unwrap().id;
            let messages = messages
                .into_iter()
                .filter_map(|mut m| {
                    let user = users.iter().find(|u| u.id == m.user_id);
                    m.user = user.cloned();
                    Some(m)
                })
                .map(|m| MessageReponse::from(m))
                .collect::<Vec<MessageReponse>>();
            return (
                StatusCode::OK,
                Json(MessagesResponse {
                    success: true,
                    http_code: 200,
                    messages: Some(messages),
                    latest_id: Some(latest_id),
                    validation_errors: None,
                    error: None,
                }),
            );
        }
        Err(validation_errors) => {
            let validation_errors =
                validation_errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| ValidationError {
                        field: field.to_string(),
                        // Message is a cow
                        messages: errors
                            .iter()
                            .map(|e| {
                                e.message
                                    .as_ref()
                                    .map(|m| m.to_string())
                                    .unwrap_or_else(|| "Unknown error".to_string())
                            })
                            .collect(),
                    });
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(MessagesResponse {
                    success: false,
                    http_code: 422,
                    validation_errors: Some(validation_errors.collect()),
                    latest_id: None,
                    error: None,
                    messages: None,
                }),
            );
        }
    }
}
