use std::{fs, path::Path};

use mkpage::{
    config::{CONFIG_FILE_NAME, ResolveOptions, discover_root, resolve},
    error::AppError,
};
use tempfile::TempDir;

fn project(config: &str) -> TempDir {
    let temp = tempfile::tempdir().expect("temporary project");
    fs::write(temp.path().join(CONFIG_FILE_NAME), config).expect("configuration");
    temp
}

fn resolve_from(root: &Path) -> mkpage::config::ResolvedProject {
    resolve(&ResolveOptions {
        start_dir: root.to_path_buf(),
        root: None,
        config: None,
    })
    .expect("project should resolve")
}

#[test]
fn discovers_the_nearest_project_from_a_nested_directory() {
    let temp = project("version = 1\n");
    let nested = temp.path().join("content").join("posts");
    fs::create_dir_all(&nested).expect("nested directory");

    assert_eq!(discover_root(&nested), Some(temp.path().to_path_buf()));
    assert_eq!(resolve_from(&nested).root, temp.path());
}

#[test]
fn explicit_config_takes_precedence_and_resolves_relative_to_root() {
    let temp = project("version = 1\n");
    fs::write(
        temp.path().join("alternate.toml"),
        "version = 1\n[paths]\noutput = \"dist\"\n",
    )
    .expect("alternate config");

    let project = resolve(&ResolveOptions {
        start_dir: temp.path().join("nested"),
        root: Some(temp.path().to_path_buf()),
        config: Some("alternate.toml".into()),
    })
    .expect("explicit config");
    assert_eq!(project.config_path, temp.path().join("alternate.toml"));
    assert_eq!(project.paths.output, temp.path().join("dist"));
}

#[test]
fn rejects_unknown_fields_and_unsupported_versions_with_config_path() {
    let unknown = project("version = 1\ntyop = true\n");
    let error = resolve(&ResolveOptions {
        start_dir: unknown.path().to_path_buf(),
        root: None,
        config: None,
    })
    .expect_err("unknown key");
    assert!(matches!(error, AppError::ConfigParse { .. }));
    assert_eq!(error.code(), "E402");

    let version = project("version = 2\n");
    let error = resolve(&ResolveOptions {
        start_dir: version.path().to_path_buf(),
        root: None,
        config: None,
    })
    .expect_err("unsupported version");
    assert!(matches!(error, AppError::UnsupportedConfigVersion { .. }));
    assert_eq!(error.code(), "E403");
}

#[test]
fn rejects_unsafe_source_output_relationships_before_writes() {
    let temp = project("version = 1\n[paths]\nsource = \"content\"\noutput = \"content/public\"\n");
    let error = resolve(&ResolveOptions {
        start_dir: temp.path().to_path_buf(),
        root: None,
        config: None,
    })
    .expect_err("overlapping output");
    assert!(matches!(error, AppError::UnsafePathRelationship { .. }));
    assert_eq!(error.code(), "E405");

    let root_output = project("version = 1\n[paths]\noutput = \".\"\n");
    let error = resolve(&ResolveOptions {
        start_dir: root_output.path().to_path_buf(),
        root: None,
        config: None,
    })
    .expect_err("project root output");
    assert!(matches!(error, AppError::UnsafeOutputPath { .. }));
    assert_eq!(error.code(), "E404");
}

#[test]
fn defaults_are_documented_by_the_resolved_model() {
    let temp = project("version = 1\n");
    let project = resolve_from(temp.path());
    assert_eq!(project.paths.source, temp.path().join("content"));
    assert_eq!(project.paths.output, temp.path().join("public"));
    assert_eq!(project.config.theme.name, "terminal");
    assert!(project.config.enhancements.keyboard);
}
