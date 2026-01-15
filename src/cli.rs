use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "Hunter's Mark - Quick directory navigation", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Mark name - when no subcommand is provided, jump to this mark
    pub mark_name: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new mark for a directory
    #[command(alias = "mark")]
    Add {
        /// Name for the mark
        ///
        /// If this is none then it will attempt to infer the correct name from a project/folder name
        #[arg(short, long)]
        name: Option<String>,

        /// Path to mark (defaults to current directory)
        path: Option<PathBuf>,

        /// Tags to associate with the mark
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// List all marks
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Sort by last accessed time
        #[arg(short, long)]
        recent: bool,
    },

    /// Remove a mark
    Remove {
        /// Name of the mark to remove
        name: String,
    },

    /// Initialize shell integration
    Init {
        /// Shell type (bash, zsh, fish)
        shell: Shell,

        /// Prefix for the hunters-mark command
        #[arg(short, long, default_value = "hm")]
        prefix: String,
    },

    /// Generate shell completions
    Completions {
        /// Shell type (bash, zsh, fish, etc.)
        shell: Shell,
    },

    /// Show path for a mark (used internally by shell wrapper)
    // #[command(hide = true)]
    Path {
        /// Mark name will be fuzzy matched
        name: String,
    },
}

impl Cli {
    pub fn command() -> clap::Command {
        <Self as CommandFactory>::command()
    }
}
