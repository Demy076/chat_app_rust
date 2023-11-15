use std::sync::Arc;

use crate::prisma_client::client::PrismaClient;

#[derive(Clone)]
pub struct State {
    pub prisma_client: Arc<PrismaClient>,
    // For now sugar coat it with option we'll implement it later
    pub redis_client: Option<Arc<rustis::client::Client>>,
}
