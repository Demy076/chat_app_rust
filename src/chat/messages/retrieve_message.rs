use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use serde::Serialize;
use validator::Validate;

use crate::{
    error::validation_error::ValidationError, prisma_client::client::messages,
    rejection::path::CustomPathDataRejection, shared::arc_clients::State as AppState,
};

use super::{
    interfaces::retrieve_message_params::RetrieveSingleMessageParam,
    retrieve_messages::MessageReponse,
};

#[derive(Serialize)]
pub struct RetrieveUserMessage {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MessageReponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn retrieve_message(
    State(state): State<AppState>,
    WithRejection(Path(params), _): WithRejection<
        Path<RetrieveSingleMessageParam>,
        CustomPathDataRejection,
    >,
) -> (StatusCode, Json<RetrieveUserMessage>) {
    match params.validate() {
        Ok(_) => {
            let message_id = params.message_id;
            let message = state
                .prisma_client
                .messages()
                .find_unique(messages::UniqueWhereParam::IdEquals(message_id))
                .with(messages::user::fetch())
                .exec()
                .await;
            let message = match message {
                Ok(message) => {
                    if message.is_none() {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(RetrieveUserMessage {
                                success: false,
                                http_code: 404,
                                message: None,
                                validation_errors: None,
                                error: Some("Message not found".to_string()),
                            }),
                        );
                    }
                    message.unwrap()
                }
                Err(_) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(RetrieveUserMessage {
                            success: false,
                            http_code: 404,
                            message: None,
                            validation_errors: None,
                            error: Some("Message not found".to_string()),
                        }),
                    );
                }
            };
            (
                StatusCode::OK,
                Json(RetrieveUserMessage {
                    success: true,
                    http_code: 200,
                    message: Some(message.into()),
                    error: None,
                    validation_errors: None,
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
                StatusCode::BAD_REQUEST,
                Json(RetrieveUserMessage {
                    success: false,
                    http_code: 400,
                    message: None,
                    validation_errors: Some(validation_errors.collect()),
                    error: None,
                }),
            );
        }
    }
}
