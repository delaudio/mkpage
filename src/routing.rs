//! Deterministic source discovery and safe clean-URL calculation.

use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Route(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputPath(PathBuf);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRoute {
    pub source: PathBuf,
    pub route: Route,
    pub output: OutputPath,
}

impl Route {
    pub fn from_source(source_root: &Path, source: &Path) -> AppResult<Self> {
        let relative = source
            .strip_prefix(source_root)
            .map_err(|_| AppError::InvalidRoute {
                input: source.to_path_buf(),
                candidate: source.display().to_string(),
                reason: "source is outside the configured source root",
            })?;
        validate_relative(relative, source)?;
        let without_extension = relative.with_extension("");
        let segments = without_extension
            .components()
            .filter_map(|component| match component {
                Component::Normal(value) => Some(value.to_string_lossy()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let segments = if segments.last().is_some_and(|segment| segment == "index") {
            &segments[..segments.len() - 1]
        } else {
            &segments[..]
        };
        let path = if segments.is_empty() {
            "/".to_owned()
        } else {
            format!("/{}/", segments.join("/"))
        };
        Ok(Self(path))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn output_path(&self, output_root: &Path) -> AppResult<OutputPath> {
        let relative = if self.0 == "/" {
            PathBuf::from("index.html")
        } else {
            PathBuf::from(self.0.trim_matches('/')).join("index.html")
        };
        let output = output_root.join(relative);
        if !output.starts_with(output_root) {
            return Err(AppError::UnsafeOutputPath { path: output });
        }
        Ok(OutputPath(output))
    }
}
impl OutputPath {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

pub fn discover(source_root: &Path, output_root: &Path) -> AppResult<Vec<SourceRoute>> {
    let mut files = Vec::new();
    visit(source_root, &mut files)?;
    files.sort();
    let mut discovered = Vec::new();
    for source in files {
        let route = Route::from_source(source_root, &source)?;
        let output = route.output_path(output_root)?;
        discovered.push(SourceRoute {
            source,
            route,
            output,
        });
    }
    validate_collisions(&discovered)?;
    Ok(discovered)
}

/// Reject exact and case-only routes independently of the host filesystem.
pub fn validate_collisions(candidates: &[SourceRoute]) -> AppResult<()> {
    let mut routes = BTreeMap::<String, PathBuf>::new();
    let mut case_routes = BTreeMap::<String, PathBuf>::new();
    for candidate in candidates {
        let key = candidate.route.as_str().to_owned();
        if let Some(owner) = routes.insert(key.clone(), candidate.source.clone()) {
            return Err(AppError::RouteCollision {
                first: owner,
                second: candidate.source.clone(),
                route: key,
            });
        }
        if let Some(owner) = case_routes.insert(key.to_lowercase(), candidate.source.clone()) {
            return Err(AppError::RouteCollision {
                first: owner,
                second: candidate.source.clone(),
                route: key,
            });
        }
    }
    Ok(())
}

/// Reject static files that would overwrite generated page output.
pub fn validate_static_collisions(
    candidates: &[SourceRoute],
    static_root: &Path,
    output_root: &Path,
) -> AppResult<()> {
    let mut static_files = Vec::new();
    visit(static_root, &mut static_files)?;
    static_files.sort();
    let generated = candidates
        .iter()
        .map(|candidate| {
            (
                candidate.output.as_path().to_string_lossy().to_lowercase(),
                candidate.source.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for asset in static_files {
        let relative = asset
            .strip_prefix(static_root)
            .map_err(|_| AppError::InvalidRoute {
                input: asset.clone(),
                candidate: asset.display().to_string(),
                reason: "static asset is outside the configured static root",
            })?;
        validate_relative(relative, &asset)?;
        let output = output_root.join(relative);
        if !output.starts_with(output_root) {
            return Err(AppError::UnsafeOutputPath { path: output });
        }
        if let Some(page) = generated.get(&output.to_string_lossy().to_lowercase()) {
            return Err(AppError::StaticAssetCollision {
                page: page.clone(),
                asset,
                output,
            });
        }
    }
    Ok(())
}

fn visit(current: &Path, files: &mut Vec<PathBuf>) -> AppResult<()> {
    for entry in fs::read_dir(current).map_err(|error| AppError::SourceRead {
        path: current.to_path_buf(),
        message: error.to_string(),
    })? {
        let entry = entry.map_err(|error| AppError::SourceRead {
            path: current.to_path_buf(),
            message: error.to_string(),
        })?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || name.starts_with('_') || path.is_symlink() {
            continue;
        }
        if path.is_dir() {
            visit(&path, files)?;
        } else if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("md") | Some("html")
        ) {
            files.push(path);
        }
    }
    Ok(())
}

fn validate_relative(relative: &Path, source: &Path) -> AppResult<()> {
    if relative.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(AppError::InvalidRoute {
            input: source.to_path_buf(),
            candidate: relative.display().to_string(),
            reason: "route contains an unsafe path component",
        });
    }
    for component in relative.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        let value = value.to_string_lossy();
        let lowercase = value.to_ascii_lowercase();
        if value.contains(['\\', ':'])
            || lowercase.contains("%2e")
            || lowercase.contains("%2f")
            || lowercase.contains("%5c")
            || lowercase.contains("%00")
        {
            return Err(AppError::InvalidRoute {
                input: source.to_path_buf(),
                candidate: relative.display().to_string(),
                reason: "route contains an unsafe path or URL-encoded component",
            });
        }
    }
    Ok(())
}
