//! Deterministic, byte-for-byte static asset copying.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use crate::error::{AppError, AppResult};

pub fn copy(
    static_root: &Path,
    output_root: &Path,
    generated: &[PathBuf],
) -> AppResult<Vec<PathBuf>> {
    if !static_root.exists() {
        return Ok(vec![]);
    }
    let mut files = Vec::new();
    visit(static_root, &mut files)?;
    files.sort();
    let mut occupied = generated
        .iter()
        .map(|path| {
            if path.is_absolute() || path.starts_with(output_root) {
                path.to_string_lossy().to_ascii_lowercase()
            } else {
                output_root
                    .join(path)
                    .to_string_lossy()
                    .to_ascii_lowercase()
            }
        })
        .collect::<BTreeSet<_>>();
    let mut copied = Vec::new();
    for source in files {
        let relative = source
            .strip_prefix(static_root)
            .expect("asset under static root");
        if relative.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        }) {
            return Err(AppError::UnsafeOutputPath { path: source });
        }
        let output = output_root.join(relative);
        let key = output.to_string_lossy().to_ascii_lowercase();
        if !output.starts_with(output_root) {
            return Err(AppError::UnsafeOutputPath { path: output });
        }
        if !occupied.insert(key) {
            return Err(AppError::StaticAssetCollision {
                page: output.clone(),
                asset: source,
                output,
            });
        }
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|error| AppError::OutputWrite {
                path: parent.to_path_buf(),
                message: error.to_string(),
            })?;
        }
        fs::copy(&source, &output).map_err(|error| AppError::OutputWrite {
            path: output.clone(),
            message: error.to_string(),
        })?;
        copied.push(relative.to_path_buf());
    }
    Ok(copied)
}
fn visit(current: &Path, files: &mut Vec<PathBuf>) -> AppResult<()> {
    for entry in fs::read_dir(current).map_err(|error| AppError::SourceRead {
        path: current.to_path_buf(),
        message: error.to_string(),
    })? {
        let path = entry
            .map_err(|error| AppError::SourceRead {
                path: current.to_path_buf(),
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
