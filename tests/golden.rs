mod support;

use std::path::Path;

use mkpage::{
    compiler::{BuildRequest, build_site},
    config::TrailingSlash,
    error::AppError,
};
use serde_json::Value;
use support::{Fixture, assert_golden_tree};

#[test]
fn builds_all_markdown_content_routes() {
    let temp = tempfile::tempdir().expect("temp");
    let source = temp.path().join("source");
    let content = source.join("content");
    let layouts = source.join("layouts");
    let static_root = source.join("static");
    std::fs::create_dir_all(&content).expect("content");
    std::fs::create_dir_all(&layouts).expect("layouts");
    std::fs::create_dir_all(content.join("projects")).expect("projects");
    std::fs::create_dir_all(static_root.join("css")).expect("static-root");

    std::fs::write(content.join("index.md"), b"# Home\n").expect("index");
    std::fs::write(
        content.join("about.md"),
        b"+++\ntitle = \"About\"\n+++\n# About\n",
    )
    .expect("about");
    std::fs::write(
        content.join("projects/index.md"),
        b"+++\ntitle = \"Projects\"\n+++\n# Projects\n",
    )
    .expect("projects index");
    std::fs::write(
        content.join("projects/mkpage.md"),
        b"+++\ntitle = \"mkpage\"\n+++\n# Project\n",
    )
    .expect("project page");
    std::fs::write(layouts.join("page.html"), "{{ content | safe }}").expect("layout");
    std::fs::write(static_root.join("css/site.css"), "body{}\n").expect("css");

    let output = temp.path().join("public");
    let request = BuildRequest {
        source_dir: source,
        output_dir: output.clone(),
        profile: mkpage::page::BuildProfile::production(mkpage::page::calendar_date(2026, 8, 4)),
        keyboard_runtime_enabled: false,
        site: Default::default(),
    };

    let report = build_site(&request).expect("build should render all pages");
    assert_eq!(report.page_count, 4);
    assert!(
        report
            .generated_files
            .contains(&Path::new("index.html").to_path_buf())
    );
    assert!(
        report
            .generated_files
            .contains(&Path::new("about/index.html").to_path_buf())
    );
    assert!(
        report
            .generated_files
            .contains(&Path::new("projects/index.html").to_path_buf())
    );
    assert!(
        report
            .generated_files
            .contains(&Path::new("projects/mkpage/index.html").to_path_buf())
    );
    assert!(report.asset_count >= 1);

    let rendered =
        std::fs::read_to_string(output.join("projects/mkpage/index.html")).expect("page");
    assert!(rendered.contains("<h1 id=\"project\">Project</h1>"));
    assert!(output.join("css/site.css").is_file());
}

#[test]
fn keyboard_runtime_rejects_static_collisions() {
    let temp = tempfile::tempdir().expect("temp");
    let source = temp.path().join("source");
    let content = source.join("content");
    let layouts = source.join("layouts");
    let static_root = source.join("static");
    std::fs::create_dir_all(&content).expect("content");
    std::fs::create_dir_all(&layouts).expect("layouts");
    std::fs::create_dir_all(static_root.join("js")).expect("static root");
    std::fs::write(content.join("index.md"), b"# Home\n").expect("index");
    std::fs::write(layouts.join("page.html"), "{{ content | safe }}").expect("layout");
    std::fs::write(
        static_root.join("js/mkpage-keyboard-v1.js"),
        "console.log('collision')\n",
    )
    .expect("collision asset");

    let request = BuildRequest {
        source_dir: source,
        output_dir: temp.path().join("public"),
        profile: mkpage::page::BuildProfile::production(mkpage::page::calendar_date(2026, 8, 4)),
        keyboard_runtime_enabled: true,
        site: Default::default(),
    };

    let error = build_site(&request).expect_err("runtime collision should fail");
    assert!(matches!(error, AppError::StaticAssetCollision { .. }));
}

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
        keyboard_runtime_enabled: false,
        site: Default::default(),
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

#[test]
fn site_artifacts_are_created_only_when_enabled() {
    let fixture = Fixture::copy("minimal");
    let mut request = fixture.request();
    request.site.include_metadata = false;
    request.site.include_feed = false;
    request.site.include_sitemap = false;
    let report = build_site(&request).unwrap();
    assert_eq!(
        report.generated_files,
        vec![std::path::PathBuf::from("index.html")]
    );
    assert!(!request.output_dir.join("metadata.json").exists());
    assert!(!request.output_dir.join("feed.xml").exists());
    assert!(!request.output_dir.join("sitemap.xml").exists());

    assert!(!request.output_dir.join("search_index.json").exists());

    request.site.include_metadata = true;
    request.site.include_feed = true;
    request.site.include_sitemap = true;
    request.site.include_search = true;
    request.site.base_url = Some("https://example.com".to_string());
    request.site.trailing_slash = TrailingSlash::Always;
    let report = build_site(&request).unwrap();

    assert!(
        report
            .generated_files
            .contains(&Path::new("metadata.json").to_path_buf())
    );
    assert!(
        report
            .generated_files
            .contains(&Path::new("search_index.json").to_path_buf())
    );
    assert!(
        report
            .generated_files
            .contains(&Path::new("feed.xml").to_path_buf())
    );
    assert!(
        report
            .generated_files
            .contains(&Path::new("sitemap.xml").to_path_buf())
    );
    assert!(report.output_dir.join("metadata.json").is_file());
    assert!(report.output_dir.join("search_index.json").is_file());
    assert!(report.output_dir.join("feed.xml").is_file());
    assert!(report.output_dir.join("sitemap.xml").is_file());

    let metadata: Value =
        serde_json::from_slice(&std::fs::read(report.output_dir.join("metadata.json")).unwrap())
            .unwrap();
    let search_index: Value = serde_json::from_slice(
        &std::fs::read(report.output_dir.join("search_index.json")).unwrap(),
    )
    .unwrap();
    let feed = std::fs::read_to_string(report.output_dir.join("feed.xml")).unwrap();
    let sitemap = std::fs::read_to_string(report.output_dir.join("sitemap.xml")).unwrap();

    assert_eq!(
        metadata["base_url"],
        serde_json::json!("https://example.com")
    );
    assert_eq!(metadata["trailing_slash"], serde_json::json!("always"));
    let pages = metadata["pages"].as_array().unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0]["route"], "/");
    assert_eq!(pages[0]["path"], "index.html");
    assert_eq!(pages[0]["title"], serde_json::Value::Null);

    assert_eq!(search_index["version"], "1");
    let entries = search_index["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["url"], "/");
    assert!(entries[0]["content"].is_string());

    assert!(feed.contains("<link>https://example.com/</link>"));
    assert!(feed.contains("<item>"));
    assert!(feed.contains("<title>mkpage</title>"));
    assert!(sitemap.contains("<loc>https://example.com/</loc>"));
}

#[test]
fn theme_fixture_uses_terminal_reference_styling_contract() {
    let fixture = Fixture::copy("theme");
    let report = build_site(&fixture.request()).expect("theme fixture should build");

    assert_eq!(report.generated_files.len(), 2);
    assert_eq!(report.page_count, 1);
    assert_eq!(report.asset_count, 1);
    assert!(fixture.output.join("css/site.css").is_file());
    assert!(fixture.output.join("index.html").is_file());
    assert_golden_tree(&fixture.output, &fixture.golden);
}
