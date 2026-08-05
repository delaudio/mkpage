//! Testable application boundary for the mkpage command-line interface.

pub mod assets;
pub mod cli;
pub mod compiler;
pub mod config;
pub mod data;
pub mod dev;
pub mod enhancements;
pub mod error;
pub mod init;
pub mod logging;
pub mod markdown;
pub mod page;
pub mod routing;
pub mod serve;
pub mod template;

use std::path::PathBuf;

use cli::Cli;
use cli::Command;
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
        Command::Init(args) => {
            let summary = init::run(context, args)?;
            for path in &summary.created {
                println!("created: {}", path.display());
            }
            for path in &summary.skipped {
                println!("skipped: {}", path.display());
            }
            Ok(())
        }
        Command::Build => build::run(context),
        Command::Dev(args) => dev::run(context, args),
        Command::Serve(args) => serve::run(context, args),
    }
}

pub mod build {
    use super::{
        CommandContext,
        compiler::{BuildRequest, build_site},
        config::{ResolveOptions, resolve},
        error::{AppError, AppResult},
        page::BuildProfile,
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
        let report = build_site(&BuildRequest {
            source_dir: project.root,
            output_dir: project.paths.output,
            profile: BuildProfile::production(time::OffsetDateTime::now_utc().date()),
            keyboard_runtime_enabled: project.config.enhancements.keyboard,
            site: project.config.site,
        })?;
        if !context.quiet {
            println!(
                "mkpage: built {} page(s), {} asset(s) in {}ms → {}",
                report.page_count,
                report.asset_count,
                report.elapsed.as_millis(),
                report.output_dir.display()
            );
        }
        Ok(())
    }
}
