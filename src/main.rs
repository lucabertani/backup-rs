use anyhow::Context;
use backup_rs::{
    app_config::AppConfig,
    compress_and_archive,
    dropbox::{connect, create_folder, upload_file},
};
use chrono::Local;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments to get config file path
    let args: Vec<String> = env::args().collect();

    // Check for help argument
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return Ok(());
    }

    let config_path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    // Display which config file is being used
    match &config_path {
        Some(path) => println!("Using config file: {}", path),
        None => println!(
            "Using default config file: config.yaml or configs/config.yaml or /etc/backup-rs/config.yaml"
        ),
    }

    println!("Connecting to Dropbox... ");

    let app_config = AppConfig::load_from_file(config_path.as_deref());

    let client = connect(&app_config)?;

    let today = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let folder_name = format!("/backup_{}", today);

    println!("Creating folder {} on Dropbox", folder_name);

    create_folder(&client, &folder_name)
        .with_context(|| format!("Failed to create folder {folder_name}"))?;

    for folder_config in app_config.folders() {
        let archive_path = compress_and_archive(folder_config, app_config.archive_folder())?;

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

fn print_help() {
    println!("backup-rs - Utility to compress folders and upload to Dropbox");
    println!();
    println!("USAGE:");
    println!("    backup-rs [CONFIG_FILE]");
    println!();
    println!("ARGUMENTS:");
    println!("    <CONFIG_FILE>    Path to the configuration YAML file");
    println!("                     If not provided, uses 'configs/config.yaml'");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    backup-rs                           # Use default config");
    println!("    backup-rs /path/to/myconfig.yaml    # Use custom config");
    println!("    backup-rs ./configs/prod.yaml       # Use relative path");
}
