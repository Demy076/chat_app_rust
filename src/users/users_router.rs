use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};

use crate::shared::arc_clients::State;

use super::{
    handlers::{create_user::create_user, current_user::current_user},
    middlewares::is_authenticated::is_authed,
};

pub fn users_router(state: State) -> Router {
    Router::new()
        .route("/create", post(create_user))
        .route("/", get(current_user))
        .layer(from_fn_with_state(state.clone(), is_authed))
        .with_state(state)
}
