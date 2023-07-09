use humansize::{format_size, DECIMAL};
use std::{fs::write, net::SocketAddr};

use axum::{
    extract::{ConnectInfo, Multipart, State},
    http::StatusCode,
    response::{Html, Redirect},
    routing::get,
    Router,
};

use crate::{files::get_destination_path, GlobalConfig};

pub fn router() -> Router<GlobalConfig> {
    Router::new().route("/receive", get(show_form).post(accept_form))
}

static HTML_FORM: &str = include_str!("./resources/form.html");
async fn show_form() -> Html<&'static str> {
    Html(HTML_FORM)
}

async fn accept_form(
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    State(state): State<GlobalConfig>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let file_name = field.file_name().unwrap_or("uploaded file").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();
        let data = field.bytes().await.unwrap();
        let size = data.len();
        if size == 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("Please upload at least one file"),
            ));
        }
        let human_size: String = format_size(data.len(), DECIMAL);
        let dest_path = get_destination_path(&file_name, &state.destination_dir);
        let written: bool = match write(&dest_path, &data) {
            Ok(_) => {
                println!(
                    "Client {} successfully transferred {}",
                    &client_addr.ip(),
                    construct_file_info(&file_name, &content_type, &human_size, &dest_path),
                );
                true
            }
            Err(err) => {
                println!(
                    "Client {} attempted to transfer {} but it failed due to `{}`",
                    &client_addr.ip(),
                    construct_file_info(&file_name, &content_type, &human_size, &dest_path),
                    err
                );
                false
            }
        };

        if written && state.auto_open {
            match opener::open(&dest_path) {
                Ok(()) => println!("File opened using a system default program."),
                Err(err) => {
                    println!("Unable to open a file using the system default program due to {err}")
                }
            }
        }
    }

    Ok(Redirect::to("/receive"))
}

fn construct_file_info(
    file_name: &str,
    content_type: &str,
    human_size: &str,
    destination_path: &str,
) -> String {
    format!(
        "file `{}` with content type `{}` and size `{}` to `{}`",
        file_name, content_type, human_size, destination_path
    )
}
