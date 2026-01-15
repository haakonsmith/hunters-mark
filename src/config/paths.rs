use crate::error::Result;
use directories::ProjectDirs;
use std::path::PathBuf;

/// Get the config directory for hunters-mark using XDG conventions
pub fn config_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "hunters-mark")
        .ok_or_else(|| color_eyre::eyre::eyre!("Could not determine config directory"))?;

    let config_dir = proj_dirs.config_dir();

    // Ensure the config directory exists
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir)?;
    }

    Ok(config_dir.to_path_buf())
}

/// Get the path to the global config file
pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}
