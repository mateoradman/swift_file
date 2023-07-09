use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::exit;
use std::{env, println};

use crate::config::GlobalConfig;
use crate::{receive, send};
use axum::extract::DefaultBodyLimit;
use axum::Router;
use tokio::signal;

pub async fn start_server(config: GlobalConfig) {
    let mut paths: Vec<PathBuf> = Vec::new();
    for path in config.uuid_path_map.lock().unwrap().values() {
        paths.push(path.clone());
    }
    let zip_flag = config.zip;
    let server_address = config.socket_addr;
    let app = build_router(config).await;
    match axum::Server::try_bind(&server_address) {
        Ok(server) => {
            server
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(shutdown_signal(paths, zip_flag))
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

async fn shutdown_signal(paths: Vec<PathBuf>, zip_flag: bool) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutting down sf server...");
    if !zip_flag {
        return;
    }
    println!("Removing created ZIP files...");
    let cwd = env::current_dir().unwrap();
    for path in paths {
        let complete_path = path.canonicalize().unwrap();
        let extension = match complete_path.extension() {
            Some(ext) => ext,
            None => continue,
        };
        if complete_path.exists()
            && complete_path.starts_with(&cwd)
            && complete_path.is_file()
            && extension == "zip"
        {
            let _ = tokio::fs::remove_file(&complete_path).await;
            println!("Successfully removed {}", complete_path.to_str().unwrap())
        }
    }
}
