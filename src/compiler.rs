//! Minimal deterministic build entry point used by the fixture harness.

use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
    time::Instant,
};

use crate::{
    error::{AppError, AppResult},
    markdown::render,
    page::{BuildProfile, parse},
    template::render as render_template,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

const MANIFEST_FILE: &str = ".mkpage-manifest.json";
const MANIFEST_VERSION: u32 = 1;

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
    pub page_count: usize,
    pub asset_count: usize,
    pub output_dir: PathBuf,
    pub elapsed: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct OutputManifest {
    version: u32,
    owner: &'static str,
    files: Vec<ManifestEntry>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ManifestEntry {
    path: PathBuf,
    kind: &'static str,
    size: usize,
    sha256: String,
}

/// Builds the minimal reference fixture into a deterministic HTML document.
///
/// This is intentionally narrow: routing, frontmatter, Markdown rendering,
/// templates, and assets are introduced by their dedicated capabilities.
pub fn build_site(request: &BuildRequest) -> AppResult<BuildReport> {
    let started = Instant::now();
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
            page_count: 0,
            asset_count: 0,
            output_dir: request.output_dir.clone(),
            elapsed: started.elapsed(),
        });
    }

    let asset_root = request.source_dir.join("static");
    let asset_sources = collect_assets(&asset_root)?;
    let mut occupied = BTreeSet::from(["index.html".to_owned()]);
    let mut planned_assets = Vec::new();
    for source in &asset_sources {
        let relative = source
            .strip_prefix(&asset_root)
            .expect("asset under root")
            .to_path_buf();
        let key = relative.to_string_lossy().to_ascii_lowercase();
        if !occupied.insert(key) {
            return Err(AppError::StaticAssetCollision {
                page: request.output_dir.join(&relative),
                asset: source.clone(),
                output: request.output_dir.join(relative),
            });
        }
        let bytes = fs::read(source).map_err(|error| AppError::SourceRead {
            path: source.clone(),
            message: error.to_string(),
        })?;
        planned_assets.push((relative, bytes));
    }

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
    let document_bytes = document.into_bytes();
    fs::create_dir_all(&request.output_dir).map_err(|error| AppError::OutputWrite {
        path: request.output_dir.clone(),
        message: error.to_string(),
    })?;
    fs::write(&output, &document_bytes).map_err(|error| AppError::OutputWrite {
        path: output.clone(),
        message: error.to_string(),
    })?;

    let mut generated_files = vec![PathBuf::from("index.html")];
    for (relative, bytes) in planned_assets {
        let destination = request.output_dir.join(&relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| AppError::OutputWrite {
                path: parent.to_path_buf(),
                message: error.to_string(),
            })?;
        }
        fs::write(&destination, bytes).map_err(|error| AppError::OutputWrite {
            path: destination,
            message: error.to_string(),
        })?;
        generated_files.push(relative);
    }
    generated_files.sort();
    remove_stale_managed(&request.output_dir, &generated_files)?;
    let mut manifest_files = Vec::new();
    for path in &generated_files {
        let target = request.output_dir.join(path);
        let size = fs::metadata(&target)
            .map_err(|error| AppError::OutputWrite {
                path: target,
                message: error.to_string(),
            })?
            .len() as usize;
        manifest_files.push(ManifestEntry {
            path: path.clone(),
            kind: if path
                .extension()
                .is_some_and(|extension| extension == "html")
            {
                "page"
            } else {
                "asset"
            },
            size,
            sha256: sha256(&fs::read(request.output_dir.join(path)).map_err(|error| {
                AppError::OutputWrite {
                    path: request.output_dir.join(path),
                    message: error.to_string(),
                }
            })?),
        });
    }
    let manifest = OutputManifest {
        version: MANIFEST_VERSION,
        owner: "mkpage",
        files: manifest_files,
    };
    let mut manifest_bytes = serde_json::to_vec_pretty(&manifest).expect("manifest serialization");
    manifest_bytes.push(b'\n');
    fs::write(request.output_dir.join(MANIFEST_FILE), manifest_bytes).map_err(|error| {
        AppError::OutputWrite {
            path: request.output_dir.join(MANIFEST_FILE),
            message: error.to_string(),
        }
    })?;
    let asset_count = asset_sources.len();
    Ok(BuildReport {
        generated_files,
        page_count: 1,
        asset_count,
        output_dir: request.output_dir.clone(),
        elapsed: started.elapsed(),
    })
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn collect_assets(root: &Path) -> AppResult<Vec<PathBuf>> {
    if !root.exists() {
        return Ok(vec![]);
    }
    let mut files = Vec::new();
    fn visit(path: &Path, files: &mut Vec<PathBuf>) -> AppResult<()> {
        for entry in fs::read_dir(path).map_err(|error| AppError::SourceRead {
            path: path.to_path_buf(),
            message: error.to_string(),
        })? {
            let path = entry
                .map_err(|error| AppError::SourceRead {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                })?
                .path();
            if path.is_symlink() {
                continue;
            }
            if path.is_dir() {
                visit(&path, files)?
            } else {
                files.push(path)
            }
        }
        Ok(())
    }
    visit(root, &mut files)?;
    files.sort();
    Ok(files)
}
fn remove_stale_managed(output: &Path, current: &[PathBuf]) -> AppResult<()> {
    let manifest_path = output.join(MANIFEST_FILE);
    let Ok(text) = fs::read_to_string(&manifest_path) else {
        return Ok(());
    };
    let Ok(previous) = serde_json::from_str::<serde_json::Value>(&text) else {
        return Ok(());
    };
    let active = current.iter().collect::<BTreeSet<_>>();
    if let Some(files) = previous.get("files").and_then(serde_json::Value::as_array) {
        for entry in files {
            if let Some(path) = entry
                .get("path")
                .and_then(serde_json::Value::as_str)
                .map(PathBuf::from)
            {
                if !active.contains(&path) {
                    let target = output.join(&path);
                    if target.starts_with(output) && target.is_file() {
                        fs::remove_file(target).map_err(|error| AppError::OutputWrite {
                            path: output.join(path),
                            message: error.to_string(),
                        })?;
                    }
                }
            }
        }
    }
    Ok(())
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
