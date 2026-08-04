use std::process::ExitCode;

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

/// Errors emitted by application handlers, kept independent of CLI parsing.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{command} is not implemented yet")]
    NotImplemented { command: &'static str },

    #[error("{message}")]
    Message { message: String },

    #[error("could not read source {path}: {message}")]
    SourceRead {
        path: std::path::PathBuf,
        message: String,
    },

    #[error("could not write output {path}: {message}")]
    OutputWrite {
        path: std::path::PathBuf,
        message: String,
    },

    #[error("output path escapes the fixture workspace: {path}")]
    OutputPathTraversal { path: std::path::PathBuf },

    #[error("invalid fixture {path}: {message}")]
    InvalidFixture {
        path: std::path::PathBuf,
        message: String,
    },
}

impl AppError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::NotImplemented { .. } => ExitCode::from(3),
            Self::Message { .. } => ExitCode::FAILURE,
            Self::SourceRead { .. }
            | Self::OutputWrite { .. }
            | Self::OutputPathTraversal { .. }
            | Self::InvalidFixture { .. } => ExitCode::from(4),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::NotImplemented { .. } => "E003",
            Self::Message { .. } => "E001",
            Self::SourceRead { .. } => "E101",
            Self::OutputWrite { .. } => "E201",
            Self::OutputPathTraversal { .. } => "E202",
            Self::InvalidFixture { .. } => "E301",
        }
    }
}
