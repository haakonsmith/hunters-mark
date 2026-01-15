use color_eyre::eyre::eyre;

use crate::backend::Backend;
use crate::error::Result;
use crate::ui::select_from_matches;

pub fn remove(pattern: Option<&str>) -> Result<()> {
    let skim = fuzzy_matcher::skim::SkimMatcherV2::default();
    let mut backend = Backend::load()?;

    let matches = if let Some(pattern) = pattern {
        backend.close_to_best_matches(skim, &pattern, 5)?
    } else {
        let cwd = std::env::current_dir()?;

        let mut matches = backend.empty_match_all();

        // Bubble CWD to the top
        matches.sort_by_key(|e| e.path != cwd);
        matches
    };

    let selected_match = select_from_matches(&matches)?.ok_or_else(|| eyre!("Nothing selected"))?;

    backend.config.remove_mark(&selected_match.path)?;
    backend.save()?;

    println!("✓ Removed mark '{}'", selected_match.path.to_string_lossy());

    Ok(())
}
