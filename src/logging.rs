use tracing_subscriber::{EnvFilter, fmt};

use crate::CommandContext;

/// Initializes process-wide logging after CLI parsing.
pub fn init(context: &CommandContext) {
    if context.quiet {
        return;
    }

    let fallback = match context.verbosity {
        0 => "warn",
        1 => "info",
        _ => "debug",
    };

    let _ = fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| fallback.into()))
        .with_target(false)
        .try_init();
}
