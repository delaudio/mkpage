//! Deterministic JSON data loading and collection route planning.

use crate::error::{AppError, AppResult};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionItem {
    pub key: String,
    pub slug: String,
    pub route: String,
    pub value: Value,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CollectionManifest {
    pub source: String,
    pub layout: String,
    pub output: String,
    #[serde(default)]
    pub slug_field: Option<String>,
}

pub fn load(root: &Path) -> AppResult<BTreeMap<String, Value>> {
    let mut files = Vec::new();
    visit(root, &mut files)?;
    files.sort();
    let mut result = BTreeMap::new();
    for path in files {
        let key = path
            .strip_prefix(root)
            .expect("data path under root")
            .with_extension("")
            .to_string_lossy()
            .replace(['\\', '/'], ".");
        let text = fs::read_to_string(&path).map_err(|error| AppError::Data {
            path: path.clone(),
            message: error.to_string(),
        })?;
        let value = serde_json::from_str(&text).map_err(|error| AppError::Data {
            path: path.clone(),
            message: error.to_string(),
        })?;
        if result.insert(key.clone(), value).is_some() {
            return Err(AppError::Data {
                path,
                message: format!("duplicate logical data key `{key}`"),
            });
        }
    }
    Ok(result)
}

pub fn collection(
    data: &BTreeMap<String, Value>,
    source: &str,
    pattern: &str,
    slug_field: Option<&str>,
) -> AppResult<Vec<CollectionItem>> {
    let source_value = data.get(source).ok_or_else(|| AppError::Data {
        path: PathBuf::from(source),
        message: "collection source is missing".into(),
    })?;
    let pairs: Vec<(String, Value)> = match source_value {
        Value::Array(values) => values
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v.clone()))
            .collect(),
        Value::Object(values) => values.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        _ => {
            return Err(AppError::Data {
                path: PathBuf::from(source),
                message: "collection source must be an array or object".into(),
            });
        }
    };
    if !pattern.starts_with('/') || pattern.contains("..") || !pattern.contains("{slug}") {
        return Err(AppError::Data {
            path: PathBuf::from(source),
            message: "output pattern must be an absolute clean route containing {slug}".into(),
        });
    }
    let mut used = BTreeSet::new();
    let mut items = Vec::new();
    for (key, value) in pairs {
        let raw = slug_field
            .and_then(|field| value.get(field))
            .and_then(Value::as_str)
            .unwrap_or(&key);
        let slug = slugify(raw);
        if slug.is_empty() || !used.insert(slug.clone()) {
            return Err(AppError::Data {
                path: PathBuf::from(source),
                message: format!("duplicate or empty collection slug `{raw}`"),
            });
        }
        items.push(CollectionItem {
            key,
            route: pattern.replace("{slug}", &slug),
            slug,
            value,
        });
    }
    Ok(items)
}
fn visit(current: &Path, files: &mut Vec<PathBuf>) -> AppResult<()> {
    for entry in fs::read_dir(current).map_err(|error| AppError::Data {
        path: current.to_path_buf(),
        message: error.to_string(),
    })? {
        let path = entry
            .map_err(|error| AppError::Data {
                path: current.to_path_buf(),
                message: error.to_string(),
            })?
            .path();
        if path.is_dir() {
            visit(&path, files)?
        } else if path.extension().and_then(|v| v.to_str()) == Some("json") {
            files.push(path)
        }
    }
    Ok(())
}
fn slugify(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
