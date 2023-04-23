use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, Router},
};

use tokio_util::io::ReaderStream;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/download/:file_uuid", get(download_file))
}

async fn download_file(
    State(state): State<AppState>,
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

    println!("{}", file_path.to_str().expect(""));
    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let filename = match file_path.file_name() {
        Some(val) => val.to_str().unwrap(),
        None => return Err((StatusCode::NOT_FOUND, String::from("Wrong filename"))),
    };

    let content_disposition_header = format!("attachment; filename=\"{}\"", filename);
    let file_mime = new_mime_guess::from_path(file_path).first_or_octet_stream();

    let headers = [
        (header::CONTENT_TYPE, file_mime.to_string()),
        (header::CONTENT_DISPOSITION, content_disposition_header),
    ];

    println!("File successfully downloaded.");
    Ok((headers, body))
}
