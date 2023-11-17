use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;

use crate::prisma_client::client::user;
#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub token: String,
    pub csrf_token: String,
}

// Implement user::Data into UserResponse
impl From<user::Data> for UserResponse {
    fn from(user: user::Data) -> Self {
        UserResponse {
            id: user.id as u64,
            username: user.username,
            email: user.email,
            created_at: user.created_at.format("%d-%m-%Y").to_string(),
            token: user.token,
            csrf_token: user.csrf_token,
        }
    }
}

#[derive(Serialize)]
pub struct CurrentUserResponse {
    pub success: bool,
    pub http_code: u16,
    pub user: UserResponse,
}

pub async fn current_user(
    Extension(user): Extension<user::Data>,
) -> (StatusCode, Json<CurrentUserResponse>) {
    (
        StatusCode::OK,
        Json(CurrentUserResponse {
            success: true,
            http_code: 200,
            user: user.into(),
        }),
    )
}
