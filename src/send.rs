use std::{format, net::SocketAddr, println};

use axum::{
    body::StreamBody,
    extract::{ConnectInfo, Path, State},
    http::{header, HeaderName, StatusCode},
    routing::{get, Router},
};

use tokio_util::io::ReaderStream;

use crate::GlobalConfig;

pub fn router() -> Router<GlobalConfig> {
    Router::new().route("/download/:file_uuid", get(send_file))
}

async fn send_file(
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    State(state): State<GlobalConfig>,
    Path(file_uuid): Path<String>,
) -> Result<
    (
        [(HeaderName, String); 3],
        StreamBody<ReaderStream<tokio::fs::File>>,
    ),
    (StatusCode, String),
> {
    let file_path = match state.uuid_path_map.lock() {
        Ok(lock) => match lock.get(&file_uuid) {
            Some(val) => val.clone(),
            None => return Err((StatusCode::NOT_FOUND, String::from("Wrong file UUID"))),
        },
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong with getting the file path: {}", err),
            ))
        }
    };

    println!(
        "Client (IP {}) requested to download {}",
        &client_addr.ip(),
        file_path.canonicalize().unwrap().to_str().unwrap(),
    );

    let file = match tokio::fs::File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let file_length = file.metadata().await.unwrap().len().to_string();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    // file metadata
    let filename = match file_path.file_name() {
        Some(val) => val.to_str().unwrap_or("file"),
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Unable to determine filename"),
            ))
        }
    };
    let content_disposition_header = format!("attachment; filename=\"{}\"", filename);
    let file_mime = match state.zip {
        true => String::from("application/zip"),
        false => new_mime_guess::from_path(file_path)
            .first_or_octet_stream()
            .to_string(),
    };
    let headers = [
        (header::CONTENT_TYPE, file_mime),
        (header::CONTENT_DISPOSITION, content_disposition_header),
        (header::CONTENT_LENGTH, file_length),
    ];

    println!(
        "Client (IP {}) successfully downloaded the file.",
        &client_addr.ip()
    );
    Ok((headers, body))
}
