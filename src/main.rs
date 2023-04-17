mod cli;
mod files;
mod network;
mod receive;

use axum::{extract::DefaultBodyLimit, Router};
use clap::Parser;
use tower_http::limit::RequestBodyLimitLayer;

use crate::cli::{Cli, Commands};
use crate::network::{find_available_port, is_port_available, LOCALHOST};
use crate::receive::generate_receive_qr_code;

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();
    let mut server_port: u16 = 0;
    match &cli_args.command {
        Commands::Send { file, port } => println!("method send"),
        Commands::Receive { port } => {
            if let Some(port) = port {
                if is_port_available(*port) {
                    server_port = *port;
                }
            } else {
                server_port = find_available_port();
            }
            // generate scannable QR code
            generate_receive_qr_code(server_port);
        }
    };

    // build app router
    let app = Router::new()
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .merge(receive::router());

    let server_address = format!("{LOCALHOST}:{server_port}");
    println!("Server launched on {server_address}");
    axum::Server::bind(&server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
