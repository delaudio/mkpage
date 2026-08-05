use clap::Parser;
use mkpage::{
    CommandContext,
    cli::{Cli, Command},
    error::AppError,
};
use tempfile::TempDir;

#[test]
fn parses_each_command() {
    for (input, expected) in [
        (["mkpage", "init"].as_slice(), "init"),
        (
            ["mkpage", "init", "site", "--template", "default"].as_slice(),
            "init",
        ),
        (["mkpage", "build"].as_slice(), "build"),
        (["mkpage", "dev"].as_slice(), "dev"),
        (["mkpage", "serve"].as_slice(), "serve"),
    ] {
        let cli = Cli::try_parse_from(input).expect("command should parse");
        let actual = match cli.command {
            Command::Init(_) => "init",
            Command::Build => "build",
            Command::Dev(_) => "dev",
            Command::Serve(_) => "serve",
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
    let temp = TempDir::new().expect("temp");
    let init_target = temp.path().join("init");
    let cli = Cli::try_parse_from([
        "mkpage",
        "init",
        init_target.to_string_lossy().as_ref(),
        "--template",
        "default",
    ])
    .expect("command should parse");
    mkpage::run(cli).expect("default template should scaffold");
    assert!(init_target.join("mkpage.toml").is_file());
    assert!(init_target.join("layouts/page.html").is_file());
    assert!(init_target.join("content/index.md").is_file());

    let unknown = temp.path().join("unknown-template");
    let cli = Cli::try_parse_from([
        "mkpage",
        "init",
        unknown.to_string_lossy().as_ref(),
        "--template",
        "nonexistent",
    ])
    .expect("command should parse");
    let error = mkpage::run(cli).expect_err("unknown template should fail fast");
    assert!(error.to_string().contains("unknown template"));

    let full = init_target.join("existing.txt");
    std::fs::write(&full, b"reserved").expect("existing marker");
    let cli = Cli::try_parse_from([
        "mkpage",
        "init",
        init_target.to_string_lossy().as_ref(),
        "--template",
        "default",
    ])
    .expect("command should parse");
    let error = mkpage::run(cli).expect_err("refused init should report refusal");
    assert!(
        error
            .to_string()
            .contains("refusing to initialize in non-empty directory")
    );

    for (input, expected) in [
        (
            ["mkpage", "build"].as_slice(),
            "could not read configuration",
        ),
        (["mkpage", "dev"].as_slice(), "could not read configuration"),
        (
            ["mkpage", "serve"].as_slice(),
            "could not read configuration",
        ),
    ] {
        let cli = Cli::try_parse_from(input).expect("command should parse");
        let error = mkpage::run(cli).expect_err("handler should report its pending state");
        assert!(error.to_string().starts_with(expected));
    }
}
