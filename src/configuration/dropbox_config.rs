use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxConfig {
    pub api_key: String,
    pub token: String,
}

impl DropboxConfig {
    pub fn new(api_key: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            token: token.into(),
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
