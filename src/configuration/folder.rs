use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderConfig {
    pub path: PathBuf,
    #[serde(default)]
    pub exclude_folders: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub exclude_files: Option<Vec<String>>,
}

impl FolderConfig {}
