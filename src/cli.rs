use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

/// mkpage command-line interface.
#[derive(Debug, Parser)]
#[command(
    name = "mkpage",
    version,
    about = "A static-site generator for terminal-minded websites"
)]
pub struct Cli {
    /// Project root to use instead of the current directory.
    #[arg(long, global = true)]
    pub root: Option<PathBuf>,

    /// Explicit configuration file path.
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Increase diagnostic output. Repeat for more detail.
    #[arg(short, long, global = true, action = ArgAction::Count, conflicts_with = "quiet")]
    pub verbose: u8,

    /// Suppress non-error output.
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new mkpage project.
    Init,
    /// Build a static site.
    Build,
    /// Watch source files and serve development output.
    Dev,
    /// Serve an existing generated site locally.
    Serve,
}
