mod cli;
mod files;
mod network;
mod qr;
mod receive;
mod send;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::cli::{Cli, Commands};
use crate::network::{find_available_port, CONTENT_LENGTH_LIMIT, LOCALHOST};
use crate::qr::generate_qr_code;
use axum::extract::DefaultBodyLimit;
use axum::Router;
use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AppState {
    pub uuid_path_map: Arc<Mutex<HashMap<String, PathBuf>>>,
}

#[tokio::main]
async fn main() {
    // create shared state
    let shared_state = AppState {
        uuid_path_map: Arc::new(Mutex::new(HashMap::new())),
    };

    let cli_args = Cli::parse();
    let mut server_port: u16 = 0;
    run_selected_action(&cli_args, &mut server_port, &shared_state);

    // build app router
    let app = build_router(shared_state);
    let server_address = format!("{LOCALHOST}:{server_port}");
    println!("Server launched on {server_address}");
    axum::Server::bind(&server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn build_router(shared_state: AppState) -> Router {
    Router::new()
        .nest("/", send::router())
        .merge(receive::router())
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .with_state(shared_state)
}

fn run_selected_action(cli_args: &Cli, server_port: &mut u16, shared_state: &AppState) {
    match &cli_args.command {
        Commands::Send { file, port } => {
            find_available_port(server_port, port);
            let uuid = Uuid::new_v4().to_string();
            shared_state
                .uuid_path_map
                .lock()
                .expect("shared state was poisoned")
                .insert(uuid.clone(), file.clone());
            let route = format!("/download/{}", uuid);
            generate_qr_code(server_port, &route);
        }
        Commands::Receive { port } => {
            find_available_port(server_port, port);
            // generate scannable QR code
            generate_qr_code(server_port, "/receive");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
