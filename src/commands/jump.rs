use crate::config::Config;
use crate::error::{HuntersMarkError, Result};
use chrono::Utc;

pub fn path(pattern: String) -> Result<()> {
    let skim = fuzzy_matcher::skim::SkimMatcherV2::default();
    let mut config = Config::load()?;

    let (path, mark) = config.match_mark_mut(skim, &pattern)?;

    // First, get a reference to check if mark exists and get the path
    // Check if directory still exists
    if !path.exists() {
        eprintln!("⚠  Warning: Directory no longer exists: {}", path.display());
        eprintln!("Consider removing this mark: hunters-mark remove {pattern}");
        return Err(HuntersMarkError::DirectoryNotFound(path.clone()).into());
    }

    // Update last accessed timestamp
    mark.last_accessed = Utc::now();

    // Print path for shell wrapper to use
    println!("{}", path.display());

    Ok(())
}
