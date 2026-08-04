use clap::Parser;
use mkpage::{
    CommandContext,
    cli::{Cli, Command},
    error::AppError,
};

#[test]
fn parses_each_command() {
    for (input, expected) in [
        (["mkpage", "init"].as_slice(), "init"),
        (["mkpage", "build"].as_slice(), "build"),
        (["mkpage", "dev"].as_slice(), "dev"),
        (["mkpage", "serve"].as_slice(), "serve"),
    ] {
        let cli = Cli::try_parse_from(input).expect("command should parse");
        let actual = match cli.command {
            Command::Init => "init",
            Command::Build => "build",
            Command::Dev => "dev",
            Command::Serve => "serve",
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn global_options_are_available_to_subcommands() {
    let cli = Cli::try_parse_from([
        "mkpage",
        "--root",
        "site",
        "--config",
        "mkpage.toml",
        "-vv",
        "build",
    ])
    .expect("options should parse");

    assert_eq!(
        CommandContext::from(&cli).root,
        Some(std::path::PathBuf::from("site"))
    );
    assert_eq!(
        CommandContext::from(&cli).config,
        Some(std::path::PathBuf::from("mkpage.toml"))
    );
    assert_eq!(CommandContext::from(&cli).verbosity, 2);
}

#[test]
fn quiet_and_verbose_conflict() {
    let error = Cli::try_parse_from(["mkpage", "--quiet", "--verbose", "build"])
        .expect_err("conflicting flags must fail");
    assert_eq!(error.kind(), clap::error::ErrorKind::ArgumentConflict);
}

#[test]
fn application_errors_have_stable_exit_codes() {
    assert_eq!(
        AppError::NotImplemented { command: "build" }.exit_code(),
        std::process::ExitCode::from(3)
    );
    assert_eq!(
        AppError::Message {
            message: "oops".into()
        }
        .exit_code(),
        std::process::ExitCode::FAILURE
    );
}

#[test]
fn each_command_reaches_its_own_handler() {
    for (input, expected) in [
        (["mkpage", "init"].as_slice(), "init is not implemented yet"),
        (
            ["mkpage", "build"].as_slice(),
            "could not read configuration",
        ),
        (["mkpage", "dev"].as_slice(), "dev is not implemented yet"),
        (
            ["mkpage", "serve"].as_slice(),
            "serve is not implemented yet",
        ),
    ] {
        let cli = Cli::try_parse_from(input).expect("command should parse");
        let error = mkpage::run(cli).expect_err("handler should report its pending state");
        assert!(error.to_string().starts_with(expected));
    }
}
