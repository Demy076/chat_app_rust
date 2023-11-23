use serde::Deserialize;

#[derive(Deserialize)]
pub struct InviteIdParam {
    pub invite_id: u32,
}
