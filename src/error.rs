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

    #[error("could not read configuration {path}: {message}")]
    ConfigRead {
        path: std::path::PathBuf,
        message: String,
    },
    #[error("invalid configuration {path}: {message}")]
    ConfigParse {
        path: std::path::PathBuf,
        message: String,
    },
    #[error("unsupported configuration version {found} in {path}; supported version is 1")]
    UnsupportedConfigVersion {
        path: std::path::PathBuf,
        found: u32,
    },
    #[error("unsafe output directory: {path}")]
    UnsafeOutputPath { path: std::path::PathBuf },
    #[error("source path {input} overlaps output directory {output}")]
    UnsafePathRelationship {
        input: std::path::PathBuf,
        output: std::path::PathBuf,
    },
    #[error("invalid route from {input} ({candidate}): {reason}")]
    InvalidRoute {
        input: std::path::PathBuf,
        candidate: String,
        reason: &'static str,
    },
    #[error("route collision at {route}: {first} and {second}")]
    RouteCollision {
        first: std::path::PathBuf,
        second: std::path::PathBuf,
        route: String,
    },
    #[error("static asset {asset} collides with generated page {page} at {output}")]
    StaticAssetCollision {
        page: std::path::PathBuf,
        asset: std::path::PathBuf,
        output: std::path::PathBuf,
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
            Self::ConfigRead { .. }
            | Self::ConfigParse { .. }
            | Self::UnsupportedConfigVersion { .. }
            | Self::UnsafeOutputPath { .. }
            | Self::UnsafePathRelationship { .. } => ExitCode::from(4),
            Self::InvalidRoute { .. }
            | Self::RouteCollision { .. }
            | Self::StaticAssetCollision { .. } => ExitCode::from(4),
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
            Self::ConfigRead { .. } => "E401",
            Self::ConfigParse { .. } => "E402",
            Self::UnsupportedConfigVersion { .. } => "E403",
            Self::UnsafeOutputPath { .. } => "E404",
            Self::UnsafePathRelationship { .. } => "E405",
            Self::InvalidRoute { .. } => "E501",
            Self::RouteCollision { .. } => "E502",
            Self::StaticAssetCollision { .. } => "E503",
        }
    }
}
