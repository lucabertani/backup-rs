use serde::{Deserialize, Serialize};

use crate::configuration::{dropbox_config::DropboxConfig, folder::FolderConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub dropbox: DropboxConfig,
    pub folders: Vec<FolderConfig>,
}

impl AppConfig {
    pub fn load_from_file(path: Option<&str>) -> AppConfig {
        let mut paths = vec![
            "/etc/backup-rs/config.yaml",
            "configs/config.yaml",
            "config.yaml",
        ];

        if let Some(path) = path {
            paths.insert(0, path);
        }

        for path in paths.iter() {
            if let Ok(file) = std::fs::File::open(path) {
                if let Ok(config) = serde_yaml::from_reader(file) {
                    return config;
                }
            }
        }

        panic!(
            "Failed to load configuration file from any of the specified paths: {:?}",
            paths
        );
    }

    pub fn dropbox(&self) -> &DropboxConfig {
        &self.dropbox
    }

    pub fn folders(&self) -> &[FolderConfig] {
        &self.folders
    }
}
