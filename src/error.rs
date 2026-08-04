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
}

impl AppError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::NotImplemented { .. } => ExitCode::from(3),
            Self::Message { .. } => ExitCode::FAILURE,
        }
    }
}
