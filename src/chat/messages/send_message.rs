use axum::{
    extract::{Json as ExtractJson, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::{Deserialize, Serialize};
use validator::Validate;

use rustrict::CensorStr;

use crate::{
    error::validation_error::ValidationError,
    prisma_client::client::{rooms, user, users_rooms},
    rejection::json::CustomJsonDataRejection,
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::WebSocketMessage,
};

#[derive(Deserialize, Validate)]
pub struct SendMessageBody {
    #[validate(
        required(message = "message is required"),
        length(
            min = 1,
            max = 1000,
            message = "message must be between 1 and 1000 characters"
        )
    )]
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn send_message(
    State(state): State<AppState>,
    Extension(participant): Extension<users_rooms::Data>,
    WithRejection(ExtractJson(body), _): WithRejection<
        ExtractJson<SendMessageBody>,
        CustomJsonDataRejection,
    >,
) -> (StatusCode, Json<SendMessageResponse>) {
    match body.validate() {
        Ok(_) => {
            let message = body.message.unwrap();
            if message.is_inappropriate() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(SendMessageResponse {
                        success: false,
                        http_code: 400,
                        error: Some("Message is inappropriate".to_string()),
                        message: None,
                        validation_errors: None,
                    }),
                );
            }
            let message = state
                .prisma_client
                .messages()
                .create(
                    message.to_string(),
                    user::UniqueWhereParam::IdEquals(participant.user_id.clone()),
                    rooms::UniqueWhereParam::IdEquals(participant.room_id.clone()),
                    vec![],
                )
                .exec()
                .await;
            let message = match message {
                Ok(message) => message,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(SendMessageResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Failed to create message".to_string()),
                            message: None,
                            validation_errors: None,
                        }),
                    );
                }
            };
            state
                .redis_client
                .publish(
                    format!("chat:{}", participant.room_id),
                    serde_json::to_string(&WebSocketMessage {
                        record: crate::socket::interfaces::websocket_message::Records::Message,
                        queue: format!("chat:{}", participant.room_id),
                        data: serde_json::json!({
                            "message_id": message.id,
                        }),
                    })
                    .unwrap(),
                )
                .await
                .ok();
            (
                StatusCode::CREATED,
                Json(SendMessageResponse {
                    success: true,
                    http_code: 201,
                    error: None,
                    message: Some(message.message),
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
                Json(SendMessageResponse {
                    success: false,
                    http_code: 400,
                    error: None,
                    message: None,
                    validation_errors: Some(validation_errors.collect()),
                }),
            );
        }
    }
}
