use std::fs::write;

use local_ip_address::local_ip;
use qrcode::render::unicode;
use qrcode::QrCode;

use axum::{extract::Multipart, response::Html, routing::get, Router};

use crate::files::get_filename;

const ROUTE: &str = "/receive";

pub fn router() -> Router {
    Router::new().route(ROUTE, get(show_form).post(accept_form))
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/receive" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let filename = get_filename(&file_name);
        write(&filename, &data).unwrap();

        println!(
            "File `{}` with content type `{}` and size {} bytes has successfully been transferred. The destination path is: ./{}",
            file_name,
            content_type,
            data.len(),
            filename
        );
    }
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
