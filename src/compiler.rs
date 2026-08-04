//! Minimal deterministic build entry point used by the fixture harness.

use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use crate::{
    assets,
    error::{AppError, AppResult},
    markdown::render,
    page::{BuildProfile, parse},
    template::render as render_template,
};

/// Inputs for one static-site build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildRequest {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub profile: BuildProfile,
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
    let content = fs::read(&source).map_err(|error| AppError::SourceRead {
        path: source.clone(),
        message: error.to_string(),
    })?;

    if content.starts_with(b"!invalid!") {
        return Err(AppError::InvalidFixture {
            path: source,
            message: "fixture starts with the reserved !invalid! marker".into(),
        });
    }
    let page = parse(&source, &content)?;
    if !request.profile.includes(&page) {
        return Ok(BuildReport {
            generated_files: vec![],
        });
    }

    fs::create_dir_all(&request.output_dir).map_err(|error| AppError::OutputWrite {
        path: request.output_dir.clone(),
        message: error.to_string(),
    })?;

    let output = request.output_dir.join("index.html");
    let rendered = render(&page.body);
    let document = match page.metadata.layout.as_deref() {
        Some(layout) => render_template(
            &request.source_dir.join("layouts"),
            layout,
            &page,
            &rendered,
        )?,
        None => render_reference_page(&rendered.html, request.profile.shows_draft_marker(&page)),
    };
    fs::write(&output, document).map_err(|error| AppError::OutputWrite {
        path: output.clone(),
        message: error.to_string(),
    })?;

    let mut generated_files = vec![PathBuf::from("index.html")];
    generated_files.extend(assets::copy(
        &request.source_dir.join("static"),
        &request.output_dir,
        std::slice::from_ref(&output),
    )?);
    generated_files.sort();
    Ok(BuildReport { generated_files })
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

fn render_reference_page(content: &str, draft_marker: bool) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>mkpage fixture</title>\n</head>\n<body>\n<main>{}\n{}\n</main>\n</body>\n</html>\n",
        if draft_marker {
            "\n<aside data-mkpage-draft=\"true\">DRAFT</aside>"
        } else {
            ""
        },
        content.trim_end()
    )
}
