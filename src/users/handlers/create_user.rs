use std::sync::Arc;

use crate::{
    error::validation_error::ValidationError,
    prisma_client::client::{user, PrismaClient},
    rejection::json::CustomJsonDataRejection,
    shared::arc_clients::State as app_state,
};

use axum::{
    http::StatusCode,
    Json,
    {extract::State, Json as ExtractedJson},
};
use axum_client_ip::XForwardedFor;
use axum_extra::extract::{cookie::Cookie, CookieJar, WithRejection};
use prisma_client_rust::operator::{and, or};
use rustrict::CensorStr;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(
        required(message = "username is required"),
        // Max between 3 and 32
        length(min = 3, max = 32, message = "username must be between 3 and 32 characters")
    )]
    pub username: Option<String>,
    #[validate(
        required(message = "email is required"),
        email(message = "email is not valid")
    )]
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
}

pub async fn can_create(
    ip: &str,
    prisma_client: Arc<PrismaClient>,
) -> Result<bool, prisma_client_rust::QueryError> {
    let user = prisma_client
        .user()
        .find_first(vec![and(vec![
            user::ip::equals(ip.to_string()),
            user::banned::equals(true),
        ])])
        .exec()
        .await?;
    Ok(user.is_none())
}

pub async fn create_user(
    State(state): State<app_state>,
    XForwardedFor(ip): XForwardedFor,
    jar: CookieJar,
    WithRejection(ExtractedJson(body), _): WithRejection<
        ExtractedJson<CreateUserRequest>,
        CustomJsonDataRejection,
    >,
) -> Result<
    (CookieJar, (StatusCode, Json<CreateUserResponse>)),
    (StatusCode, Json<CreateUserResponse>),
> {
    let ip = ip[0];
    if ip.is_loopback() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(CreateUserResponse {
                success: false,
                http_code: 422,
                validation_errors: None,
                csrf_token: None,
                error: Some("IP is not allowed".to_string()),
            }),
        ));
    }
    let can_register = can_create(&ip.to_string(), state.prisma_client.clone()).await;
    match can_register {
        Ok(can_register) => {
            if !can_register {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(CreateUserResponse {
                        success: false,
                        http_code: 422,
                        validation_errors: None,
                        csrf_token: None,
                        error: Some("IP is not allowed".to_string()),
                    }),
                ));
            }
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateUserResponse {
                    success: false,
                    http_code: 500,
                    validation_errors: None,
                    csrf_token: None,
                    error: Some("Internal server error".to_string()),
                }),
            ));
        }
    };
    match body.validate() {
        Ok(_) => {
            let (username, email) = (body.username.unwrap(), body.email.unwrap());
            if username.is_inappropriate() {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(CreateUserResponse {
                        success: false,
                        http_code: 422,
                        validation_errors: None,
                        csrf_token: None,
                        error: Some("Username is inappropriate".to_string()),
                    }),
                ));
            }
            if email.is_inappropriate() {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(CreateUserResponse {
                        success: false,
                        http_code: 422,
                        validation_errors: None,
                        csrf_token: None,
                        error: Some("Email is inappropriate".to_string()),
                    }),
                ));
            }
            let user = state
                .prisma_client
                .user()
                .find_first(vec![or(vec![
                    user::username::equals(username.clone()),
                    user::email::equals(email.clone()),
                ])])
                .exec()
                .await;
            match user {
                Ok(user) => {
                    if user.is_some() {
                        return Err((
                            StatusCode::CONFLICT,
                            Json(CreateUserResponse {
                                success: false,
                                http_code: 409,
                                validation_errors: None,
                                csrf_token: None,
                                error: Some("Username / email is already taken".to_string()),
                            }),
                        ));
                    }
                }
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CreateUserResponse {
                            success: false,
                            http_code: 500,
                            validation_errors: None,
                            csrf_token: None,
                            error: Some("Internal server error".to_string()),
                        }),
                    ));
                }
            };
            let user = state
                .prisma_client
                .user()
                .create(email, username, ip.to_string(), vec![])
                .exec()
                .await;
            let user = match user {
                Ok(user) => user,
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CreateUserResponse {
                            success: false,
                            http_code: 500,
                            validation_errors: None,
                            csrf_token: None,
                            error: Some("Internal server error".to_string()),
                        }),
                    ));
                }
            };
            let offset_week = time::OffsetDateTime::from_unix_timestamp(
                chrono::Utc::now().timestamp() + 60 * 60 * 24 * 7,
            );
            let offset_week = match offset_week {
                Ok(offset_week) => offset_week,
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CreateUserResponse {
                            success: false,
                            http_code: 500,
                            validation_errors: None,
                            csrf_token: None,
                            error: Some("Internal server error".to_string()),
                        }),
                    ));
                }
            };

            let jar = jar.add(
                Cookie::build("session", user.token)
                    .secure(true)
                    .http_only(true)
                    .path("/")
                    .same_site(axum_extra::extract::cookie::SameSite::Lax)
                    .expires(offset_week)
                    .finish(),
            );

            Ok((
                jar,
                (
                    StatusCode::CREATED,
                    Json(CreateUserResponse {
                        success: true,
                        http_code: 201,
                        validation_errors: None,
                        csrf_token: None,
                        error: None,
                    }),
                ),
            ))
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
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(CreateUserResponse {
                    success: false,
                    http_code: 422,
                    validation_errors: Some(validation_errors.collect()),
                    csrf_token: None,
                    error: None,
                }),
            ));
        }
    }
}
