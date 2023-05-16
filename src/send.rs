use std::net::SocketAddr;

use axum::{
    body::StreamBody,
    extract::{ConnectInfo, Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, Router},
};

use tokio_util::io::ReaderStream;

use crate::GlobalConfig;

pub fn router() -> Router<GlobalConfig> {
    Router::new().route("/download/:file_uuid", get(download_file))
}

async fn download_file(
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    State(state): State<GlobalConfig>,
    Path(file_uuid): Path<String>,
) -> impl IntoResponse {
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

    let file = match tokio::fs::File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let file_length = file.metadata().await.unwrap().len().to_string();
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let filename = match file_path.file_name() {
        Some(val) => val.to_str().unwrap(),
        None => return Err((StatusCode::NOT_FOUND, String::from("Wrong filename"))),
    };

    println!(
        "Client with IP {} requested to download {}",
        &client_addr.ip(),
        file_path.canonicalize().unwrap().to_str().unwrap(),
    );
    let content_disposition_header = format!("attachment; filename=\"{}\"", filename);
    let file_mime = new_mime_guess::from_path(file_path).first_or_octet_stream();

    let headers = [
        (header::CONTENT_TYPE, file_mime.to_string()),
        (header::CONTENT_DISPOSITION, content_disposition_header),
        (header::CONTENT_LENGTH, file_length),
    ];

    println!(
        "Client with IP {} successfully downloaded the file.",
        &client_addr.ip()
    );
    Ok((headers, body))
}
