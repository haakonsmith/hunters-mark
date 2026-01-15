use crate::error::Result;
use dialoguer::{Select, console::Term, theme::ColorfulTheme};
use std::fs::OpenOptions;

/// Displays an interactive selection dialog for choosing from multiple matches.
///
/// Returns the index of the selected item, or None if the user cancelled.
pub fn select_from_matches<T>(
    items: &[T],
    formatter: impl Fn(&T) -> String,
) -> Result<Option<usize>> {
    // Format all items for display
    let formatted_items: Vec<String> = items.iter().map(|item| formatter(item)).collect();

    // Open /dev/tty directly to avoid interfering with stdout
    let tty_file = OpenOptions::new().read(true).write(true).open("/dev/tty")?;

    // Wrap in a Term for dialoguer
    let term = Term::read_write_pair(tty_file.try_clone()?, tty_file);

    // Create a Select prompt with a nice theme
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple close matches found - select one")
        .items(&formatted_items)
        .default(0)
        .interact_on_opt(&term)?;

    Ok(selection)
}
