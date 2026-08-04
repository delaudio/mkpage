//! Testable application boundary for the mkpage command-line interface.

pub mod cli;
pub mod compiler;
pub mod config;
pub mod error;
pub mod logging;
pub mod markdown;
pub mod page;
pub mod routing;

use std::path::PathBuf;

use cli::{Cli, Command};
use error::AppResult;

/// Shared options resolved before a command handler runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandContext {
    pub root: Option<PathBuf>,
    pub config: Option<PathBuf>,
    pub verbosity: u8,
    pub quiet: bool,
}

impl From<&Cli> for CommandContext {
    fn from(cli: &Cli) -> Self {
        Self {
            root: cli.root.clone(),
            config: cli.config.clone(),
            verbosity: cli.verbose,
            quiet: cli.quiet,
        }
    }
}

/// Routes a parsed command to its independently testable handler.
pub fn run(cli: Cli) -> AppResult<()> {
    let context = CommandContext::from(&cli);

    match cli.command {
        Command::Init => init::run(context),
        Command::Build => build::run(context),
        Command::Dev => dev::run(context),
        Command::Serve => serve::run(context),
    }
}

pub mod init {
    use super::{
        CommandContext,
        error::{AppError, AppResult},
    };

    pub fn run(_context: CommandContext) -> AppResult<()> {
        Err(AppError::NotImplemented { command: "init" })
    }
}

pub mod build {
    use super::{
        CommandContext,
        config::{ResolveOptions, resolve},
        error::{AppError, AppResult},
    };
    use std::env;

    pub fn run(context: CommandContext) -> AppResult<()> {
        let project = resolve(&ResolveOptions {
            start_dir: env::current_dir().map_err(|error| AppError::Message {
                message: error.to_string(),
            })?,
            root: context.root,
            config: context.config,
        })?;
        if context.verbosity > 0 {
            eprintln!("mkpage: project root: {}", project.root.display());
            eprintln!("mkpage: configuration: {}", project.config_path.display());
        }
        Err(AppError::NotImplemented { command: "build" })
    }
}

pub mod dev {
    use super::{
        CommandContext,
        error::{AppError, AppResult},
    };

    pub fn run(_context: CommandContext) -> AppResult<()> {
        Err(AppError::NotImplemented { command: "dev" })
    }
}

pub mod serve {
    use super::{
        CommandContext,
        error::{AppError, AppResult},
    };

    pub fn run(_context: CommandContext) -> AppResult<()> {
        Err(AppError::NotImplemented { command: "serve" })
    }
}
