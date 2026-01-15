use std::path::Path;

use crate::config::Config;
use crate::error::Result;

pub fn remove(pattern: impl AsRef<str>) -> Result<()> {
    let skim = fuzzy_matcher::skim::SkimMatcherV2::default();
    let mut config = Config::load()?;

    let (path, _) = config.match_mark_mut(skim, pattern)?;
    let path = path.clone();

    config.remove_mark(&path)?;
    config.save()?;

    println!("✓ Removed mark '{}'", path.to_string_lossy());

    Ok(())
}
