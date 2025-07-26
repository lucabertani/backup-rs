use backup_rs::{app_config::AppConfig, compress_and_archive};

#[test]
fn test_compress() {
    let app_config = AppConfig::load_from_file(None);
    let folder_config = app_config.folders.first().expect("No folder config found");
    let path = compress_and_archive(folder_config).expect("Compression failed");

    println!("Compression successful: {:?}", path);
}
