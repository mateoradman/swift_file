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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_filename() {
        let filename = "bla";
        let available = get_available_filename(filename);
        assert_eq!(filename, &available);
    }
    #[test]
    fn test_get_available_filename_appends_timestamp() {
        let cwd = env::current_dir().unwrap();
        let cwd_as_string = cwd.to_str().unwrap();
        let available = get_available_filename(cwd_as_string);
        assert!(available.len() > cwd_as_string.len());
        assert!(available.contains(cwd_as_string));
    }

    #[test]
    fn test_get_destination_path() {
        let filename = "bla";
        let cwd = env::current_dir().unwrap();
        let path = cwd.join(filename);
        let expected_result = path.to_str().unwrap();
        let dest_path = get_destination_path(filename);
        assert_eq!(expected_result, dest_path);
    }
}
