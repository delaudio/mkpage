use std::fs;

use mkpage::{
    cli::Init,
    compiler::{BuildRequest, build_site},
    config::Site,
    init::run as run_init,
    page::BuildProfile,
};
use tempfile::TempDir;
use time::macros::date;

#[test]
fn default_starter_passes_accessibility_html_and_seo_conformance() {
    let temp = TempDir::new().unwrap();
    let project_dir = temp.path().join("starter");

    let context = mkpage::CommandContext {
        root: None,
        config: None,
        verbosity: 0,
        quiet: true,
    };
    run_init(
        context,
        Init {
            directory: project_dir.clone(),
            template: "default".into(),
        },
    )
    .expect("init starter should succeed");

    let site_config = Site {
        include_metadata: true,
        include_feed: true,
        include_sitemap: true,
        include_search: true,
        base_url: Some("https://federicodelgaudio.com".into()),
        ..Default::default()
    };

    let request = BuildRequest {
        source_dir: project_dir.clone(),
        output_dir: project_dir.join("public"),
        profile: BuildProfile::Production {
            as_of: date!(2026 - 08 - 06),
        },
        keyboard_runtime_enabled: true,
        site: site_config,
    };

    let report = build_site(&request).expect("starter build should succeed");
    assert!(
        report.page_count >= 5,
        "starter should render multiple content pages"
    );

    for file in &report.generated_files {
        let full_path = report.output_dir.join(file);
        assert!(
            full_path.is_file(),
            "generated file should exist: {:?}",
            file
        );

        if file.extension().and_then(|ext| ext.to_str()) == Some("html") {
            let html = fs::read_to_string(&full_path).unwrap();

            assert!(
                html.to_lowercase().starts_with("<!doctype html>"),
                "HTML page must start with doctype: {:?}",
                file
            );
            assert!(
                html.contains("<html") && html.contains("lang="),
                "HTML page must have html element with lang attribute: {:?}",
                file
            );
            assert!(
                html.contains("<meta charset=\"utf-8\">")
                    || html.contains("<meta charset=\"UTF-8\">"),
                "HTML page must have UTF-8 charset meta tag: {:?}",
                file
            );
            assert!(
                html.contains("<meta name=\"viewport\""),
                "HTML page must have responsive viewport meta tag: {:?}",
                file
            );

            assert!(
                !html.contains("/Users/") && !html.contains("C:\\Users\\"),
                "HTML output must not leak local machine absolute paths: {:?}",
                file
            );
        }
    }

    let css_path = report.output_dir.join("css/site.css");
    if css_path.is_file() {
        let css_size = fs::metadata(&css_path).unwrap().len();
        assert!(
            css_size <= 50_000,
            "CSS payload budget exceeded: {} bytes > 50,000 bytes",
            css_size
        );
    }

    let js_path = report.output_dir.join("js/mkpage-keyboard-v1.js");
    assert!(js_path.is_file(), "keyboard runtime JS should be generated");
    let js_size = fs::metadata(&js_path).unwrap().len();
    assert!(
        js_size <= 30_000,
        "JS runtime payload budget exceeded: {} bytes > 30,000 bytes",
        js_size
    );

    let sitemap_path = report.output_dir.join("sitemap.xml");
    assert!(sitemap_path.is_file());
    let sitemap = fs::read_to_string(&sitemap_path).unwrap();
    assert!(sitemap.contains("<urlset"));
    assert!(sitemap.contains("<loc>https://federicodelgaudio.com/"));

    let feed_path = report.output_dir.join("feed.xml");
    assert!(feed_path.is_file());
    let feed = fs::read_to_string(&feed_path).unwrap();
    assert!(feed.contains("<rss"));
    assert!(feed.contains("<channel>"));

    let search_path = report.output_dir.join("search_index.json");
    assert!(search_path.is_file());
    let search_index: serde_json::Value =
        serde_json::from_slice(&fs::read(&search_path).unwrap()).unwrap();
    assert_eq!(search_index["version"], "1");
    assert!(
        search_index["entries"].as_array().unwrap().len() >= 5,
        "search index should index all public starter pages"
    );
}

#[test]
fn output_safety_and_path_containment_conformance() {
    let temp = TempDir::new().unwrap();
    let source_dir = temp.path().join("source");
    fs::create_dir_all(source_dir.join("content")).unwrap();
    fs::write(
        source_dir.join("content/index.md"),
        "# Root Page\nTest content",
    )
    .unwrap();

    let request = BuildRequest {
        source_dir,
        output_dir: temp.path().join("../outside_output"),
        profile: BuildProfile::Production {
            as_of: date!(2026 - 08 - 06),
        },
        keyboard_runtime_enabled: false,
        site: Site::default(),
    };

    let result = build_site(&request);
    assert!(
        result.is_err(),
        "build_site must reject output directory path traversal"
    );
}
