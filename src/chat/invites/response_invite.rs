use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use rustis::commands::PubSubCommands;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    error::validation_error::ValidationError as CustomValidationError,
    prisma_client::client::{invites, rooms, user, users_rooms},
    rejection::{json::CustomJsonDataRejection, path::CustomPathDataRejection},
    shared::arc_clients::State as AppState,
    socket::interfaces::websocket_message::WebSocketMessage,
};

use super::interfaces::invite_id_param::InviteIdParam;

#[derive(Serialize)]
pub struct InviteUserResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<CustomValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub enum InviteUserReaction {
    Accept,
    Decline,
}

fn validate_reaction(reaction: &InviteUserReaction) -> Result<(), ValidationError> {
    match reaction {
        InviteUserReaction::Accept | InviteUserReaction::Decline => Ok(()),
    }
}

#[derive(Deserialize, Validate)]
pub struct InviteUserRequest {
    // It can only be accept or decline
    #[validate(custom = "validate_reaction", required(message = "Invalid reaction"))]
    pub reaction: Option<InviteUserReaction>,
}

pub async fn invite_response(
    State(state): State<AppState>,
    Extension(participant): Extension<users_rooms::Data>,
    WithRejection(Path(InviteIdParam { invite_id }), _): WithRejection<
        Path<InviteIdParam>,
        CustomPathDataRejection,
    >,
    WithRejection(ExtractJson(body), _): WithRejection<
        ExtractJson<InviteUserRequest>,
        CustomJsonDataRejection,
    >,
) -> (StatusCode, Json<InviteUserResponse>) {
    match body.validate() {
        Ok(_) => {
            let reaction = body.reaction.unwrap();
            let invite = state
                .prisma_client
                .invites()
                .find_first(vec![
                    invites::id::equals(invite_id),
                    invites::room_id::equals(participant.room_id),
                    // invite cannot be older than 1 minute
                    invites::created_at::gt(
                        (chrono::Utc::now() - chrono::Duration::minutes(1)).into(),
                    ),
                    invites::user_id::equals(participant.user_id as i32),
                ])
                .exec()
                .await;
            let invite = match invite {
                Ok(invite) => {
                    if invite.is_none() {
                        return (
                            StatusCode::NOT_FOUND,
                            Json(InviteUserResponse {
                                success: false,
                                http_code: 404,
                                error: Some("Invite not found".to_string()),
                                validation_errors: None,
                            }),
                        );
                    }
                    invite.unwrap()
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(InviteUserResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    )
                }
            };
            let room = state
                .prisma_client
                .rooms()
                .find_unique(rooms::UniqueWhereParam::IdEquals(participant.room_id))
                .exec()
                .await;
            let is_room_full = state
                .prisma_client
                .users_rooms()
                .count(vec![users_rooms::room_id::equals(participant.room_id)])
                .exec()
                .await;
            let is_room_full = match is_room_full {
                Ok(is_room_full) => is_room_full >= room.unwrap().unwrap().capacity as i64,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(InviteUserResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    )
                }
            };
            if is_room_full {
                let invite = state
                    .prisma_client
                    .invites()
                    .update(
                        invites::UniqueWhereParam::IdEquals(invite.id),
                        vec![invites::state::set(
                            crate::prisma_client::client::InviteState::Declined,
                        )],
                    )
                    .exec()
                    .await;
                match invite {
                    Ok(_) => {}
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(InviteUserResponse {
                                success: false,
                                http_code: 500,
                                error: Some("Internal server error".to_string()),
                                validation_errors: None,
                            }),
                        )
                    }
                };
                return (
                    StatusCode::BAD_REQUEST,
                    Json(InviteUserResponse {
                        success: false,
                        http_code: 400,
                        error: Some("Room is full".to_string()),
                        validation_errors: None,
                    }),
                );
            }

            match reaction {
                InviteUserReaction::Accept => {
                    let participant_insertion = state
                        .prisma_client
                        .users_rooms()
                        .create(
                            user::UniqueWhereParam::IdEquals(participant.user_id as i32),
                            rooms::UniqueWhereParam::IdEquals(participant.room_id),
                            vec![],
                        )
                        .exec()
                        .await;
                    match participant_insertion {
                        Ok(_) => {}
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(InviteUserResponse {
                                    success: false,
                                    http_code: 500,
                                    error: Some("Internal server error".to_string()),
                                    validation_errors: None,
                                }),
                            )
                        }
                    };
                    state
                    .redis_client
                    .publish(
                        format!("chat:{}", participant.user_id),
                        serde_json::to_string(&WebSocketMessage {
                            record: crate::socket::interfaces::websocket_message::Records::ParticipantJoined,
                            data: serde_json::json!({}),
                            queue: participant.user_id.to_string(),
                        })
                        .unwrap(),
                    )
                    .await
                    .unwrap();
                    let invite = state
                        .prisma_client
                        .invites()
                        .update(
                            invites::UniqueWhereParam::IdEquals(invite.id),
                            vec![invites::state::set(
                                crate::prisma_client::client::InviteState::Accepted,
                            )],
                        )
                        .exec()
                        .await;
                    match invite {
                        Ok(_) => {
                            return (
                                StatusCode::OK,
                                Json(InviteUserResponse {
                                    success: true,
                                    http_code: 200,
                                    error: None,
                                    validation_errors: None,
                                }),
                            );
                        }
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(InviteUserResponse {
                                    success: false,
                                    http_code: 500,
                                    error: Some("Internal server error".to_string()),
                                    validation_errors: None,
                                }),
                            )
                        }
                    }
                }
                InviteUserReaction::Decline => {
                    let invite = state
                        .prisma_client
                        .invites()
                        .update(
                            invites::UniqueWhereParam::IdEquals(invite.id),
                            vec![invites::state::set(
                                crate::prisma_client::client::InviteState::Declined,
                            )],
                        )
                        .exec()
                        .await;
                    match invite {
                        Ok(_) => {
                            return (
                                StatusCode::OK,
                                Json(InviteUserResponse {
                                    success: true,
                                    http_code: 200,
                                    error: None,
                                    validation_errors: None,
                                }),
                            );
                        }
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(InviteUserResponse {
                                    success: false,
                                    http_code: 500,
                                    error: Some("Internal server error".to_string()),
                                    validation_errors: None,
                                }),
                            )
                        }
                    }
                }
            }
        }
        Err(validation_errors) => {
            let validation_errors =
                validation_errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| CustomValidationError {
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
                Json(InviteUserResponse {
                    success: false,
                    http_code: 422,
                    error: None,
                    validation_errors: Some(validation_errors.collect()),
                }),
            );
        }
    }
}
