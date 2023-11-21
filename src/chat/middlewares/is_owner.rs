use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Serialize;

use crate::prisma_client::client::{user, users_rooms};

enum ParticipantError {
    NotOwner,
}

#[derive(Serialize)]
pub struct ParticipantErrorResponse {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

impl IntoResponse for ParticipantError {
    fn into_response(self) -> axum::response::Response {
        let error_message: String = match self {
            ParticipantError::NotOwner => "Not the owner".to_string(),
        };
        match self {
            ParticipantError::NotOwner => (
                StatusCode::UNAUTHORIZED,
                Json(ParticipantErrorResponse {
                    success: false,
                    http_code: 400,
                    error: error_message,
                }),
            ),
        }
        .into_response()
    }
}

pub async fn is_owner<B>(
    Extension(user): Extension<user::Data>,
    Extension(participant): Extension<users_rooms::Data>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let is_owner = participant.room.unwrap().user_id == user.id;
    if !is_owner {
        return ParticipantError::NotOwner.into_response();
    }
    next.run(request).await
}
