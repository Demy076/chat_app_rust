use serde::Deserialize;

#[derive(Deserialize)]
pub struct SingleUserParam {
    pub user_id: u32,
}
