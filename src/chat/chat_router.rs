use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, patch, post},
    Router,
};
use tower::ServiceBuilder;

use crate::{shared::arc_clients::State, users::middlewares::is_authenticated::is_authed};

use super::{
    messages::{
        delete_message::delete_message, middlewares::can_talk::can_talk,
        retrieve_messages::retrieve_messages, send_message::send_message,
    },
    middlewares::is_participant::is_participant,
    rooms::{
        create_chat::create_chat, join_chat::join_chat, leave_chat::leave_chat,
        retrieve_chat::retrieve_chat,
    },
};

pub fn chat_general_router(state: State) -> Router {
    Router::new().nest("/chat", chatroom_router(state))
}

pub fn chatroom_router(state: State) -> Router {
    Router::new()
        .route("/", post(create_chat))
        .route("/chat-:id", get(retrieve_chat))
        .route("/chat-:id", patch(join_chat))
        .route("/chat-:id", delete(leave_chat))
        .layer(from_fn_with_state(state.clone(), is_authed))
        .with_state(state.clone())
        .nest("/chat-:id/messages", messages_router(state))
}

pub fn messages_router(state: State) -> Router {
    Router::new()
        .route("/:limit/:message_id", get(retrieve_messages))
        .route(
            "/",
            post(send_message).layer(from_fn_with_state(state.clone(), can_talk)),
        )
        .route("/:message_id", delete(delete_message))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state.clone(), is_authed))
                .layer(from_fn_with_state(state.clone(), is_participant)),
        )
        .with_state(state)
}
