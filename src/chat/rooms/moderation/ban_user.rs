use crate::{
    error::validation_error::ValidationError,
    prisma_client::client::{banned_users_room, user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::WebSocketMessage,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize)]
pub struct BanUserResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct BanUserParams {
    #[validate(range(min = 1, message = "user_id must be greater than 0"))]
    pub user_id: i32,
}

pub async fn ban_user(
    State(state): State<AppState>,
    Extension(user): Extension<user::Data>,
    WithRejection(Path(params), _): WithRejection<Path<BanUserParams>, CustomPathDataRejection>,
) -> (StatusCode, Json<BanUserResponse>) {
    match params.validate() {
        Ok(_) => {
            if params.user_id == user.id {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(BanUserResponse {
                        success: false,
                        http_code: 400,
                        message: None,
                        error: Some("You cannot ban yourself (owner)".to_string()),
                        validation_errors: None,
                    }),
                );
            }
            let user = state
                .prisma_client
                .users_rooms()
                .find_first(vec![users_rooms::user_id::equals(params.user_id)])
                .with(users_rooms::user::fetch())
                .exec()
                .await;
            let user = match user {
                Ok(participant) => {
                    if participant.is_none() {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(BanUserResponse {
                                success: false,
                                http_code: 404,
                                message: None,
                                error: Some("User not found".to_string()),
                                validation_errors: None,
                            }),
                        );
                    }
                    participant.unwrap()
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(BanUserResponse {
                            success: false,
                            http_code: 500,
                            message: None,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    )
                }
            };
            let is_already_banned = state
                .prisma_client
                .banned_users_room()
                .find_first(vec![
                    banned_users_room::user_id::equals(user.id),
                    banned_users_room::room_id::equals(user.room_id),
                ])
                .exec()
                .await;
            let is_already_banned = match is_already_banned {
                Ok(banned) => banned.is_some(),
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(BanUserResponse {
                            success: false,
                            http_code: 500,
                            message: None,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    );
                }
            };
            if is_already_banned {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(BanUserResponse {
                        success: false,
                        http_code: 400,
                        message: None,
                        error: Some("User is already banned".to_string()),
                        validation_errors: None,
                    }),
                );
            }
            // Delete user from room
            let delete_user = state
                .prisma_client
                .users_rooms()
                .delete(users_rooms::UniqueWhereParam::IdEquals(user.id))
                .exec()
                .await;
            match delete_user {
                Ok(_) => {
                    state
                    .redis_client
                    .publish(
                        format!("priv_user:{}", user.room_id),
                        serde_json::to_string(&WebSocketMessage {
                            record: crate::socket::interfaces::websocket_message::Records::LeftQueue,
                            queue: format!("priv_user:{}", user.room_id),
                            data: serde_json::json!({}),
                        }).unwrap(),
                    )
                    .await
                    .ok();
                    tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        state
                        .redis_client
                        .publish(
                            format!("chat:{}", user.room_id),
                            serde_json::to_string(&WebSocketMessage {
                                record: crate::socket::interfaces::websocket_message::Records::ParticipantLeft,
                                queue: format!("chat:{}", user.room_id),
                                data: serde_json::json!({
                                    "user_id": user.user_id,
                                }),
                            }).unwrap(),
                        )
                        .await
                        .ok();
                    });
                    (
                        StatusCode::CREATED,
                        Json(BanUserResponse {
                            success: true,
                            http_code: 201,
                            message: Some("User banned".to_string()),
                            error: None,
                            validation_errors: None,
                        }),
                    )
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(BanUserResponse {
                            success: false,
                            http_code: 500,
                            message: None,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    );
                }
            }
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
                Json(BanUserResponse {
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
