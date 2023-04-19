use std::fs::write;

use local_ip_address::local_ip;
use qrcode::render::unicode;
use qrcode::QrCode;

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::files::get_destination_path;

const ROUTE: &str = "/receive";

pub fn router() -> Router {
    Router::new()
        .route(ROUTE, get(show_form).post(accept_form))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
<!doctype html>
<html>
    <head>
        <title>Swift File</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                background-color: #f0f0f0;
            }
            form {
                margin: 48px;
                padding: 24px;
                background-color: #fff;
                border: 1px solid #ccc;
                border-radius: 5px;
            }
            label {
                display: block;
                margin-bottom: 10px;
                font-weight: bold;
            }
            input[type="file"] {
                display: block;
                margin-bottom: 10px;
                border: 1px solid #ccc;
                padding: 5px;
                border-radius: 5px;
            }
            input[type="submit"] {
                background-color: #4CAF50;
                color: #fff;
                border: none;
                padding: 10px 20px;
                border-radius: 5px;
                cursor: pointer;
            }
        </style>
    </head>
    <body>
        <h1>Swift File</h1>
        <form action="/receive" method="post" enctype="multipart/form-data">
            <label>
                Choose one or more files to upload:
                <input type="file" name="file" multiple>
            </label>

            <input type="submit" value="Upload">
        </form>
    </body>
</html>
"#,
    )
}

async fn accept_form(mut multipart: Multipart) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let dest_path = get_destination_path(&file_name);
        write(&dest_path, &data).unwrap();

        println!(
            "File `{}` with content type `{}`, size {} bytes has been transferred to {}",
            file_name,
            content_type,
            data.len(),
            dest_path
        );
    }
    StatusCode::OK
}

pub fn generate_receive_qr_code(port: u16) {
    let machine_ip = local_ip().unwrap().to_string();
    let complete_url = format!("http://{machine_ip}:{port}{ROUTE}");
    let code = QrCode::new(complete_url).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}
