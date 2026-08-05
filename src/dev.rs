//! Development command with watching and auto-rebuild loop.

use std::{
    fs::{self, File},
    io::Read,
    net::TcpListener,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use sha2::{Digest, Sha256};

use crate::{
    CommandContext,
    cli::Dev,
    compiler::{BuildRequest, build_site},
    config::{ResolveOptions, ResolvedPaths, ResolvedProject, resolve},
    error::{AppError, AppResult},
    page::BuildProfile,
    serve::serve_from_listener,
};

/// Dispatches `mkpage dev` with watch and serve behavior.
pub fn run(context: CommandContext, args: Dev) -> AppResult<()> {
    let interval = Duration::from_millis(args.interval_ms.max(250));
    let mut project = resolve(&ResolveOptions {
        start_dir: std::env::current_dir().map_err(|error| AppError::Message {
            message: error.to_string(),
        })?,
        root: context.root.clone(),
        config: context.config.clone(),
    })?;

    if !context.quiet {
        println!("mkpage: built starting project {}", project.root.display());
    }

    build_once(&project, &context)?;

    let address = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&address).map_err(|error| AppError::Message {
        message: format!("could not bind {address}: {error}"),
    })?;

    if !context.quiet {
        println!(
            "mkpage: serving {} at http://{address}",
            project.paths.output.display()
        );
    }

    let output_dir = project.paths.output.clone();
    let quiet = context.quiet;
    let server = thread::Builder::new()
        .name("mkpage-dev-server".into())
        .spawn(move || {
            if let Err(error) = serve_from_listener(listener, output_dir, !quiet) {
                eprintln!("mkpage: dev server stopped: {error}");
            }
        });
    server.map_err(|error| AppError::Message {
        message: error.to_string(),
    })?;

    let mut snapshot = tracked_fingerprint(&project.paths, &project.config_path)?;
    loop {
        thread::sleep(interval);

        let current = match resolve(&ResolveOptions {
            start_dir: std::env::current_dir().map_err(|error| AppError::Message {
                message: error.to_string(),
            })?,
            root: project.root.clone().into(),
            config: Some(project.config_path.clone()),
        }) {
            Ok(next) => next,
            Err(error) => {
                eprintln!("mkpage: build context error: {error}");
                continue;
            }
        };

        let next_snapshot = match tracked_fingerprint(&current.paths, &current.config_path) {
            Ok(next) => next,
            Err(error) => {
                eprintln!("mkpage: file fingerprint error: {error}");
                continue;
            }
        };

        if next_snapshot != snapshot {
            if let Err(error) = build_once(&current, &context) {
                eprintln!("mkpage: rebuild failed: {error}");
            } else {
                snapshot = next_snapshot;
                project = current;
            }
        }
    }
}

fn build_once(project: &ResolvedProject, context: &CommandContext) -> AppResult<()> {
    let report = build_site(&BuildRequest {
        source_dir: project.root.clone(),
        output_dir: project.paths.output.clone(),
        profile: BuildProfile::development(time::OffsetDateTime::now_utc().date()),
        keyboard_runtime_enabled: project.config.enhancements.keyboard,
    })?;

    if !context.quiet {
        println!(
            "mkpage: built {} page(s), {} asset(s) in {}ms",
            report.page_count,
            report.asset_count,
            report.elapsed.as_millis()
        );
    }

    Ok(())
}

fn tracked_fingerprint(paths: &ResolvedPaths, config_path: &Path) -> AppResult<TrackedFingerprint> {
    let mut files = Vec::new();

    for path in [
        paths.source.as_path(),
        paths.layouts.as_path(),
        paths.data.as_path(),
        paths.static_files.as_path(),
    ] {
        collect_states(path, path, &mut files)?;
    }
    collect_states(config_path, config_path, &mut files)?;

    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(TrackedFingerprint { files })
}

fn collect_states(root: &Path, current: &Path, files: &mut Vec<TrackedItem>) -> AppResult<()> {
    if !current.exists() {
        return Ok(());
    }
    if current.is_file() {
        files.push(file_state(root, current)?);
        return Ok(());
    }

    let entries = fs::read_dir(current).map_err(|error| AppError::SourceRead {
        path: current.to_path_buf(),
        message: error.to_string(),
    })?;
    for entry in entries {
        let path = entry
            .map_err(|error| AppError::SourceRead {
                path: current.to_path_buf(),
                message: error.to_string(),
            })?
            .path();
        if path.is_dir() {
            collect_states(root, &path, files)?;
        } else if path.is_file() {
            files.push(file_state(root, &path)?);
        }
    }
    Ok(())
}

fn file_state(root: &Path, path: &Path) -> AppResult<TrackedItem> {
    let relative = path
        .strip_prefix(root)
        .map(|path| path.to_path_buf())
        .unwrap_or_else(|_| path.to_path_buf());

    let mut file = File::open(path).map_err(|error| AppError::SourceRead {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    let mut hasher = Sha256::new();
    let mut bytes = [0u8; 8192];
    loop {
        let count = file
            .read(&mut bytes)
            .map_err(|error| AppError::SourceRead {
                path: path.to_path_buf(),
                message: error.to_string(),
            })?;
        if count == 0 {
            break;
        }
        hasher.update(&bytes[..count]);
    }

    Ok(TrackedItem {
        path: relative,
        hash: hasher.finalize().to_vec(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrackedFingerprint {
    files: Vec<TrackedItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrackedItem {
    path: PathBuf,
    hash: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::file_state;

    #[test]
    fn file_hash_changes_when_file_changes() {
        let temp = TempDir::new().expect("temp");
        let file = temp.path().join("a.txt");
        fs::write(&file, b"first").expect("write");

        let first = file_state(temp.path(), &file).expect("state");
        fs::write(&file, b"second").expect("write2");
        let second = file_state(temp.path(), &file).expect("state2");

        assert_ne!(first, second);
    }
}
