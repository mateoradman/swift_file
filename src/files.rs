use std::path::PathBuf;

use chrono::Utc;

pub fn get_filename(filename: &str) -> String {
    let now = Utc::now();
    let path = PathBuf::from(filename);
    if path.is_dir() || path.is_file() {
        let new_filename = format!("{filename}{now}");
        return new_filename;
    };
    filename.to_string()
}
