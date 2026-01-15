use crate::backend::Backend;

use crate::error::{HuntersMarkError, Result};
use crate::ui::select_from_matches;
use chrono::Utc;

pub fn path(pattern: String) -> Result<()> {
    let skim = fuzzy_matcher::skim::SkimMatcherV2::default();
    let mut backend = Backend::load()?;

    let matches = backend.close_to_best_matches(skim, &pattern, 5)?;

    let selected_match = select_from_matches(&matches)?
        .ok_or_else(|| HuntersMarkError::MarkNotFound(pattern.clone()))?;

    let selected_path = selected_match.path.clone();

    // Check if directory still exists
    if !selected_path.exists() {
        eprintln!(
            "⚠  Warning: Directory no longer exists: {}",
            selected_path.display()
        );
        eprintln!("Consider removing this mark: hunters-mark remove {pattern}");
        return Err(HuntersMarkError::DirectoryNotFound(selected_path).into());
    }

    // Update last accessed timestamp
    if let Some(mark) = backend.config.marks.get_mut(&selected_path) {
        mark.last_accessed = Utc::now();
        backend.save()?;
    }

    // Print path for shell wrapper to use
    println!("{}", selected_path.display());

    Ok(())
}
