//! HTTP serving helpers for local preview.

use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    path::{Component, Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::Duration,
};

use crate::CommandContext;
use crate::{
    cli::Serve,
    config::{ResolveOptions, resolve},
    error::{AppError, AppResult},
};

const DEV_RELOAD_PATH: &str = "/__mkpage/dev/reload";

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
    serve_from_listener_with_reload(listener, root, verbose, None)
}

/// Runs a static file server from an already-bound listener with reload events.
pub fn serve_from_listener_with_reload(
    listener: TcpListener,
    root: PathBuf,
    verbose: bool,
    reload_epoch: Option<Arc<AtomicU64>>,
) -> AppResult<()> {
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                return Err(AppError::Message {
                    message: error.to_string(),
                });
            }
        };
        let root = root.clone();
        let reload_epoch = reload_epoch.clone();

        thread::spawn(move || {
            let mut stream = stream;
            if let Err(error) = handle_request(&mut stream, &root, reload_epoch) {
                if verbose {
                    eprintln!("mkpage: request error: {error}");
                }
            }
        });
    }

    Ok(())
}

fn handle_request<W>(
    stream: &mut W,
    root: &Path,
    reload_epoch: Option<Arc<AtomicU64>>,
) -> AppResult<()>
where
    W: Read + Write,
{
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
            return write_text_response(stream, 400, "Bad Request", "invalid request", false, None);
        }
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");

    if method == "GET" && target == DEV_RELOAD_PATH {
        return match reload_epoch {
            Some(epoch) => serve_reload_stream(stream, epoch),
            None => write_text_response(
                stream,
                404,
                "Not Found",
                "dev reload endpoint not available",
                false,
                None,
            ),
        };
    }

    if method != "GET" && method != "HEAD" {
        return write_text_response(
            stream,
            405,
            "Method Not Allowed",
            "only GET and HEAD are supported",
            false,
            Some("GET, HEAD"),
        );
    }

    let (requested, is_directory) = match resolve_request_path(target) {
        Some(value) => value,
        None => {
            return write_text_response(
                stream,
                400,
                "Bad Request",
                "unsafe request path",
                false,
                None,
            );
        }
    };

    let file = if is_directory {
        requested.join("index.html")
    } else {
        requested.clone()
    };
    let candidate = resolve_candidate(root, &file);

    match candidate {
        Some(candidate) => {
            let bytes = fs::read(&candidate).map_err(|error| AppError::SourceRead {
                path: candidate.clone(),
                message: error.to_string(),
            })?;
            write_file_response(
                stream,
                200,
                "OK",
                &candidate,
                &bytes,
                method == "HEAD",
                reload_epoch,
            )
        }
        None => {
            let fallback = root.join("404.html");
            if fallback.exists() {
                let bytes = fs::read(&fallback).map_err(|error| AppError::SourceRead {
                    path: fallback.clone(),
                    message: error.to_string(),
                })?;
                return write_file_response(
                    stream,
                    404,
                    "Not Found",
                    &fallback,
                    &bytes,
                    method == "HEAD",
                    None,
                );
            }

            write_text_response(
                stream,
                404,
                "Not Found",
                "not found",
                method == "HEAD",
                None,
            )
        }
    }
}

fn serve_reload_stream<W>(stream: &mut W, reload_epoch: Arc<AtomicU64>) -> AppResult<()>
where
    W: Write,
{
    let headers = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream; charset=utf-8\r\nCache-Control: no-cache\r\n\r\n";
    stream
        .write_all(headers.as_bytes())
        .map_err(|error| AppError::Message {
            message: error.to_string(),
        })?;

    let mut last = reload_epoch.load(Ordering::Acquire);
    loop {
        let current = reload_epoch.load(Ordering::Acquire);
        if current != last {
            let event = format!("event: reload\ndata: {current}\n\n");
            stream
                .write_all(event.as_bytes())
                .map_err(|_| AppError::Message {
                    message: "client disconnected".to_string(),
                })?;
            last = current;
        }
        thread::sleep(Duration::from_millis(250));
    }
}

fn resolve_candidate(root: &Path, requested: &Path) -> Option<PathBuf> {
    let safe = safe_relative_path(requested).ok()?;
    if safe.as_os_str().is_empty() {
        let index = root.join("index.html");
        if index.is_file() {
            return Some(index);
        }
        return None;
    }

    let candidate = root.join(&safe);
    if candidate.is_file() {
        return Some(candidate);
    }

    let index = candidate.join("index.html");
    if index.is_file() {
        return Some(index);
    }

    None
}

fn resolve_request_path(raw: &str) -> Option<(PathBuf, bool)> {
    let path = raw.split('?').next().unwrap_or("");
    if path == DEV_RELOAD_PATH {
        return Some((
            PathBuf::from(DEV_RELOAD_PATH.trim_start_matches('/')),
            false,
        ));
    }

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

fn write_text_response<W>(
    stream: &mut W,
    status: u16,
    reason: &str,
    body: &str,
    head_only: bool,
    allow: Option<&str>,
) -> AppResult<()>
where
    W: Write,
{
    let bytes = body.as_bytes();
    write_response(
        stream,
        status,
        reason,
        "text/plain; charset=utf-8",
        bytes,
        head_only,
        allow,
    )
}

fn write_file_response<W>(
    stream: &mut W,
    status: u16,
    reason: &str,
    file: &Path,
    bytes: &[u8],
    head_only: bool,
    reload_epoch: Option<Arc<AtomicU64>>,
) -> AppResult<()>
where
    W: Write,
{
    let bytes = if reload_epoch.is_some()
        && file.extension().and_then(|extension| extension.to_str()) == Some("html")
    {
        inject_live_reload_script(bytes)
    } else {
        bytes.to_vec()
    };

    write_response(
        stream,
        status,
        reason,
        content_type(file),
        &bytes,
        head_only,
        None,
    )
}

fn write_response<W>(
    stream: &mut W,
    status: u16,
    reason: &str,
    content_type: &str,
    bytes: &[u8],
    head_only: bool,
    allow: Option<&str>,
) -> AppResult<()>
where
    W: Write,
{
    let mut header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n",
        status,
        reason,
        content_type,
        bytes.len(),
    );
    if let Some(allowed) = allow {
        header.push_str(&format!("Allow: {allowed}\r\n"));
    }
    header.push_str("Cache-Control: no-cache, no-store, must-revalidate\r\n\r\n");

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
    let a = (a as char).to_digit(16)? as u8;
    let b = (b as char).to_digit(16)? as u8;
    Some((a << 4) | b)
}

fn inject_live_reload_script(bytes: &[u8]) -> Vec<u8> {
    const MARKER: &str = "</body>";
    const SCRIPT: &str = "<script>(function(){const source=new EventSource('/__mkpage/dev/reload');source.addEventListener('reload',function(){window.location.reload();});})();</script>";

    let source = String::from_utf8_lossy(bytes);
    if let Some(index) = source.rfind(MARKER) {
        let mut output = String::with_capacity(source.len() + SCRIPT.len());
        output.push_str(&source[..index]);
        output.push_str(SCRIPT);
        output.push_str(&source[index..]);
        output.into_bytes()
    } else {
        format!("{source}{SCRIPT}").into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Cursor, Read},
        path::PathBuf,
    };
    use tempfile::TempDir;

    use super::{
        handle_request, inject_live_reload_script, resolve_candidate, resolve_request_path,
        safe_relative_path,
    };

    #[test]
    fn resolve_request_paths_ensure_index_fallback() {
        let temp = TempDir::new().expect("temp dir");
        fs::write(temp.path().join("index.html"), "home").expect("write index");
        fs::create_dir(temp.path().join("posts")).expect("mkdir");
        fs::write(temp.path().join("posts").join("index.html"), "post").expect("write post index");

        assert_eq!(
            resolve_candidate(temp.path(), &PathBuf::from("")).expect("root index"),
            temp.path().join("index.html")
        );
        assert_eq!(
            resolve_candidate(temp.path(), &PathBuf::from("posts")).expect("directory"),
            temp.path().join("posts/index.html")
        );
        assert!(resolve_candidate(temp.path(), &PathBuf::from("missing")).is_none());
    }

    #[test]
    fn resolve_request_paths_split_query_and_root() {
        assert_eq!(
            resolve_request_path("/blog/page.html?x=1"),
            Some((PathBuf::from("blog/page.html"), false))
        );
        assert_eq!(
            resolve_request_path("/blog/?page=2"),
            Some((PathBuf::from("blog"), true))
        );
        assert_eq!(resolve_request_path("/"), Some((PathBuf::new(), true)));
    }

    #[test]
    fn safe_relative_path_blocks_traversal() {
        assert!(safe_relative_path(&PathBuf::from("../etc/passwd")).is_err());
        assert!(safe_relative_path(&PathBuf::from("/absolute")).is_err());
    }

    #[test]
    fn method_not_allowed_includes_allow_header() {
        let temp = TempDir::new().expect("temp dir");
        let response =
            run_request_via_cursor(temp.path(), b"POST / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        let (status_line, headers) = split_response(&response);

        assert_eq!(status_line, "HTTP/1.1 405 Method Not Allowed");
        assert!(
            headers.contains("Allow: GET, HEAD"),
            "missing allow header in {headers}"
        );
    }

    #[test]
    fn head_request_does_not_return_body() {
        let temp = TempDir::new().expect("temp dir");
        fs::write(temp.path().join("index.html"), b"index page").expect("write");

        let response =
            run_request_via_cursor(temp.path(), b"HEAD / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        let (status_line, body) = split_response_parts(&response);
        assert_eq!(status_line, "HTTP/1.1 200 OK");
        assert!(
            status_line.contains("200"),
            "expected 200 for head request: {status_line}"
        );
        assert_eq!(body.len(), 0);
    }

    #[test]
    fn injects_reload_script_for_html_pages() {
        let html = b"<html><body><h1>hello</h1></body></html>";
        let patched = inject_live_reload_script(html);
        assert!(
            std::str::from_utf8(&patched)
                .expect("utf8")
                .contains("/__mkpage/dev/reload")
        );
    }

    #[test]
    fn query_parameters_are_preserved_for_path_matching() {
        let temp = TempDir::new().expect("temp dir");
        fs::create_dir(temp.path().join("blog")).expect("mkdir");
        fs::write(temp.path().join("blog").join("index.html"), "blog").expect("write");

        let response = run_request_via_cursor(
            temp.path(),
            b"GET /blog/?page=2 HTTP/1.1\r\nHost: localhost\r\n\r\n",
        );
        let (status_line, body) = split_response_parts(&response);

        assert_eq!(status_line, "HTTP/1.1 200 OK");
        assert_eq!(std::str::from_utf8(body).expect("utf8"), "blog");
    }

    fn run_request_via_cursor(root: &std::path::Path, request: &[u8]) -> Vec<u8> {
        let mut stream = Cursor::new(request.to_vec());
        let bytes_len = stream.get_ref().len() as u64;
        let root = root.to_path_buf();
        handle_request(&mut stream, &root, None).expect("handle");

        let mut response = Vec::new();
        stream.set_position(bytes_len);
        stream.read_to_end(&mut response).expect("read response");
        response
    }

    fn split_response(response: &[u8]) -> (String, String) {
        let delimiter = b"\r\n\r\n";
        let idx = response
            .windows(delimiter.len())
            .position(|window| window == delimiter)
            .map(|i| i + delimiter.len())
            .unwrap_or(0);
        let header_block = String::from_utf8_lossy(&response[..idx]).into_owned();
        let status = header_block.lines().next().unwrap_or("").trim().to_string();
        let headers = header_block.lines().skip(1).collect::<Vec<_>>().join("\n");
        (status, headers)
    }

    fn split_response_parts(response: &[u8]) -> (String, &[u8]) {
        let delimiter = b"\r\n\r\n";
        let idx = response
            .windows(delimiter.len())
            .position(|window| window == delimiter)
            .map(|i| i + delimiter.len())
            .unwrap_or(0);
        let status = String::from_utf8_lossy(&response[..idx])
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        (status, &response[idx..])
    }
}
