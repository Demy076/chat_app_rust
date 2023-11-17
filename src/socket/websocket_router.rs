use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{shared::arc_clients::State, users::middlewares::is_authenticated::is_authed};

use super::handlers::websocket_primary_handler::websocket_upgrade;

pub fn websocket_router(state: State) -> Router {
    Router::new()
        .route("/", get(websocket_upgrade))
        .layer(from_fn_with_state(state.clone(), is_authed))
        .with_state(state)
}
