use crate::prisma_client::client::user;
use crate::shared::arc_clients::State as AppState;
use crate::{
    error::validation_error::ValidationError, prisma_client::client::rooms,
    rejection::json::CustomJsonDataRejection,
};
use axum::Extension;
use axum::{
    extract::{Json as ExtractorJson, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use rustrict::CensorStr;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateChatBody {
    #[validate(
        required(message = "The field `name` is required"),
        length(
            min = 1,
            max = 50,
            message = "Name must be between 1 and 50 characters"
        )
    )]
    pub name: Option<String>,
    // Capacity u8
    #[validate(
        required(message = "The field `capacity` is required"),
        range(min = 1, max = 10, message = "Capacity must be between 1 and 10")
    )]
    pub capacity: Option<u8>,
}

#[derive(Serialize)]
pub struct CreateChat {
    pub name: String,
    pub capacity: u8,
}

impl From<rooms::Data> for CreateChat {
    fn from(value: rooms::Data) -> Self {
        CreateChat {
            name: value.name,
            capacity: value.capacity.try_into().unwrap(),
        }
    }
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub success: bool,
    pub http_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<CreateChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

pub async fn create_chat(
    State(state): State<AppState>,
    Extension(user): Extension<user::Data>,
    WithRejection(ExtractorJson(body), _): WithRejection<
        ExtractorJson<CreateChatBody>,
        CustomJsonDataRejection,
    >,
) -> (StatusCode, Json<CreateChatResponse>) {
    match body.validate() {
        Ok(_) => {
            let (name, capacity) = (body.name.unwrap(), body.capacity.unwrap());
            if name.is_inappropriate() {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(CreateChatResponse {
                        success: false,
                        http_code: 422,
                        chat: None,
                        errors: Some(vec![ValidationError {
                            field: "name".to_string(),
                            messages: vec!["Name is inappropriate".to_string()],
                        }]),
                    }),
                );
            }
            let create_chat = state
                .prisma_client
                .rooms()
                .create(
                    name.clone(),
                    capacity.try_into().unwrap(),
                    user::UniqueWhereParam::IdEquals(user.id),
                    vec![],
                )
                .exec()
                .await;
            let create_chat = match create_chat {
                Ok(create_chat) => create_chat,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CreateChatResponse {
                            success: false,
                            http_code: 500,
                            chat: None,
                            errors: None,
                        }),
                    );
                }
            };
            let owner_participant = state
                .prisma_client
                .users_rooms()
                .create(
                    user::UniqueWhereParam::IdEquals(user.id),
                    rooms::UniqueWhereParam::IdEquals(create_chat.id),
                    vec![],
                )
                .exec()
                .await;
            match owner_participant {
                Ok(_) => (
                    StatusCode::CREATED,
                    Json(CreateChatResponse {
                        success: true,
                        http_code: 201,
                        chat: Some(create_chat.into()),
                        errors: None,
                    }),
                ),
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CreateChatResponse {
                            success: false,
                            http_code: 500,
                            chat: None,
                            errors: None,
                        }),
                    );
                }
            }
        }
        Err(validation_error) => {
            let validation_errors =
                validation_error
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
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(CreateChatResponse {
                    success: false,
                    http_code: 422,
                    chat: None,
                    errors: Some(validation_errors.collect()),
                }),
            );
        }
    }
}
