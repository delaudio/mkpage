use mkpage::{
    compiler::{BuildRequest, build_site},
    error::AppError,
    page::{BuildProfile, calendar_date, parse},
};

#[test]
fn missing_frontmatter_produces_a_valid_empty_metadata_model() {
    let page = parse("content/index.md".as_ref(), b"# Hello\n").unwrap();
    assert_eq!(page.body, "# Hello\n");
    assert_eq!(page.metadata.title, None);
    assert!(!page.metadata.draft);
}

#[test]
fn valid_frontmatter_separates_reserved_and_extension_metadata() {
    let page = parse(
        "content/about.md".as_ref(),
        br#"+++
title = "About"
description = "A profile"
date = "2026-08-03"
updated = "2026-08-04"
draft = true
layout = "page"
slug = "whoami"
tags = ["rust", "tui"]
projects = ["mkpage"]
canonical_url = "https://example.test/about/"
social_image = "/og.png"
[extra]
accent = "green"
+++
# About
"#,
    )
    .unwrap();
    assert_eq!(page.metadata.title.as_deref(), Some("About"));
    assert_eq!(page.metadata.date, Some(calendar_date(2026, 8, 3)));
    assert_eq!(page.metadata.extra["accent"].as_str(), Some("green"));
    assert_eq!(page.body, "# About\n");
}

#[test]
fn crlf_frontmatter_is_parsed_correctly() {
    let page = parse(
        "content/crlf.md".as_ref(),
        b"+++\r\ntitle = \"CRLF Test\"\r\ndraft = false\r\n+++\r\n# Hello CRLF\r\n",
    )
    .unwrap();
    assert_eq!(page.metadata.title.as_deref(), Some("CRLF Test"));
    assert_eq!(page.body, "# Hello CRLF\r\n");
}

#[test]
fn malformed_and_unknown_frontmatter_are_source_aware() {
    let error = parse("content/about.md".as_ref(), b"+++\ntitl = 1\n+++\nbody").unwrap_err();
    assert!(matches!(error, AppError::Frontmatter { .. }));
    assert_eq!(error.code(), "E601");
    assert!(error.to_string().contains("content/about.md"));
    assert!(error.to_string().contains("did you mean `title`"));

    let delimiter = parse("content/about.md".as_ref(), b"+++ title = 'bad'").unwrap_err();
    assert!(
        delimiter
            .to_string()
            .contains("malformed frontmatter delimiter")
    );
}

#[test]
fn invalid_unicode_and_date_types_have_stable_diagnostics() {
    let unicode = parse("content/about.md".as_ref(), b"\xff").unwrap_err();
    assert_eq!(unicode.code(), "E601");
    assert!(unicode.to_string().contains("valid UTF-8"));

    let date = parse(
        "content/about.md".as_ref(),
        b"+++\ndate = 2026-08-04\n+++\n",
    )
    .unwrap_err();
    assert!(date.to_string().contains("YYYY-MM-DD string"));
}

#[test]
fn build_profiles_are_timezone_stable_for_drafts_and_future_dates() {
    let draft = parse(
        "content/draft.md".as_ref(),
        b"+++\ndraft = true\n+++\nDraft",
    )
    .unwrap();
    let future = parse(
        "content/future.md".as_ref(),
        b"+++\ndate = \"2026-08-05\"\n+++\nFuture",
    )
    .unwrap();
    let production = BuildProfile::production(calendar_date(2026, 8, 4));
    let development = BuildProfile::development(calendar_date(2026, 8, 4));
    assert!(!production.includes(&draft));
    assert!(!production.includes(&future));
    assert!(development.includes(&draft));
    assert!(development.includes(&future));
    assert!(development.shows_draft_marker(&draft));
}

#[test]
fn build_pipeline_uses_validated_metadata_for_visibility_and_draft_marker() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    std::fs::create_dir_all(source.join("content")).unwrap();
    std::fs::write(
        source.join("content/index.md"),
        b"+++\ndraft = true\n+++\nDraft body\n",
    )
    .unwrap();
    let production = build_site(&BuildRequest {
        source_dir: source.clone(),
        output_dir: temp.path().join("production"),
        profile: BuildProfile::production(calendar_date(2026, 8, 4)),
        keyboard_runtime_enabled: false,
        site: Default::default(),
    })
    .unwrap();
    assert!(production.generated_files.is_empty());
    let development_output = temp.path().join("development");
    build_site(&BuildRequest {
        source_dir: source,
        output_dir: development_output.clone(),
        profile: BuildProfile::development(calendar_date(2026, 8, 4)),
        keyboard_runtime_enabled: false,
        site: Default::default(),
    })
    .unwrap();
    let html = std::fs::read_to_string(development_output.join("index.html")).unwrap();
    assert!(html.contains("data-mkpage-draft=\"true\""));
    assert!(html.contains("Draft body"));
    assert!(!html.contains("+++"));
}
