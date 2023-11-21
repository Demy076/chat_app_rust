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
    prisma_client::client::{messages, users_rooms},
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
    pub message: String,
    pub sender: Sender,
}

impl From<messages::Data> for MessageReponse {
    fn from(value: messages::Data) -> Self {
        let user = value.user.unwrap();
        Self {
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
            // If message id is 0, we want to retrieve the last messages
            let messages_count = state
                .prisma_client
                .messages()
                .count(vec![messages::room_id::equals(participant.room_id)])
                .exec()
                .await;
            // Retrieve last message in database
            let messages: Result<Vec<messages::Data>, QueryError>;
            if params.message_id == 0 {
                messages = state
                    .prisma_client
                    .messages()
                    .find_many(vec![messages::room_id::equals(participant.room_id)])
                    .order_by(messages::OrderByParam::Id(
                        prisma_client_rust::Direction::Asc,
                    ))
                    .with(messages::user::fetch())
                    .exec()
                    .await;
            } else {
                messages = state
                    .prisma_client
                    .messages()
                    .find_many(vec![
                        messages::room_id::equals(participant.room_id),
                        messages::id::lt(params.message_id),
                    ])
                    .take(params.limit as i64)
                    .with(messages::user::fetch())
                    .exec()
                    .await;
            }
            let messages = match messages {
                Ok(messages) => {
                    if messages.len() == 0 {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(MessagesResponse {
                                success: false,
                                http_code: 404,
                                validation_errors: None,
                                error: Some("No messages found".to_string()),
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
                            validation_errors: None,
                            error: Some("Internal server error".to_string()),
                            messages: None,
                        }),
                    )
                }
            };
            let messages = messages
                .into_iter()
                .map(|message| MessageReponse::from(message))
                .collect();
            (
                StatusCode::OK,
                Json(MessagesResponse {
                    success: true,
                    http_code: 200,
                    validation_errors: None,
                    error: None,
                    messages: Some(messages),
                }),
            )
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
                    error: None,
                    messages: None,
                }),
            );
        }
    }
}
