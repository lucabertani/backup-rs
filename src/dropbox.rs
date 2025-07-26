use std::path::Path;

use anyhow::Context;
use dropbox_sdk::default_client::{NoauthDefaultClient, UserAuthDefaultClient};

use crate::app_config::AppConfig;

pub fn connect(app_config: &AppConfig) -> anyhow::Result<UserAuthDefaultClient> {
    if !app_config.dropbox().api_key().is_empty() && !app_config.dropbox().token().is_empty() {
        unsafe {
            std::env::set_var("DBX_CLIENT_ID", app_config.dropbox().api_key());
        }
        unsafe {
            std::env::set_var("DBX_OAUTH", app_config.dropbox().token());
        }
    } else {
        return Err(anyhow::anyhow!("Dropbox API key or token is missing"));
    }

    let mut auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    if auth.save().is_none() {
        auth.obtain_access_token(NoauthDefaultClient::default())
            .context("Unable to get access token")?;
        eprintln!("Next time set these environment variables to reuse this authorization:");
        eprintln!("  DBX_CLIENT_ID={}", auth.client_id());
        eprintln!(
            "  DBX_OAUTH={}",
            auth.save().context("Unable to save OAuth token")?
        );

        anyhow::bail!("Authorization failed, please SAVE your credentials");
    }

    let client = UserAuthDefaultClient::new(auth);

    Ok(client)
}

pub fn create_folder(client: &UserAuthDefaultClient, path: &str) -> anyhow::Result<()> {
    let arg = dropbox_sdk::files::CreateFolderArg::new(path.into());

    dropbox_sdk::files::create_folder_v2(client, &arg)
        .context(format!("Failed to create folder at path {path}"))?;

    Ok(())
}

pub fn upload_file(client: &UserAuthDefaultClient, path: &str, file: &Path) -> anyhow::Result<()> {
    let arg = dropbox_sdk::files::UploadArg::new(path.into());
    let data: Vec<u8> = std::fs::read(file)
        .with_context(|| format!("Failed to read file at path {}", file.display()))?;

    dropbox_sdk::files::upload(client, &arg, &data)?;

    Ok(())
}
