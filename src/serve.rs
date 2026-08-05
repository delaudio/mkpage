//! HTTP serving helpers for local preview.

use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Component, Path, PathBuf},
};

use crate::CommandContext;
use crate::{
    cli::Serve,
    config::{ResolveOptions, resolve},
    error::{AppError, AppResult},
};

/// Dispatches `mkpage serve` to the local preview loop.
pub fn run(context: CommandContext, args: Serve) -> AppResult<()> {
    let project = resolve(&ResolveOptions {
        start_dir: std::env::current_dir().map_err(|error| AppError::Message {
            message: error.to_string(),
        })?,
        root: context.root,
        config: context.config,
    })?;

    if !project.paths.output.exists() {
        return Err(AppError::Message {
            message: format!(
                "output directory does not exist: {}. Run `mkpage build` first.",
                project.paths.output.display()
            ),
        });
    }

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

    serve_from_listener(listener, project.paths.output, !context.quiet)
}

/// Runs a static file server from an already-bound listener.
pub fn serve_from_listener(listener: TcpListener, root: PathBuf, verbose: bool) -> AppResult<()> {
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                return Err(AppError::Message {
                    message: error.to_string(),
                });
            }
        };

        if let Err(error) = handle_request(&mut stream, &root) {
            if verbose {
                eprintln!("mkpage: request error: {error}");
            }
        }
    }

    Ok(())
}

fn handle_request(stream: &mut TcpStream, root: &Path) -> AppResult<()> {
    let mut buffer = [0u8; 8192];
    let len = stream
        .read(&mut buffer)
        .map_err(|error| AppError::Message {
            message: error.to_string(),
        })?;
    if len == 0 {
        return Ok(());
    }

    let request_line = match std::str::from_utf8(&buffer[..len]) {
        Ok(text) => text.split("\r\n").next().unwrap_or("").to_string(),
        Err(_) => {
            return write_text_response(stream, 400, "Bad Request", "invalid request", false);
        }
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");

    if method != "GET" && method != "HEAD" {
        return write_text_response(
            stream,
            405,
            "Method Not Allowed",
            "only GET and HEAD are supported",
            false,
        );
    }

    let (requested, is_directory) = match resolve_request_path(target) {
        Some(value) => value,
        None => {
            return write_text_response(stream, 400, "Bad Request", "unsafe request path", false);
        }
    };

    let file = if is_directory {
        requested.join("index.html")
    } else {
        requested.clone()
    };
    let candidate = ensure_index_or_file(root, &file).unwrap_or(file);
    let candidate = root.join(candidate);

    if candidate.is_dir() {
        return write_text_response(stream, 404, "Not Found", "not found", method == "HEAD");
    }

    let bytes = match fs::read(&candidate) {
        Ok(bytes) => bytes,
        Err(_) => {
            return write_text_response(stream, 404, "Not Found", "not found", method == "HEAD");
        }
    };

    write_file_response(stream, 200, "OK", &candidate, &bytes, method == "HEAD")
}

fn ensure_index_or_file(root: &Path, requested: &Path) -> Option<PathBuf> {
    let safe = match safe_relative_path(requested) {
        Ok(path) => path,
        Err(_) => return None,
    };
    let candidate = root.join(&safe);
    if candidate.is_file() {
        return Some(safe);
    }
    let index_path = safe.join("index.html");
    if root.join(index_path).exists() {
        return Some(safe.join("index.html"));
    }
    None
}

fn resolve_request_path(raw: &str) -> Option<(PathBuf, bool)> {
    let path = raw.split('?').next().unwrap_or("");
    let decoded = percent_decode(path);
    let no_prefix = decoded.strip_prefix('/').unwrap_or(&decoded);
    if no_prefix.is_empty() {
        return Some((PathBuf::new(), true));
    }
    let candidate = Path::new(no_prefix);
    let safe = safe_relative_path(candidate).ok()?;
    Some((safe, path.ends_with('/')))
}

fn safe_relative_path(path: &Path) -> std::result::Result<PathBuf, ()> {
    let mut normalized = PathBuf::new();
    for part in path.components() {
        match part {
            Component::Prefix(_) | Component::RootDir => {
                return Err(());
            }
            Component::ParentDir => return Err(()),
            Component::CurDir => continue,
            Component::Normal(segment) => normalized.push(segment),
        }
    }
    Ok(normalized)
}

fn write_text_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    body: &str,
    head_only: bool,
) -> AppResult<()> {
    let bytes = body.as_bytes();
    write_response(
        stream,
        status,
        reason,
        "text/plain; charset=utf-8",
        bytes,
        head_only,
    )
}

fn write_file_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    file: &Path,
    bytes: &[u8],
    head_only: bool,
) -> AppResult<()> {
    write_response(stream, status, reason, content_type(file), bytes, head_only)
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    content_type: &str,
    bytes: &[u8],
    head_only: bool,
) -> AppResult<()> {
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status,
        reason,
        content_type,
        bytes.len(),
    );
    stream
        .write_all(header.as_bytes())
        .map_err(|error| AppError::Message {
            message: error.to_string(),
        })?;
    if !head_only {
        stream.write_all(bytes).map_err(|error| AppError::Message {
            message: error.to_string(),
        })?;
    }
    Ok(())
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("txt") => "text/plain; charset=utf-8",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        _ => "application/octet-stream",
    }
}

fn percent_decode(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut output = String::with_capacity(raw.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let decoded = hex_pair(bytes[i + 1], bytes[i + 2]);
            if let Some(byte) = decoded {
                output.push(char::from(byte));
                i += 3;
                continue;
            }
        }
        output.push(bytes[i] as char);
        i += 1;
    }
    output
}

fn hex_pair(a: u8, b: u8) -> Option<u8> {
    let hi = hex_value(a)?;
    let lo = hex_value(b)?;
    Some((hi << 4) | lo)
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use super::{ensure_index_or_file, safe_relative_path};

    #[test]
    fn resolve_request_paths_ensure_index_fallback() {
        let temp = TempDir::new().expect("temp dir");
        fs::write(temp.path().join("index.html"), "home").expect("write index");
        fs::create_dir(temp.path().join("posts")).expect("mkdir");
        fs::write(temp.path().join("posts").join("index.html"), "post").expect("write post index");

        assert_eq!(
            ensure_index_or_file(temp.path(), &PathBuf::from("")).expect("root index"),
            PathBuf::from("index.html")
        );
        assert_eq!(
            ensure_index_or_file(temp.path(), &PathBuf::from("posts")).expect("directory"),
            PathBuf::from("posts/index.html")
        );
        assert!(ensure_index_or_file(temp.path(), &PathBuf::from("missing")).is_none());
    }

    #[test]
    fn safe_relative_path_blocks_traversal() {
        assert!(safe_relative_path(&PathBuf::from("../etc/passwd")).is_err());
        assert!(safe_relative_path(&PathBuf::from("/absolute")).is_err());
    }
}
