use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use serde::Serialize;

use crate::{
    chat::rooms::interfaces::params_chat::RetrieveChatParams,
    prisma_client::client::{user, users_rooms},
    rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
};

enum ParticipantError {
    NotParticipant,
    InternalError,
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
            ParticipantError::InternalError => "Internal Server Error".to_string(),
            ParticipantError::NotParticipant => "Not A Participant".to_string(),
        };
        match self {
            ParticipantError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ParticipantErrorResponse {
                    success: false,
                    http_code: 500,
                    error: error_message,
                }),
            ),
            ParticipantError::NotParticipant => (
                StatusCode::BAD_REQUEST,
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

pub async fn is_participant<B>(
    State(state): State<AppState>,
    Extension(user): Extension<user::Data>,
    WithRejection(Path(chat_params), _): WithRejection<
        Path<RetrieveChatParams>,
        CustomPathDataRejection,
    >,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    if chat_params.id > i32::MAX as u64 {
        return ParticipantError::InternalError.into_response();
    }
    let is_participant = state
        .prisma_client
        .users_rooms()
        .find_first(vec![
            users_rooms::user_id::equals(user.id),
            users_rooms::room_id::equals(chat_params.id as i32),
        ])
        .exec()
        .await;
    let is_participant = match is_participant {
        Ok(participant) => {
            if let Some(participant) = participant {
                participant
            } else {
                return ParticipantError::NotParticipant.into_response();
            }
        }
        Err(_) => {
            return ParticipantError::InternalError.into_response();
        }
    };
    request.extensions_mut().insert(is_participant);
    next.run(request).await
}
