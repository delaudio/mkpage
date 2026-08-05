//! Deterministic static-site build entry point.

use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
    time::Instant,
};

use crate::{
    assets,
    config::Site,
    enhancements,
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
    pub site: Site,
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

#[derive(Debug, Clone, Serialize)]
struct SiteMetadata {
    base_url: Option<String>,
    trailing_slash: &'static str,
    pages: Vec<MetadataPage>,
}

#[derive(Debug, Clone, Serialize)]
struct MetadataPage {
    route: String,
    path: String,
    title: Option<String>,
    description: Option<String>,
    date: Option<String>,
    updated: Option<String>,
    slug: Option<String>,
    layout: Option<String>,
    canonical_url: Option<String>,
}

#[derive(Debug, Clone)]
struct RenderedPage {
    page: MetadataPage,
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
        let route = candidate.route.as_str().to_owned();
        let output_path = candidate
            .output
            .as_path()
            .strip_prefix(&request.output_dir)
            .map_err(|error| AppError::OutputWrite {
                path: request.output_dir.clone(),
                message: error.to_string(),
            })?
            .to_path_buf();

        let rendered_metadata = RenderedPage {
            page: MetadataPage {
                route: route.clone(),
                path: output_path.to_string_lossy().to_string(),
                title: page.metadata.title.clone(),
                description: page.metadata.description.clone(),
                date: page.metadata.date.map(|date| date.to_string()),
                updated: page.metadata.updated.map(|date| date.to_string()),
                slug: page.metadata.slug.clone(),
                layout: page.metadata.layout.clone(),
                canonical_url: page.metadata.canonical_url.clone(),
            },
        };

        let document = match page.metadata.layout.as_deref() {
            Some(layout) => render_template(
                &layouts_root,
                layout,
                &page,
                &rendered,
                &request.site,
                request.keyboard_runtime_enabled,
            )?,
            None => {
                render_reference_page(&rendered.html, request.profile.shows_draft_marker(&page))
            }
        };

        planned_pages.push((output_path, document.into_bytes(), rendered_metadata));
    }

    let asset_root = request.source_dir.join("static");
    let mut generated_files = planned_pages
        .iter()
        .map(|(path, _, _)| path.clone())
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

    let mut page_metadata = Vec::new();
    if request.site.include_metadata || request.site.include_feed || request.site.include_sitemap {
        for (_, _, rendered) in &planned_pages {
            page_metadata.push(rendered.page.clone());
        }
        page_metadata.sort_by(|left, right| left.route.cmp(&right.route));
    }

    for (relative, bytes, _rendered_page) in planned_pages {
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

    if request.site.include_metadata {
        let metadata = SiteMetadata {
            base_url: request.site.base_url.clone(),
            trailing_slash: request.site.trailing_slash.as_str(),
            pages: page_metadata.clone(),
        };
        generated_files.push(PathBuf::from("metadata.json"));
        fs::write(
            request.output_dir.join("metadata.json"),
            serde_json::to_vec_pretty(&metadata).expect("metadata serialization"),
        )
        .map_err(|error| AppError::OutputWrite {
            path: request.output_dir.join("metadata.json"),
            message: error.to_string(),
        })?;
    }

    if request.site.include_sitemap {
        let base_url = request.site.base_url.clone().unwrap_or_default();
        generated_files.push(PathBuf::from("sitemap.xml"));
        fs::write(
            request.output_dir.join("sitemap.xml"),
            generate_sitemap(&base_url, &request.site.trailing_slash, &page_metadata),
        )
        .map_err(|error| AppError::OutputWrite {
            path: request.output_dir.join("sitemap.xml"),
            message: error.to_string(),
        })?;
    }

    if request.site.include_feed {
        let base_url = request.site.base_url.clone().unwrap_or_default();
        generated_files.push(PathBuf::from("feed.xml"));
        fs::write(
            request.output_dir.join("feed.xml"),
            generate_feed(&base_url, &request.site.trailing_slash, &page_metadata),
        )
        .map_err(|error| AppError::OutputWrite {
            path: request.output_dir.join("feed.xml"),
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
        let target = request.output_dir.join(path.as_path());
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
        .filter(|path: &&PathBuf| {
            path.extension()
                .is_some_and(|extension| extension == "html")
        })
        .count();
    let asset_count = generated_files
        .iter()
        .filter(|path: &&PathBuf| path.extension().is_none_or(|extension| extension != "html"))
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

fn generate_sitemap(
    base_url: &str,
    trailing_slash: &crate::config::TrailingSlash,
    pages: &[MetadataPage],
) -> Vec<u8> {
    let mut output = String::new();
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    output.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for page in pages {
        let loc = absolute_url(base_url, &page.route, trailing_slash);
        output.push_str("  <url>\n");
        output.push_str(&format!("    <loc>{}</loc>\n", xml_escape(&loc)));
        output.push_str("  </url>\n");
    }
    output.push_str("</urlset>\n");
    output.into_bytes()
}

fn generate_feed(
    base_url: &str,
    trailing_slash: &crate::config::TrailingSlash,
    pages: &[MetadataPage],
) -> Vec<u8> {
    let mut output = String::new();
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    output.push_str("<rss version=\"2.0\">\n");
    output.push_str("  <channel>\n");
    output.push_str("    <title>mkpage</title>\n");
    output.push_str("    <link>");
    output.push_str(&xml_escape(&if base_url.is_empty() {
        "/".to_string()
    } else {
        base_url.to_string()
    }));
    output.push_str("</link>\n");

    for page in pages {
        let loc = absolute_url(base_url, &page.route, trailing_slash);
        output.push_str("    <item>\n");
        let title = page.title.clone().unwrap_or_else(|| page.route.clone());
        output.push_str(&format!("      <title>{}</title>\n", xml_escape(&title)));
        output.push_str(&format!("      <link>{}</link>\n", xml_escape(&loc)));
        output.push_str(&format!("      <guid>{}</guid>\n", xml_escape(&loc)));
        if let Some(description) = &page.description {
            output.push_str(&format!(
                "      <description>{}</description>\n",
                xml_escape(description)
            ));
        }
        if let Some(date) = &page.date {
            output.push_str(&format!("      <pubDate>{}</pubDate>\n", date));
        }
        output.push_str("    </item>\n");
    }

    output.push_str("  </channel>\n");
    output.push_str("</rss>\n");
    output.into_bytes()
}

fn absolute_url(
    base_url: &str,
    route: &str,
    trailing_slash: &crate::config::TrailingSlash,
) -> String {
    if base_url.is_empty() {
        return route.to_string();
    }

    if route == "/" {
        if *trailing_slash == crate::config::TrailingSlash::Always {
            return format!("{}/", base_url.trim_end_matches('/'));
        }
        return base_url.trim_end_matches('/').to_string();
    }

    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        route.trim_matches('/')
    )
}

fn xml_escape(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        match char {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            _ => output.push(char),
        }
    }
    output
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
