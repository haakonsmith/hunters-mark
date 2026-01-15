use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum HuntersMarkError {
    #[error("Mark for pattern '{0}' not found")]
    MarkNotFound(String),

    #[error("Mark '{0}' already exists at {1}")]
    MarkAlreadyExists(String, PathBuf),

    #[error("Invalid mark name '{0}': {1}")]
    InvalidMarkName(String, String),

    #[error("Directory does not exist: {0}")]
    DirectoryNotFound(PathBuf),
}

pub type Result<T> = color_eyre::Result<T>;
