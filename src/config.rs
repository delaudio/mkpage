//! Project discovery and versioned configuration resolution.

use std::{
    env, fs,
    path::{Component, Path, PathBuf},
};

use serde::Deserialize;

use crate::error::{AppError, AppResult};

pub const CONFIG_FILE_NAME: &str = "mkpage.toml";
pub const CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub paths: Paths,
    pub site: Site,
    pub theme: Theme,
    pub enhancements: Enhancements,
    pub development: Development,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Paths {
    pub source: PathBuf,
    pub layouts: PathBuf,
    pub data: PathBuf,
    pub static_files: PathBuf,
    pub output: PathBuf,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Site {
    pub base_url: Option<String>,
    pub trailing_slash: TrailingSlash,
    pub include_metadata: bool,
    pub include_feed: bool,
    pub include_sitemap: bool,
    pub include_search: bool,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TrailingSlash {
    #[default]
    Always,
    Never,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Enhancements {
    pub keyboard: bool,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct Development {
    pub base_url: Option<String>,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            source: "content".into(),
            layouts: "layouts".into(),
            data: "data".into(),
            static_files: "static".into(),
            output: "public".into(),
            include: vec![],
            exclude: vec![],
        }
    }
}
impl Default for Site {
    fn default() -> Self {
        Self {
            base_url: None,
            trailing_slash: TrailingSlash::Always,
            include_metadata: false,
            include_feed: false,
            include_sitemap: false,
            include_search: false,
        }
    }
}

impl TrailingSlash {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "terminal".into(),
        }
    }
}
impl Default for Enhancements {
    fn default() -> Self {
        Self { keyboard: true }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveOptions {
    pub start_dir: PathBuf,
    pub root: Option<PathBuf>,
    pub config: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedProject {
    pub root: PathBuf,
    pub config_path: PathBuf,
    pub config: Config,
    pub paths: ResolvedPaths,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPaths {
    pub source: PathBuf,
    pub layouts: PathBuf,
    pub data: PathBuf,
    pub static_files: PathBuf,
    pub output: PathBuf,
}

pub fn resolve(options: &ResolveOptions) -> AppResult<ResolvedProject> {
    let start = absolute_lexical(&options.start_dir)?;
    let explicit_root = options.root.as_ref().map(|path| resolve_from(&start, path));
    let config_path = match &options.config {
        Some(path) => resolve_from(explicit_root.as_deref().unwrap_or(&start), path),
        None => {
            let root = explicit_root
                .clone()
                .unwrap_or_else(|| discover_root(&start).unwrap_or(start.clone()));
            root.join(CONFIG_FILE_NAME)
        }
    };
    let root =
        explicit_root.unwrap_or_else(|| config_path.parent().unwrap_or(&start).to_path_buf());
    let text = fs::read_to_string(&config_path).map_err(|error| AppError::ConfigRead {
        path: config_path.clone(),
        message: error.to_string(),
    })?;
    let config: Config = toml::from_str(&text).map_err(|error| AppError::ConfigParse {
        path: config_path.clone(),
        message: error.to_string(),
    })?;
    if config.version != CONFIG_VERSION {
        return Err(AppError::UnsupportedConfigVersion {
            path: config_path,
            found: config.version,
        });
    }
    let base = config_path.parent().unwrap_or(&root);
    let paths = ResolvedPaths {
        source: resolve_from(base, &config.paths.source),
        layouts: resolve_from(base, &config.paths.layouts),
        data: resolve_from(base, &config.paths.data),
        static_files: resolve_from(base, &config.paths.static_files),
        output: resolve_from(base, &config.paths.output),
    };
    validate_paths(&root, &paths)?;
    Ok(ResolvedProject {
        root,
        config_path,
        config,
        paths,
    })
}

pub fn discover_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(CONFIG_FILE_NAME).is_file() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

pub fn display_path(project: &ResolvedProject, path: &Path) -> PathBuf {
    path.strip_prefix(&project.root)
        .map_or_else(|_| path.to_path_buf(), Path::to_path_buf)
}

fn absolute_lexical(path: &Path) -> AppResult<PathBuf> {
    let initial = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|error| AppError::ConfigRead {
                path: path.to_path_buf(),
                message: error.to_string(),
            })?
            .join(path)
    };
    Ok(normalize(&initial))
}
fn resolve_from(base: &Path, path: &Path) -> PathBuf {
    normalize(&if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    })
}
fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            _ => result.push(component.as_os_str()),
        }
    }
    result
}

fn validate_paths(root: &Path, paths: &ResolvedPaths) -> AppResult<()> {
    if paths.output == root
        || paths.output.parent().is_none()
        || home_dir().is_some_and(|home| paths.output == home)
    {
        return Err(AppError::UnsafeOutputPath {
            path: paths.output.clone(),
        });
    }
    for source in [
        &paths.source,
        &paths.layouts,
        &paths.data,
        &paths.static_files,
    ] {
        if source.starts_with(&paths.output) || paths.output.starts_with(source) {
            return Err(AppError::UnsafePathRelationship {
                input: source.clone(),
                output: paths.output.clone(),
            });
        }
    }
    Ok(())
}
fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .map(|path| normalize(&path))
}
