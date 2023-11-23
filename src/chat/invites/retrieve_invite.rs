use crate::{
    prisma_client::client::invites, rejection::path::CustomPathDataRejection,
    shared::arc_clients::State as AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use serde::Serialize;

use super::interfaces::invite_id_param::{self, InviteIdParam};

#[derive(Serialize)]
pub struct Inviter {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct Invite {
    pub id: String,
    pub invitee: Inviter,
    pub inviter: Inviter,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<invites::Data> for Invite {
    fn from(value: invites::Data) -> Self {
        let from = value.from.unwrap();
        let to = value.user.unwrap();
        Self {
            id: value.id,
            created_at: value.created_at.into(),
            invitee: Inviter {
                id: to.id,
                username: to.username,
            },
            inviter: Inviter {
                id: from.id,
                username: from.username,
            },
        }
    }
}

#[derive(Serialize)]
pub struct RetrieveInviteResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<Invite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
pub async fn retrieve_invite(
    State(state): State<AppState>,
    WithRejection(Path(params), _): WithRejection<Path<InviteIdParam>, CustomPathDataRejection>,
) -> (StatusCode, Json<RetrieveInviteResponse>) {
    let invite_id = params.invite_id;
    let invite = state
        .prisma_client
        .invites()
        .find_first(vec![
            invites::id::equals(invite_id),
            invites::state::equals(crate::prisma_client::client::InviteState::Pending),
            invites::created_at::gt((chrono::Utc::now() - chrono::Duration::days(1)).into()),
        ])
        .with(invites::from::fetch())
        .with(invites::user::fetch())
        .exec()
        .await;
    let invite = match invite {
        Ok(invite) => {
            if invite.is_none() {
                return (
                    StatusCode::NOT_FOUND,
                    Json(RetrieveInviteResponse {
                        success: false,
                        http_code: 404,
                        invite: None,
                        error: Some("Invite not found".to_string()),
                    }),
                );
            }
            invite.unwrap()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RetrieveInviteResponse {
                    success: false,
                    http_code: 500,
                    invite: None,
                    error: Some(e.to_string()),
                }),
            )
        }
    };
    (
        StatusCode::OK,
        Json(RetrieveInviteResponse {
            success: true,
            http_code: 200,
            invite: Some(invite.into()),
            error: None,
        }),
    )
}
