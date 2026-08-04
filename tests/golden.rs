mod support;

use std::path::Path;

use mkpage::{
    compiler::{BuildRequest, build_site},
    error::AppError,
};
use support::{Fixture, assert_golden_tree};

#[test]
fn minimal_fixture_matches_checked_in_output() {
    let fixture = Fixture::copy("minimal");
    let report = build_site(&fixture.request()).expect("minimal fixture should build");

    assert_eq!(
        report.generated_files,
        vec![std::path::PathBuf::from("index.html")]
    );
    assert_eq!(report.page_count, 1);
    assert_eq!(report.asset_count, 0);
    assert_eq!(report.output_dir, fixture.output);
    assert_golden_tree(&fixture.output, &fixture.golden);
}

#[test]
fn repeated_builds_are_byte_identical() {
    let fixture = Fixture::copy("deterministic");
    build_site(&fixture.request()).expect("first build should succeed");
    let first = support::read_tree(&fixture.output);

    build_site(&fixture.request()).expect("second build should succeed");
    assert_eq!(first, support::read_tree(&fixture.output));
}

#[test]
fn malformed_fixture_reports_code_message_and_source_path() {
    let fixture = Fixture::copy("malformed");
    let error = build_site(&fixture.request()).expect_err("fixture should fail");

    assert_eq!(error.code(), "E301");
    assert!(error.to_string().contains("reserved !invalid! marker"));
    let AppError::InvalidFixture { path, .. } = error else {
        panic!("malformed fixture must retain its source path");
    };
    assert!(path.ends_with(std::path::Path::new("content").join("index.md")));
}

#[test]
fn output_path_traversal_is_rejected() {
    let fixture = Fixture::copy("output-path-traversal");
    let error = build_site(&BuildRequest {
        source_dir: fixture.source,
        output_dir: fixture.temp.path().join("escape").join("..").join(".."),
        profile: mkpage::page::BuildProfile::production(mkpage::page::calendar_date(2026, 8, 4)),
    })
    .expect_err("traversal should fail");

    assert!(matches!(error, AppError::OutputPathTraversal { .. }));
    assert_eq!(error.code(), "E202");
}

#[test]
fn golden_updates_require_an_explicit_environment_flag() {
    assert_ne!(
        std::env::var("MKPAGE_UPDATE_GOLDENS").ok().as_deref(),
        Some("1")
    );
    assert!(Path::new("tests/fixtures/minimal/golden/index.html").exists());
}

#[test]
fn stale_managed_files_are_removed_but_unmanaged_files_remain() {
    let fixture = Fixture::copy("minimal");
    let static_root = fixture.source.join("static");
    std::fs::create_dir_all(&static_root).unwrap();
    std::fs::write(static_root.join("old.css"), "old").unwrap();
    build_site(&fixture.request()).unwrap();
    assert!(fixture.output.join("old.css").exists());
    std::fs::remove_file(static_root.join("old.css")).unwrap();
    std::fs::write(fixture.output.join("notes.txt"), "unmanaged").unwrap();
    build_site(&fixture.request()).unwrap();
    assert!(!fixture.output.join("old.css").exists());
    assert!(fixture.output.join("notes.txt").exists());
}
