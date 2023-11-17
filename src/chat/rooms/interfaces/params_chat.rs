use serde::Deserialize;

#[derive(Deserialize)]
pub struct RetrieveChatParams {
    pub id: u64,
}
