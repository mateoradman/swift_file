use humansize::{format_size, DECIMAL};
use std::fs::write;

use axum::{
    extract::{Multipart, State},
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

async fn accept_form(State(state): State<GlobalConfig>, mut multipart: Multipart) -> Redirect {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let human_size: String = format_size(data.len(), DECIMAL);
        let dest_path = get_destination_path(&file_name, &state.destination_dir);
        let written: bool = match write(&dest_path, &data) {
            Ok(_) => {
                println!(
                    "Successfully transferred {}",
                    construct_file_info(&file_name, &content_type, &human_size, &dest_path),
                );
                true
            }
            Err(err) => {
                println!(
                    "Unable to transfer {} due to `{}`",
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

    Redirect::to("/receive")
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
