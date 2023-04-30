mod cli;
mod files;
mod network;
mod qr;
mod receive;
mod send;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::cli::Cli;
use crate::network::{CONTENT_LENGTH_LIMIT, LOCALHOST};
use axum::extract::DefaultBodyLimit;
use axum::Router;
use clap::Parser;

#[derive(Debug, Clone)]
pub struct AppState {
    pub uuid_path_map: Arc<Mutex<HashMap<String, PathBuf>>>,
    pub destination_dir: Option<PathBuf>,
    pub auto_open: bool,
}

#[tokio::main]
async fn main() {
    // create shared state
    let mut shared_state = AppState {
        uuid_path_map: Arc::new(Mutex::new(HashMap::new())),
        destination_dir: None,
        auto_open: true,
    };

    let cli_args = Cli::parse();
    let mut server_port: u16 = 0;
    cli_args.decide(&mut server_port, &mut shared_state);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
