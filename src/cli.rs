use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand};

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
    Init(Init),
    /// Build a static site.
    Build,
    /// Watch source files and serve development output.
    Dev,
    /// Serve an existing generated site locally.
    Serve,
}

#[derive(Debug, Args)]
pub struct Init {
    /// Target directory. Defaults to current working directory.
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub directory: std::path::PathBuf,

    /// Starter template to install.
    #[arg(short, long, default_value = "default")]
    pub template: String,
}
