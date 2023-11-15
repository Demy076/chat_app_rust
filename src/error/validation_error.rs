use serde::Serialize;

#[derive(Serialize)]
pub struct ValidationError {
    pub field: String,
    pub messages: Vec<String>,
}
