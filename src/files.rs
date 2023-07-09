use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use async_zip::{tokio::write::ZipFileWriter, Compression, ZipEntryBuilder};
use chrono::Local;
use tokio::io::AsyncReadExt;

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

pub async fn create_archive(original_filename: &str, original_path: &Path) -> Result<PathBuf> {
    let zip_path = PathBuf::from(format!("{original_filename}.zip"));
    println!("Creating a ZIP file...");
    let mut zip_file = tokio::fs::File::create(&zip_path).await?;
    let mut writer = ZipFileWriter::with_tokio(&mut zip_file);
    if original_path.is_dir() {
        handle_directory(original_path, &mut writer).await?;
    } else {
        handle_singular(original_path, &mut writer).await?;
    }

    writer.close().await?;
    println!(
        "Successfully created a ZIP file {}",
        original_path.to_str().ok_or(anyhow!("invalid zip path"))?
    );
    Ok(zip_path)
}

async fn handle_singular(
    input_path: &Path,
    writer: &mut ZipFileWriter<&mut tokio::fs::File>,
) -> Result<()> {
    let filename = input_path
        .file_name()
        .ok_or(anyhow!("Input path terminates in '...'."))?;
    let filename = filename
        .to_str()
        .ok_or(anyhow!("Input path not valid UTF-8."))?;

    write_entry(filename, input_path, writer).await
}

async fn handle_directory(
    input_path: &Path,
    writer: &mut ZipFileWriter<&mut tokio::fs::File>,
) -> Result<()> {
    let entries = walk_dir(input_path.into()).await?;
    let input_dir_str = input_path
        .as_os_str()
        .to_str()
        .ok_or(anyhow!("Input path not valid UTF-8."))?;

    for entry_path_buf in entries {
        let entry_path = entry_path_buf.as_path();
        let entry_str = entry_path
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("Directory file path not valid UTF-8."))?;

        if !entry_str.starts_with(input_dir_str) {
            bail!("Directory file path does not start with base input directory path.");
        }

        let entry_str = &entry_str[input_dir_str.len() + 1..];
        write_entry(entry_str, entry_path, writer).await?;
    }

    Ok(())
}

async fn write_entry(
    filename: &str,
    input_path: &Path,
    writer: &mut ZipFileWriter<&mut tokio::fs::File>,
) -> Result<()> {
    let mut input_file = tokio::fs::File::open(input_path).await?;
    let input_file_size = input_file.metadata().await?.len() as usize;

    let mut buffer = Vec::with_capacity(input_file_size);
    input_file.read_to_end(&mut buffer).await?;

    let builder = ZipEntryBuilder::new(filename.into(), Compression::Deflate);
    writer.write_entry_whole(builder, &buffer).await?;

    Ok(())
}

async fn walk_dir(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut dirs = vec![dir];
    let mut files = vec![];

    while !dirs.is_empty() {
        let mut dir_iter = tokio::fs::read_dir(dirs.remove(0)).await?;

        while let Some(entry) = dir_iter.next_entry().await? {
            let entry_path_buf = entry.path();

            if entry_path_buf.is_dir() {
                dirs.push(entry_path_buf);
            } else {
                files.push(entry_path_buf);
            }
        }
    }

    Ok(files)
}
