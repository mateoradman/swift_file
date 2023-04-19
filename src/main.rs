mod cli;
mod files;
mod network;
mod receive;
mod send;

use axum::Router;
use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::network::{find_available_port, LOCALHOST};
use crate::receive::generate_receive_qr_code;

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();
    let mut server_port: u16 = 0;
    match &cli_args.command {
        Commands::Send { file, port } => find_available_port(&mut server_port, port),
        Commands::Receive { port } => {
            find_available_port(&mut server_port, port);
            // generate scannable QR code
            generate_receive_qr_code(server_port);
        }
    };

    // build app router
    let app = Router::new().merge(receive::router());

    let server_address = format!("{LOCALHOST}:{server_port}");
    println!("Server launched on {server_address}");
    axum::Server::bind(&server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
