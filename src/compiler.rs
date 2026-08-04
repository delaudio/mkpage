//! Minimal deterministic build entry point used by the fixture harness.

use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use crate::error::{AppError, AppResult};

/// Inputs for one static-site build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildRequest {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
}

/// Files emitted by a successful build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildReport {
    pub generated_files: Vec<PathBuf>,
}

/// Builds the minimal reference fixture into a deterministic HTML document.
///
/// This is intentionally narrow: routing, frontmatter, Markdown rendering,
/// templates, and assets are introduced by their dedicated capabilities.
pub fn build_site(request: &BuildRequest) -> AppResult<BuildReport> {
    validate_output_path(&request.source_dir, &request.output_dir)?;

    let source = request.source_dir.join("content").join("index.md");
    let content = fs::read_to_string(&source).map_err(|error| AppError::SourceRead {
        path: source.clone(),
        message: error.to_string(),
    })?;

    if content.starts_with("!invalid!") {
        return Err(AppError::InvalidFixture {
            path: source,
            message: "fixture starts with the reserved !invalid! marker".into(),
        });
    }

    fs::create_dir_all(&request.output_dir).map_err(|error| AppError::OutputWrite {
        path: request.output_dir.clone(),
        message: error.to_string(),
    })?;

    let output = request.output_dir.join("index.html");
    fs::write(&output, render_reference_page(&content)).map_err(|error| AppError::OutputWrite {
        path: output.clone(),
        message: error.to_string(),
    })?;

    Ok(BuildReport {
        generated_files: vec![PathBuf::from("index.html")],
    })
}

fn validate_output_path(source_dir: &Path, output_dir: &Path) -> AppResult<()> {
    if output_dir
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(AppError::OutputPathTraversal {
            path: output_dir.to_path_buf(),
        });
    }

    let source_root = source_dir
        .canonicalize()
        .map_err(|error| AppError::SourceRead {
            path: source_dir.to_path_buf(),
            message: error.to_string(),
        })?;
    let candidate = output_dir
        .parent()
        .unwrap_or(output_dir)
        .canonicalize()
        .unwrap_or_else(|_| output_dir.to_path_buf());

    if candidate.starts_with(&source_root) {
        return Err(AppError::OutputPathTraversal {
            path: output_dir.to_path_buf(),
        });
    }

    Ok(())
}

fn render_reference_page(content: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>mkpage fixture</title>\n</head>\n<body>\n<main>\n<pre>{}</pre>\n</main>\n</body>\n</html>\n",
        escape_html(content.trim_end())
    )
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
