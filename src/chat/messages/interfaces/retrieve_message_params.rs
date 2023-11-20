use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RetrieveMessageParams {
    #[validate(range(min = 1, max = 50, message = "Limit must be between 1 and 50"))]
    pub limit: i32,
    #[validate(range(min = 0, message = "Message id invalid"))]
    pub message_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct RetrieveSingleMessageParam {
    #[validate(range(min = 1, message = "Message id invalid"))]
    pub message_id: i32,
}
