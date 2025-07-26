use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::Context;
use chrono::Local;
use zip::{ZipWriter, write::FileOptions};

use crate::configuration::folder::FolderConfig;

pub mod app_config;
pub mod configuration;
pub mod dropbox;

static ARCHIVE_DIR: &str = "archive";

pub fn compress_and_archive(folder_config: &FolderConfig) -> anyhow::Result<PathBuf> {
    // Create the archive folder if it doesn't exist
    let archive_dir = PathBuf::from(ARCHIVE_DIR);
    if !archive_dir.exists() {
        fs::create_dir_all(&archive_dir).with_context(|| "Failed to create archive directory")?;
    }

    // Get the name of the folder to compress
    let folder_name = folder_config
        .path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    // Create the ZIP filename with today's date
    // let today = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let today = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let zip_filename = format!("{}_{}.zip", folder_name, today);
    let zip_path = archive_dir.join(&zip_filename);

    // println!("Creating archive: {}", zip_path.display());

    // Create the ZIP file
    let zip_file = File::create(&zip_path)
        .with_context(|| format!("Failed to create ZIP file: {}", zip_path.display()))?;
    let mut zip_writer = ZipWriter::new(BufWriter::new(zip_file));

    // Recursively compress all files and folders
    compress_directory_recursive(
        &mut zip_writer,
        &folder_config.path,
        &folder_config.path,
        folder_config,
    )?;

    // Finalize the ZIP file
    zip_writer
        .finish()
        .with_context(|| "Failed to finalize ZIP file")?;

    // println!("Archive created successfully: {}", zip_path.display());
    Ok(zip_path)
}

fn should_exclude(
    full_path: &Path,
    relative_path: &Path,
    file_type: &fs::FileType,
    folder_config: &FolderConfig,
) -> bool {
    // Check if it's a folder to exclude
    if file_type.is_dir() {
        if let Some(exclude_folders) = &folder_config.exclude_folders {
            for excluded_folder in exclude_folders {
                // Compare the relative path with exclusion patterns
                if relative_path == excluded_folder || relative_path.starts_with(excluded_folder) {
                    return true;
                }
            }
        }
    }

    // Check if it's a file to exclude
    if file_type.is_file() {
        if let Some(exclude_files) = &folder_config.exclude_files {
            if let Some(filename) = full_path.file_name().and_then(|name| name.to_str()) {
                for excluded_pattern in exclude_files {
                    // Support simple patterns with wildcards
                    if matches_pattern(filename, excluded_pattern) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn matches_pattern(filename: &str, pattern: &str) -> bool {
    // Simple implementation for patterns with wildcards
    if pattern.contains('*') {
        // Pattern with wildcard (e.g. "*.tmp", "temp*", "*cache*")
        if pattern.starts_with('*') && pattern.ends_with('*') {
            // Pattern like "*cache*"
            let middle = &pattern[1..pattern.len() - 1];
            filename.contains(middle)
        } else if pattern.starts_with('*') {
            // Pattern like "*.txt"
            let suffix = &pattern[1..];
            filename.ends_with(suffix)
        } else if pattern.ends_with('*') {
            // Pattern like "temp*"
            let prefix = &pattern[..pattern.len() - 1];
            filename.starts_with(prefix)
        } else {
            // Pattern with wildcard in the middle - use simple check
            filename.contains(&pattern.replace("*", ""))
        }
    } else {
        // Exact match
        filename == pattern
    }
}

fn compress_directory_recursive(
    zip_writer: &mut ZipWriter<BufWriter<File>>,
    current_path: &Path,
    root_path: &Path,
    folder_config: &FolderConfig,
) -> anyhow::Result<()> {
    let entries = fs::read_dir(current_path)
        .with_context(|| format!("Failed to read directory: {}", current_path.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to get file type for: {}", path.display()))?;

        // Calculate the relative path from the origin to maintain the folder structure
        let relative_path = path
            .strip_prefix(root_path)
            .with_context(|| format!("Failed to create relative path for: {}", path.display()))?;

        // Check if this element should be excluded
        if should_exclude(&path, relative_path, &file_type, folder_config) {
            // println!("Skipping excluded element: {}", relative_path.display());
            continue;
        }

        if file_type.is_file() {
            // Add the file to the ZIP
            add_file_to_zip(zip_writer, &path, relative_path)?;
        } else if file_type.is_dir() {
            // Add the folder to the ZIP (as empty entry)
            add_directory_to_zip(zip_writer, relative_path)?;

            // Recursively compress the folder content
            compress_directory_recursive(zip_writer, &path, root_path, folder_config)?;
        }
    }

    Ok(())
}

fn add_file_to_zip(
    zip_writer: &mut ZipWriter<BufWriter<File>>,
    file_path: &Path,
    relative_path: &Path,
) -> anyhow::Result<()> {
    // println!("Adding file: {}", relative_path.display());

    // Convert the path to Unix format (with /) for ZIP compatibility
    let zip_path = relative_path.to_string_lossy().replace('\\', "/");

    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip_writer
        .start_file(&zip_path, options)
        .with_context(|| format!("Failed to start ZIP file entry: {}", zip_path))?;

    // Read and copy the file content
    let file_content = fs::read(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    zip_writer
        .write_all(&file_content)
        .with_context(|| format!("Failed to write file to ZIP: {}", zip_path))?;

    Ok(())
}

fn add_directory_to_zip(
    zip_writer: &mut ZipWriter<BufWriter<File>>,
    relative_path: &Path,
) -> anyhow::Result<()> {
    // println!("Adding folder: {}", relative_path.display());

    // Convert the path to Unix format and add trailing slash for folders
    let mut zip_path = relative_path.to_string_lossy().replace('\\', "/");
    if !zip_path.ends_with('/') {
        zip_path.push('/');
    }

    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip_writer
        .start_file(&zip_path, options)
        .with_context(|| format!("Failed to start ZIP directory entry: {}", zip_path))?;

    // Folders have no content, so we don't write anything

    Ok(())
}
