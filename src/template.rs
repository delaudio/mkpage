//! Bounded MiniJinja layout rendering.

use std::{fs, path::Path};

use minijinja::{Environment, context, value::Value};
use serde_json::json;

use crate::{
    data::CollectionItem,
    error::{AppError, AppResult},
    markdown::RenderedMarkdown,
    page::Page,
};

pub fn render_collection(
    layouts: &Path,
    layout: &str,
    item: &CollectionItem,
    data: &serde_json::Value,
) -> AppResult<String> {
    let mut environment = Environment::new();
    environment.set_auto_escape_callback(|_| minijinja::AutoEscape::Html);
    load_templates(&mut environment, layouts, layouts)?;
    let name = normalize_name(layout);
    let template = environment
        .get_template(&name)
        .map_err(|error| AppError::Template {
            path: layouts.join(&name),
            message: error.to_string(),
        })?;
    template
        .render(context! {
            site => json!({ "base_url": null, "trailing_slash": "always" }),
            page => json!({ "route": item.route, "slug": item.slug }),
            item => item.value.clone(),
            collection => json!({ "key": item.key, "slug": item.slug, "route": item.route }),
            data => data,
            build => json!({ "profile": "bounded" }),
        })
        .map_err(|error| AppError::Template {
            path: layouts.join(&name),
            message: error.to_string(),
        })
}

pub fn render(
    layouts: &Path,
    layout: &str,
    page: &Page,
    markdown: &RenderedMarkdown,
) -> AppResult<String> {
    let mut environment = Environment::new();
    environment.set_auto_escape_callback(|_| minijinja::AutoEscape::Html);
    load_templates(&mut environment, layouts, layouts)?;
    let name = normalize_name(layout);
    let template = environment
        .get_template(&name)
        .map_err(|error| AppError::Template {
            path: layouts.join(&name),
            message: error.to_string(),
        })?;
    template.render(context! {
        site => json!({ "base_url": null, "trailing_slash": "always" }),
        page => json!({ "title": page.metadata.title, "description": page.metadata.description, "layout": page.metadata.layout, "slug": page.metadata.slug, "tags": page.metadata.tags, "projects": page.metadata.projects, "headings": markdown.headings.iter().map(|heading| json!({ "level": heading.level, "id": heading.id, "text": heading.text })).collect::<Vec<_>>(), "links": markdown.links.iter().map(|link| json!({ "url": link.url, "internal": link.internal, "outbound": link.outbound })).collect::<Vec<_>>() }),
        content => Value::from_safe_string(markdown.html.clone()),
        data => {},
        build => json!({ "profile": "bounded" }),
    }).map_err(|error| AppError::Template { path: layouts.join(&name), message: error.to_string() })
}

fn load_templates(environment: &mut Environment<'_>, root: &Path, current: &Path) -> AppResult<()> {
    let mut entries = fs::read_dir(current)
        .map_err(|error| AppError::Template {
            path: current.to_path_buf(),
            message: error.to_string(),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| AppError::Template {
            path: current.to_path_buf(),
            message: error.to_string(),
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            load_templates(environment, root, &path)?;
            continue;
        }
        if !matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("html") | Some("jinja")
        ) {
            continue;
        }
        let name = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&path).map_err(|error| AppError::Template {
            path: path.clone(),
            message: error.to_string(),
        })?;
        environment
            .add_template_owned(name, source)
            .map_err(|error| AppError::Template {
                path: path.clone(),
                message: error.to_string(),
            })?;
    }
    Ok(())
}

fn normalize_name(layout: &str) -> String {
    if layout.ends_with(".html") || layout.ends_with(".jinja") {
        layout.replace('\\', "/")
    } else {
        format!("{}.html", layout.replace('\\', "/"))
    }
}
