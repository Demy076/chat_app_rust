use std::sync::Arc;

use axum::{middleware::from_fn_with_state, Router};

use crate::{shared::arc_clients::State, users::middlewares::is_authenticated::is_authed};

pub fn websocket_router(state: State) -> Router {
    Router::new()
        .layer(from_fn_with_state(state.clone(), is_authed))
        .with_state(state)
}
