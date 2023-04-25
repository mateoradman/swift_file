use std::{env, path::PathBuf};

use chrono::Local;

fn set_available_filename(path: &mut PathBuf) {
    if path.is_dir() || path.is_file() {
        let now = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        let new_filename = match path.file_name() {
            Some(filename) => format!("{now} - {}", filename.to_str().unwrap()),
            None => String::from("uploaded file"),
        };
        path.set_file_name(new_filename);
    };
}

pub fn get_destination_path(filename: &str, destination_dir: &Option<PathBuf>) -> String {
    let cwd = env::current_dir().unwrap();
    let working_dir = match destination_dir {
        Some(dest_dir) => dest_dir,
        None => &cwd,
    };
    let mut dest_path = working_dir.join(filename);
    set_available_filename(&mut dest_path);
    dest_path.into_os_string().into_string().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_filename() {
        let mut filename = PathBuf::from("bla");
        set_available_filename(&mut filename);
        assert_eq!(filename, PathBuf::from("bla"));
    }
    #[test]
    fn test_get_available_filename_appends_timestamp() {
        let mut working_dir = env::current_dir().unwrap();
        set_available_filename(&mut working_dir);
        let cwd = env::current_dir().unwrap();
        assert!(working_dir.file_name() != cwd.file_name());
    }

    #[test]
    fn test_get_destination_path() {
        let filename = "bla";
        let cwd = env::current_dir().unwrap();
        let path = cwd.join(filename);
        let expected_result = path.to_str().unwrap();
        let dest_path = get_destination_path(filename, &Some(cwd));
        assert_eq!(expected_result, dest_path);
    }
}
