#![allow(dead_code)]

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use mkpage::{
    compiler::BuildRequest,
    page::{BuildProfile, calendar_date},
};
use tempfile::TempDir;

pub struct Fixture {
    pub temp: TempDir,
    pub source: PathBuf,
    pub output: PathBuf,
    pub golden: PathBuf,
}

impl Fixture {
    pub fn copy(name: &str) -> Self {
        let temp = tempfile::Builder::new()
            .prefix("mkpage-fixture-")
            .tempdir()
            .expect("temporary directory");
        let fixture = Path::new("tests/fixtures").join(name);
        let source = temp.path().join("source");
        copy_tree(&fixture.join("source"), &source);

        Self {
            output: temp.path().join("output"),
            golden: fixture.join("golden"),
            temp,
            source,
        }
    }

    pub fn request(&self) -> BuildRequest {
        BuildRequest {
            source_dir: self.source.clone(),
            output_dir: self.output.clone(),
            profile: BuildProfile::production(calendar_date(2026, 8, 4)),
            keyboard_runtime_enabled: false,
            site: Default::default(),
        }
    }
}

pub fn assert_golden_tree(actual: &Path, expected: &Path) {
    let actual = read_tree(actual);
    let expected_tree = read_tree(expected);

    if std::env::var("MKPAGE_UPDATE_GOLDENS").ok().as_deref() == Some("1") {
        write_tree(expected, &actual);
        return;
    }

    let changed = actual
        .keys()
        .chain(expected_tree.keys())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|path| actual.get(*path) != expected_tree.get(*path))
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        expected_tree,
        "golden mismatch in: {}",
        changed.join(", ")
    );
}

pub fn read_tree(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    collect_tree(root, root, &mut files);
    files
}

fn collect_tree(root: &Path, current: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
    if !current.exists() {
        return;
    }
    for entry in fs::read_dir(current).expect("read fixture tree") {
        let path = entry.expect("fixture entry").path();
        if path.is_dir() {
            collect_tree(root, &path, files);
        } else {
            let bytes = fs::read(&path).expect("read fixture file");
            files.insert(
                path.strip_prefix(root)
                    .expect("relative fixture path")
                    .to_path_buf(),
                normalize_bytes(bytes),
            );
        }
    }
}

fn normalize_bytes(bytes: Vec<u8>) -> Vec<u8> {
    String::from_utf8(bytes)
        .map(|text| text.replace("\r\n", "\n").into_bytes())
        .unwrap_or_else(|error| error.into_bytes())
}

fn write_tree(root: &Path, files: &BTreeMap<PathBuf, Vec<u8>>) {
    for (path, contents) in files {
        let destination = root.join(path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).expect("create golden parent");
        }
        fs::write(destination, contents).expect("write reviewed golden file");
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create fixture destination");
    for entry in fs::read_dir(source).expect("read immutable fixture") {
        let entry = entry.expect("fixture entry");
        let target = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy fixture file");
        }
    }
}
