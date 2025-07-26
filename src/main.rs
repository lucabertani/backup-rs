use anyhow::Context;
use backup_rs::{
    app_config::AppConfig,
    compress_and_archive,
    dropbox::{connect, create_folder, upload_file},
};
use chrono::Local;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print!("Connecting to Dropbox... ");

    let app_config = AppConfig::load_from_file(None);

    let client = connect(&app_config)?;

    let today = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let folder_name = format!("/backup_{}", today);

    println!("Creating folder {} on Dropbox", folder_name);

    create_folder(&client, &folder_name)
        .with_context(|| format!("Failed to create folder {folder_name}"))?;

    for folder_config in app_config.folders() {
        let archive_path = compress_and_archive(folder_config)?;

        let dropbox_path = format!(
            "{}/{}",
            folder_name,
            archive_path
                .file_name()
                .expect("Unable to extract file_name")
                .to_str()
                .expect("Unable to extract str")
        );

        println!(
            "send archive {} to Dropbox path {}",
            archive_path.display(),
            dropbox_path
        );

        upload_file(&client, &dropbox_path, &archive_path)?;

        // delete archive
        std::fs::remove_file(&archive_path)
            .with_context(|| format!("Failed to delete archive file {}", archive_path.display()))?;
    }

    Ok(())
}

// cross build --target aarch64-unknown-linux-gnu
// cross build --target armv7-unknown-linux-gnueabihf
