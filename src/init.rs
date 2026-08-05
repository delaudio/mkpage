//! Project scaffolding and safe starter initialization.

use std::{fs, path::PathBuf};

use crate::{
    cli::Init as InitCommand,
    error::{AppError, AppResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitSummary {
    pub created: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
}

#[derive(Debug)]
struct TemplateAsset {
    path: &'static str,
    contents: &'static str,
}

/// Execute mkpage init with deterministic, overwrite-safe scaffold creation.
pub fn run(_context: super::CommandContext, request: InitCommand) -> AppResult<InitSummary> {
    let target = request.directory;
    let template = request.template;
    if template != "default" {
        return Err(AppError::Message {
            message: format!("unknown template `{template}`; available templates: default"),
        });
    }
    let files = template_files();
    if !target.exists() {
        fs::create_dir_all(&target).map_err(|error| AppError::OutputWrite {
            path: target.clone(),
            message: error.to_string(),
        })?;
    }
    if !is_directory_empty(&target)? {
        return Err(AppError::Message {
            message: format!(
                "refusing to initialize in non-empty directory: {}",
                target.display()
            ),
        });
    }
    let mut summary = InitSummary {
        created: Vec::new(),
        skipped: Vec::new(),
    };

    for asset in files {
        let destination = target.join(asset.path);
        if destination.exists() {
            summary.skipped.push(PathBuf::from(asset.path));
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| AppError::OutputWrite {
                path: parent.to_path_buf(),
                message: error.to_string(),
            })?;
        }
        fs::write(&destination, asset.contents).map_err(|error| AppError::OutputWrite {
            path: destination,
            message: error.to_string(),
        })?;
        summary.created.push(PathBuf::from(asset.path));
    }
    Ok(summary)
}

fn template_files() -> &'static [TemplateAsset] {
    &[
        TemplateAsset {
            path: "mkpage.toml",
            contents: include_str!("../assets/init/default/mkpage.toml"),
        },
        TemplateAsset {
            path: "README.md",
            contents: include_str!("../assets/init/default/README.md"),
        },
        TemplateAsset {
            path: "layouts/page.html",
            contents: include_str!("../assets/init/default/layouts/page.html"),
        },
        TemplateAsset {
            path: "layouts/widgets.jinja",
            contents: include_str!("../assets/init/default/layouts/widgets.jinja"),
        },
        TemplateAsset {
            path: "content/index.md",
            contents: include_str!("../assets/init/default/content/index.md"),
        },
        TemplateAsset {
            path: "content/about.md",
            contents: include_str!("../assets/init/default/content/about.md"),
        },
        TemplateAsset {
            path: "content/uses.md",
            contents: include_str!("../assets/init/default/content/uses.md"),
        },
        TemplateAsset {
            path: "content/projects/index.md",
            contents: include_str!("../assets/init/default/content/projects/index.md"),
        },
        TemplateAsset {
            path: "content/projects/mkpage.md",
            contents: include_str!("../assets/init/default/content/projects/mkpage.md"),
        },
        TemplateAsset {
            path: "content/writing/index.md",
            contents: include_str!("../assets/init/default/content/writing/index.md"),
        },
        TemplateAsset {
            path: "content/writing/notes-from-terminal-design.md",
            contents: include_str!(
                "../assets/init/default/content/writing/notes-from-terminal-design.md"
            ),
        },
        TemplateAsset {
            path: "data/projects.json",
            contents: include_str!("../assets/init/default/data/projects.json"),
        },
        TemplateAsset {
            path: "static/css/site.css",
            contents: include_str!("../assets/init/default/static/css/site.css"),
        },
        TemplateAsset {
            path: "static/css/override.css",
            contents: include_str!("../assets/init/default/static/css/override.css"),
        },
    ]
}

fn is_directory_empty(path: &std::path::Path) -> AppResult<bool> {
    let mut entries = fs::read_dir(path).map_err(|error| AppError::OutputWrite {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    Ok(entries.next().is_none())
}
