use crate::error::{HuntersMarkError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub marks: HashMap<PathBuf, Mark>,
    #[serde(default)]
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mark {
    // pub path: PathBuf,
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_accessed: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_true")]
    pub run_init_scripts: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            run_init_scripts: true,
        }
    }
}

fn default_true() -> bool {
    true
}

impl Config {
    /// Load the config from the default location
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Save the config to the default location
    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }

    /// Add a new mark
    pub fn add_mark(&mut self, name: String, path: PathBuf, tags: Vec<String>) -> Result<()> {
        // Validate mark name
        if !is_valid_mark_name(&name) {
            return Err(HuntersMarkError::InvalidMarkName(
                name,
                "Mark names must contain only alphanumeric characters, hyphens, and underscores"
                    .to_string(),
            )
            .into());
        }

        // Check if mark already exists
        if let Entry::Occupied(existing) = self.marks.entry(path.clone()) {
            return Err(HuntersMarkError::MarkAlreadyExists(name, existing.key().clone()).into());
        }

        // Verify directory exists
        if !path.exists() {
            return Err(HuntersMarkError::DirectoryNotFound(path).into());
        }

        let now = Utc::now();
        let mark = Mark {
            name,
            tags,
            last_accessed: now,
            created_at: now,
        };

        self.marks.insert(path, mark);
        Ok(())
    }

    /// Remove a mark
    pub fn remove_mark(&mut self, path: &Path) -> Result<()> {
        self.marks
            .remove(path)
            .ok_or_else(|| HuntersMarkError::MarkNotFound(path.display().to_string()))?;
        Ok(())
    }
}

/// Validate that a mark name only contains alphanumeric characters, hyphens, and underscores
fn is_valid_mark_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mark_names() {
        assert!(is_valid_mark_name("myproject"));
        assert!(is_valid_mark_name("my-project"));
        assert!(is_valid_mark_name("my_project"));
        assert!(is_valid_mark_name("project123"));
        assert!(is_valid_mark_name("123project"));
    }

    #[test]
    fn test_invalid_mark_names() {
        assert!(!is_valid_mark_name(""));
        assert!(!is_valid_mark_name("my project"));
        assert!(!is_valid_mark_name("my.project"));
        assert!(!is_valid_mark_name("my/project"));
        assert!(!is_valid_mark_name("my@project"));
    }
}
