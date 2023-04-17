use std::{env, path::PathBuf};

use chrono::Local;

fn get_available_filename(filename: &str) -> String {
    let now = Local::now();
    let now = now.format("%d-%m-%Y %H:%M:%S").to_string();
    let path = PathBuf::from(filename);
    if path.is_dir() || path.is_file() {
        let new_filename = format!("{now} - {filename}");
        return new_filename;
    };
    filename.to_string()
}

pub fn get_destination_path(filename: &str) -> String {
    let new_filename = get_available_filename(filename);
    let cwd = env::current_dir().unwrap();
    let dest_path = cwd.join(new_filename);
    dest_path.into_os_string().into_string().unwrap()
}
