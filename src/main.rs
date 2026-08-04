use std::process::ExitCode;

use clap::Parser;
use mkpage::{CommandContext, cli::Cli, logging};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let context = CommandContext::from(&cli);
    logging::init(&context);

    match mkpage::run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("mkpage: {error}");
            error.exit_code()
        }
    }
}
