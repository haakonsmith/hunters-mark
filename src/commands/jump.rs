use crate::config::Config;
use crate::error::{HuntersMarkError, Result};
use crate::ui::select_from_matches;
use chrono::Utc;

pub fn path(pattern: String) -> Result<()> {
    let skim = fuzzy_matcher::skim::SkimMatcherV2::default();
    let mut config = Config::load()?;

    let matches = config.match_all(skim, &pattern)?;

    let best_match = matches
        .first()
        .ok_or(HuntersMarkError::MarkNotFound(pattern.clone()))?;

    // matches that are only 5 away
    let close_matches = matches
        .iter()
        .filter(|m| m.2 - best_match.2 <= 5)
        .collect::<Vec<_>>();

    // Select the match to use (either prompt user or use best match)
    let selected_match = if close_matches.len() > 1 {
        let selected_idx = select_from_matches(&close_matches, |m| {
            format!("{} → {}", m.1.name, m.0.display())
        })?;

        match selected_idx {
            Some(idx) => close_matches[idx],
            None => {
                eprintln!("Selection cancelled");
                return Ok(());
            }
        }
    } else {
        best_match
    };

    let selected_path = selected_match.0.to_path_buf();

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
    if let Some(mark) = config.marks.get_mut(&selected_path) {
        mark.last_accessed = Utc::now();
        config.save()?;
    }

    // Print path for shell wrapper to use
    println!("{}", selected_path.display());

    Ok(())
}
