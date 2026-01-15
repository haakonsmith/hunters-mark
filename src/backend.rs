use crate::config::Config;

use crate::config::global::Mark;
use crate::error::{HuntersMarkError, Result};
use directories::ProjectDirs;
use fuzzy_matcher::FuzzyMatcher;
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

pub struct Backend {
    config_path: PathBuf,
    pub config: Config,
}

impl Backend {
    pub fn load() -> Result<Self> {
        let config_path = config_file()?;
        let config = Config::load_from_path(&config_path)?;

        Ok(Backend {
            config_path,
            config,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.config.save_to_path(&self.config_path)
    }

    /// Returns a list of all matches and their respective scores
    ///
    /// Returns them sorted
    pub fn match_all(
        &mut self,
        skim: impl FuzzyMatcher,
        pattern: impl AsRef<str>,
    ) -> Result<Vec<Match>> {
        let pattern = pattern.as_ref();

        let mut matches: Vec<Match> = Vec::new();

        for (path, mark) in self.config.marks.iter_mut() {
            if let Some(score) = skim.fuzzy_match(&mark.name, pattern) {
                matches.push(Match {
                    path: path.as_path().to_path_buf(),
                    mark: mark.clone(),
                    score,
                });
            }
        }

        matches.sort_by(|a, b| b.score.cmp(&a.score));
        Ok(matches)
    }

    /// This just returns all marks with a score of zero
    pub fn empty_match_all(&mut self) -> Vec<Match> {
        let mut matches: Vec<Match> = Vec::new();

        for (path, mark) in self.config.marks.iter_mut() {
            matches.push(Match {
                path: path.as_path().to_path_buf(),
                mark: mark.clone(),
                score: 0,
            });
        }

        matches
    }

    pub fn close_to_best_matches(
        &mut self,
        skim: impl FuzzyMatcher,
        pattern: impl AsRef<str>,
        distance: i64,
    ) -> Result<Vec<Match>> {
        let pattern = pattern.as_ref();
        let matches = self.match_all(skim, pattern)?;

        let best_match = matches
            .first()
            .ok_or(HuntersMarkError::MarkNotFound(pattern.to_string()))?
            .clone();

        // matches that are only 5 away, including self
        let close_matches = matches
            .into_iter()
            .filter(|m| m.score - best_match.score <= distance)
            .collect::<Vec<_>>();

        Ok(close_matches)
    }
}

#[derive(Debug, Clone)]
pub struct Match {
    pub path: PathBuf,
    pub mark: Mark,
    pub score: i64,
}
