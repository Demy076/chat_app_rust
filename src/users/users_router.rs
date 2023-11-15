use axum::{routing::post, Router};

use crate::shared::arc_clients::State;

use super::handlers::create_user::create_user;

pub fn users_router(state: State) -> Router {
    Router::new()
        .route("/create", post(create_user))
        .with_state(state)
}
