use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use serde::Serialize;
use validator::Validate;

use crate::{
    error::validation_error::ValidationError,
    prisma_client::client::{banned_users_room, user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
};

use super::ban_user::BanUserParams;

#[derive(Serialize)]
pub struct UnbanUserErrorResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn unban_user(
    State(state): State<AppState>,
    Extension(participant): Extension<users_rooms::Data>,
    WithRejection(Path(params), _): WithRejection<Path<BanUserParams>, CustomPathDataRejection>,
) -> Result<StatusCode, (StatusCode, Json<UnbanUserErrorResponse>)> {
    match params.validate() {
        Ok(_) => {
            let user_id = params.user_id;
            let user = state
                .prisma_client
                .user()
                .find_unique(user::UniqueWhereParam::IdEquals(user_id))
                .with(user::banned_users_room::fetch(vec![
                    banned_users_room::room_id::equals(participant.room_id),
                ]))
                .exec()
                .await;
            let user = match user {
                Ok(user) => {
                    if user.is_none() {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(UnbanUserErrorResponse {
                                success: false,
                                http_code: 400,
                                error: Some("User not found".to_string()),
                                validation_errors: None,
                            }),
                        ));
                    }
                    let user = user.unwrap();
                    match user.banned_users_room() {
                        Ok(_) => user,
                        Err(_) => {
                            return Err((
                                StatusCode::BAD_REQUEST,
                                Json(UnbanUserErrorResponse {
                                    success: false,
                                    http_code: 400,
                                    error: Some("User is not banned".to_string()),
                                    validation_errors: None,
                                }),
                            ));
                        }
                    }
                }
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(UnbanUserErrorResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    ));
                }
            };
            let user = state
                .prisma_client
                .banned_users_room()
                .delete(banned_users_room::UniqueWhereParam::IdEquals(
                    user.banned_users_room.unwrap()[0].id,
                ))
                .exec()
                .await;
            match user {
                Ok(_) => Ok(StatusCode::NO_CONTENT),
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(UnbanUserErrorResponse {
                            success: false,
                            http_code: 500,
                            error: Some("Internal server error".to_string()),
                            validation_errors: None,
                        }),
                    ));
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
            return Err((
                StatusCode::BAD_GATEWAY,
                Json(UnbanUserErrorResponse {
                    success: false,
                    http_code: 400,
                    validation_errors: Some(validation_errors.collect()),
                    error: None,
                }),
            ));
        }
    }
}
