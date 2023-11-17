use std::sync::Arc;

use rustis::{
    client::Client,
    commands::{GenericCommands, HashCommands},
};

pub async fn check_ratelimit(
    user_id: i64,
    uuid: String,
    redis_client: Arc<Client>,
) -> Result<bool, rustis::Error> {
    let key = format!("ratelimit:{}:{}", user_id, uuid);
    let command: Option<String> = redis_client.hget(&key, "count").await?;
    match command {
        Some(count) => {
            let count: i64 = count.parse().unwrap();
            if count >= 60 {
                Ok(false)
            } else {
                redis_client.hincrby(&key, "count", 1).await?;
                Ok(true)
            }
        }
        None => {
            redis_client.hincrby(&key, "count", 1).await?;
            redis_client
                .expire(&key, 60, rustis::commands::ExpireOption::None)
                .await?;
            Ok(true)
        }
    }
}
