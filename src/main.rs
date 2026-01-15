mod cli;
mod commands;
mod config;
mod error;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use color_eyre::config::HookBuilder;

fn main() -> color_eyre::Result<()> {
    // Install color_eyre for better error reporting
    HookBuilder::default()
        .display_env_section(false)
        .capture_span_trace_by_default(false)
        .install()?;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { name, path, tags }) => commands::add(name, path, tags)?,
        Some(Commands::List { tag, recent }) => commands::list(tag, recent)?,
        Some(Commands::Remove { name }) => commands::remove(name)?,
        Some(Commands::Init { shell, prefix }) => commands::init(shell, prefix)?,
        Some(Commands::Completions { shell }) => commands::completions(shell)?,
        Some(Commands::Path { name }) => commands::path(name)?,
        None => {
            // If no subcommand but a mark name is provided, jump to it
            if let Some(mark_name) = cli.mark_name {
                commands::path(mark_name)?;
            } else {
                // No command and no mark name, show help
                Cli::command().print_help()?;
            }
        }
    }

    Ok(())
}
