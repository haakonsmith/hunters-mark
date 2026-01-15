use crate::error::{HuntersMarkError, Result};
use chrono::{DateTime, Utc};
use fuzzy_matcher::FuzzyMatcher;
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
    pub fn load() -> Result<Self> {
        let config_path = super::paths::config_file()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Save the config to the default location
    pub fn save(&self) -> Result<()> {
        let config_path = super::paths::config_file()?;
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;
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

    /// Returns a list of all matches and their respective scores
    ///
    /// Returns them sorted
    pub fn match_all(
        &mut self,
        skim: impl FuzzyMatcher,
        pattern: impl AsRef<str>,
    ) -> Result<Vec<(&Path, &Mark, i64)>> {
        let pattern = pattern.as_ref();

        let mut matches: Vec<(&Path, &Mark, i64)> = Vec::new();

        for (path, mark) in self.marks.iter_mut() {
            if let Some(score) = skim.fuzzy_match(&mark.name, pattern) {
                matches.push((path.as_path(), mark, score));
            }
        }

        matches.sort_by(|a, b| b.2.cmp(&a.2));
        Ok(matches)
    }

    pub fn match_mark_mut(
        &mut self,
        skim: impl FuzzyMatcher,
        pattern: impl AsRef<str>,
    ) -> std::result::Result<(&PathBuf, &mut Mark), HuntersMarkError> {
        let pattern = pattern.as_ref();

        let mut best_match = None;
        let mut best_score = 0;

        for (path, mark) in self.marks.iter_mut() {
            if let Some(score) = skim.fuzzy_match(&mark.name, pattern) {
                if score > best_score {
                    best_match = Some((path, mark));
                    best_score = score;
                }
            }
        }

        best_match.ok_or_else(|| HuntersMarkError::MarkNotFound(pattern.to_string()))
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
