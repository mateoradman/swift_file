use std::fs::write;

use axum::{extract::Multipart, http::StatusCode, response::Html, routing::get, Router};

use crate::{files::get_destination_path, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/receive", get(show_form).post(accept_form))
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
    <!doctype html>
    <html>
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1" />
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
