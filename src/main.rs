use std::{net::ToSocketAddrs, sync::Arc};

use axum::{error_handling::HandleErrorLayer, BoxError, Router};
use chat_app_rust::{governor::display_error::display_error, prisma_client::client::PrismaClient};
use tower::{Layer, ServiceBuilder};
// I import my modules using lib.rs
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
#[tokio::main]
async fn main() {
    let client = Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to construct Prisma Client"),
    );

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

    let app = Router::new().layer(
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
