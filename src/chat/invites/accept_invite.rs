use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::StatusCode,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    error::validation_error::ValidationError as CustomValidationError,
    prisma_client::client::users_rooms,
    rejection::{json::CustomJsonDataRejection, path::CustomPathDataRejection},
    shared::arc_clients::State as AppState,
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
        Ok(_) => (
            StatusCode::OK,
            Json(InviteUserResponse {
                success: true,
                http_code: 200,
                error: None,
                validation_errors: None,
            }),
        ),
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
