use crate::{backend::Match, error::Result};
use dialoguer::{
    Select,
    console::{Style, Term},
    theme::ColorfulTheme,
};
use std::fs::OpenOptions;

/// Displays an interactive selection dialog for choosing from multiple matches.
///
/// Returns the index of the selected item, or None if the user cancelled.
pub fn select_generic<T>(items: &[T], formatter: impl Fn(&T) -> String) -> Result<Option<usize>> {
    // Format all items for display
    let formatted_items: Vec<String> = items.iter().map(|item| formatter(item)).collect();

    // Open /dev/tty directly to avoid interfering with stdout
    let tty_file = OpenOptions::new().read(true).write(true).open("/dev/tty")?;

    // Wrap in a Term for dialoguer
    let term = Term::read_write_pair_with_style(
        tty_file.try_clone()?,
        tty_file,
        Style::new().for_stdout(),
    );

    // Create a Select prompt with a nice theme
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple close matches found - select one")
        .items(&formatted_items)
        .default(0)
        .interact_on_opt(&term)?;

    Ok(selection)
}

pub fn select_from_matches(matches: &[Match]) -> Result<Option<&Match>> {
    // Select the match to use (either prompt user or use best match)
    let selected_match = if matches.len() > 1 {
        let selected_idx = select_generic(&matches, |m| {
            format!(
                "{} → {} Accessed: {}",
                m.mark.name,
                m.path.display(),
                m.mark.last_accessed
            )
        })?;

        match selected_idx {
            Some(idx) => &matches[idx],
            None => return Ok(None),
        }
    } else {
        &matches[0]
    };

    Ok(Some(selected_match))
}
