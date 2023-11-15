use std::{net::ToSocketAddrs, sync::Arc};

use axum::{error_handling::HandleErrorLayer, BoxError, Router};
use chat_app_rust::{
    error::default_error::default_error, governor::display_error::display_error,
    prisma_client::client::PrismaClient, shared::arc_clients::State,
};
use tower::ServiceBuilder;

use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

#[tokio::main]
async fn main() {
    let state = State {
        prisma_client: Arc::new(
            PrismaClient::_builder()
                .build()
                .await
                .expect("Failed to construct Prisma Client"),
        ),
        redis_client: None,
    };

    let governor = Box::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(50)
            .use_headers()
            .finish()
            .expect("Failed to construct Governor config."),
    );

    let api_address = "localhost:4466"
        .to_socket_addrs()
        .expect("Failed to parse API address")
        .next()
        .unwrap();

    let app = Router::new().fallback(default_error).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: BoxError| async move {
                display_error(e)
            }))
            .layer(GovernorLayer {
                config: Box::leak(governor),
            }),
    );
    axum::Server::bind(&api_address)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .expect("Failed to start server.");
}
