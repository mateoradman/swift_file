use std::net::SocketAddr;
use std::process::exit;

use crate::config::GlobalConfig;
use crate::{receive, send};
use axum::extract::DefaultBodyLimit;
use axum::Router;

pub async fn start_server(config: GlobalConfig) {
    let server_address = config.socket_addr;
    let app = build_router(config).await;
    match axum::Server::try_bind(&server_address) {
        Ok(server) => {
            server
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();
        }
        Err(_) => {
            eprintln!("Unable to bind the server to IP address {server_address}");
            exit(1)
        }
    }
}

async fn build_router(shared_state: GlobalConfig) -> Router {
    Router::new()
        .nest("/", send::router())
        .merge(receive::router())
        .layer(DefaultBodyLimit::disable())
        .with_state(shared_state)
}
