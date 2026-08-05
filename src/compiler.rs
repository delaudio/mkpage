//! Deterministic static-site build entry point.

use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
    time::Instant,
};

use crate::{
    assets, enhancements,
    error::{AppError, AppResult},
    markdown::render,
    page::{BuildProfile, parse},
    routing::discover,
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
    pub keyboard_runtime_enabled: bool,
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

/// Build all markdown content pages into deterministic output files.
pub fn build_site(request: &BuildRequest) -> AppResult<BuildReport> {
    let started = Instant::now();
    validate_output_path(&request.source_dir, &request.output_dir)?;

    let source_root = request.source_dir.join("content");
    let candidates = discover(&source_root, &request.output_dir)?;
    let layouts_root = request.source_dir.join("layouts");

    let mut planned_pages = Vec::new();
    for candidate in candidates {
        if candidate
            .source
            .extension()
            .and_then(|value| value.to_str())
            != Some("md")
        {
            continue;
        }

        let source = candidate.source;
        let bytes = fs::read(&source).map_err(|error| AppError::SourceRead {
            path: source.clone(),
            message: error.to_string(),
        })?;

        if bytes.starts_with(b"!invalid!") {
            return Err(AppError::InvalidFixture {
                path: source,
                message: "fixture starts with the reserved !invalid! marker".into(),
            });
        }

        let page = parse(&source, &bytes)?;
        if !request.profile.includes(&page) {
            continue;
        }

        let rendered = render(&page.body);
        let document = match page.metadata.layout.as_deref() {
            Some(layout) => render_template(
                &layouts_root,
                layout,
                &page,
                &rendered,
                request.keyboard_runtime_enabled,
            )?,
            None => {
                render_reference_page(&rendered.html, request.profile.shows_draft_marker(&page))
            }
        };

        let relative_output = candidate
            .output
            .as_path()
            .strip_prefix(&request.output_dir)
            .map_err(|error| AppError::OutputWrite {
                path: request.output_dir.clone(),
                message: error.to_string(),
            })?
            .to_path_buf();
        planned_pages.push((relative_output, document.into_bytes()));
    }

    let asset_root = request.source_dir.join("static");
    let mut generated_files = planned_pages
        .iter()
        .map(|(path, _)| path.clone())
        .collect::<Vec<_>>();

    let mut runtime = None;
    if request.keyboard_runtime_enabled {
        let runtime_path = PathBuf::from(enhancements::KEYBOARD_RUNTIME_PATH);
        let generated_path = runtime_path.to_string_lossy().to_ascii_lowercase();
        if generated_files
            .iter()
            .map(|path| path.to_string_lossy().to_ascii_lowercase())
            .any(|path| path == generated_path)
        {
            return Err(AppError::StaticAssetCollision {
                page: request.output_dir.join(&runtime_path),
                asset: runtime_path.clone(),
                output: request.output_dir.join(runtime_path),
            });
        }
        generated_files.push(runtime_path.clone());
        runtime = Some((runtime_path, enhancements::runtime().as_bytes().to_vec()));
    }

    let copied_assets = assets::copy(&asset_root, &request.output_dir, &generated_files)?;
    generated_files.extend(copied_assets);

    fs::create_dir_all(&request.output_dir).map_err(|error| AppError::OutputWrite {
        path: request.output_dir.clone(),
        message: error.to_string(),
    })?;

    for (relative, bytes) in planned_pages {
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
    }

    if let Some((path, bytes)) = runtime {
        let destination = request.output_dir.join(&path);
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
    }

    generated_files.sort();
    generated_files.dedup();
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

    let page_count = generated_files
        .iter()
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "html")
        })
        .count();
    let asset_count = generated_files
        .iter()
        .filter(|path| path.extension().is_none_or(|extension| extension != "html"))
        .count()
        .saturating_sub(usize::from(request.keyboard_runtime_enabled));

    Ok(BuildReport {
        generated_files,
        page_count,
        asset_count,
        output_dir: request.output_dir.clone(),
        elapsed: started.elapsed(),
    })
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
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
    let _ = source_dir
        .canonicalize()
        .map_err(|error| AppError::SourceRead {
            path: source_dir.to_path_buf(),
            message: error.to_string(),
        })?;
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
