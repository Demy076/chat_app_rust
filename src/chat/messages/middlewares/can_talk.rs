use std::sync::Arc;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use rustis::{
    client::Client,
    commands::{GenericCommands, HashCommands},
};
use serde::Serialize;

use crate::{prisma_client::client::users_rooms, shared::arc_clients::State as AppState};

enum CanTalkError {
    Muted,
    TooFast,
    InternalError,
}

#[derive(Serialize)]
pub struct CanTalkErrorResponse {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

impl IntoResponse for CanTalkError {
    fn into_response(self) -> axum::response::Response {
        let error_message: String = match self {
            CanTalkError::Muted => "Muted".to_string(),
            CanTalkError::TooFast => "Sending Messages Too Fast".to_string(),
            CanTalkError::InternalError => "Internal Server Error".to_string(),
        };
        match self {
            CanTalkError::Muted => (
                StatusCode::BAD_REQUEST,
                Json(CanTalkErrorResponse {
                    success: false,
                    http_code: 400,
                    error: error_message,
                }),
            ),
            CanTalkError::TooFast => (
                StatusCode::TOO_MANY_REQUESTS,
                Json(CanTalkErrorResponse {
                    success: false,
                    http_code: 429,
                    error: error_message,
                }),
            ),
            CanTalkError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CanTalkErrorResponse {
                    success: false,
                    http_code: 500,
                    error: error_message,
                }),
            ),
        }
        .into_response()
    }
}
pub async fn check_ratelimit(
    redis_client: Arc<Client>,
    user_id: i64,
    chat_id: i64,
) -> Result<bool, rustis::Error> {
    let key = format!("ratelimit:{}:{}", user_id, chat_id);
    let command: Option<String> = redis_client.hget(&key, "count").await?;
    match command {
        Some(count) => {
            let count = count.parse::<i64>();
            let count = match count {
                Ok(count) => count,
                Err(_) => return Ok(false),
            };
            if count >= 5 {
                Ok(false)
            } else {
                redis_client.hincrby(&key, "count", 1).await?;
                Ok(true)
            }
        }
        None => {
            redis_client.hincrby(&key, "count", 1).await?;
            let is_set = redis_client
                .expire(&key, 5, rustis::commands::ExpireOption::None)
                .await?;
            if !is_set {
                redis_client.del(&key).await?;
                return Ok(false);
            }
            Ok(true)
        }
    }
}
pub async fn can_talk<B>(
    State(state): State<AppState>,
    Extension(participant_room): Extension<users_rooms::Data>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if participant_room.muted {
        return CanTalkError::Muted.into_response();
    }
    let rate_limit = check_ratelimit(
        state.redis_client,
        participant_room.user_id.into(),
        participant_room.room_id.into(),
    )
    .await;
    let rate_limit = match rate_limit {
        Ok(rate_limit) => rate_limit,
        Err(_) => return CanTalkError::InternalError.into_response(),
    };
    if !rate_limit {
        return CanTalkError::TooFast.into_response();
    }

    next.run(request).await
}
