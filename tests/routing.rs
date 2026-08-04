use std::path::PathBuf;

use mkpage::{
    error::AppError,
    routing::{Route, SourceRoute, discover, validate_collisions, validate_static_collisions},
};
use tempfile::tempdir;

#[test]
fn maps_documented_content_paths_to_clean_urls() {
    let root = PathBuf::from("content");
    for (path, expected) in [
        ("index.md", "/"),
        ("about.md", "/about/"),
        ("projects/index.md", "/projects/"),
        ("projects/mkpage.md", "/projects/mkpage/"),
    ] {
        assert_eq!(
            Route::from_source(&root, &root.join(path))
                .unwrap()
                .as_str(),
            expected
        );
    }
}

#[test]
fn fixture_discovery_is_sorted_and_skips_private_candidates() {
    let source = PathBuf::from("tests/fixtures/routing-nested/source/content");
    let temp = tempdir().unwrap();
    let output = temp.path().join("public");
    let routes = discover(&source, &output).unwrap();
    assert_eq!(
        routes
            .iter()
            .map(|route| route.route.as_str())
            .collect::<Vec<_>>(),
        vec!["/", "/projects/", "/projects/mkpage/", "/z/"]
    );
}

#[test]
fn case_only_collisions_are_rejected_on_every_platform() {
    let source = PathBuf::from("content");
    let output = PathBuf::from("public");
    let lower_route = Route::from_source(&source, &source.join("a.md")).unwrap();
    let lower_output = lower_route.output_path(&output).unwrap();
    let upper_route = Route::from_source(&source, &source.join("A.md")).unwrap();
    let upper_output = upper_route.output_path(&output).unwrap();
    assert!(matches!(
        validate_collisions(&[
            SourceRoute {
                source: source.join("a.md"),
                route: lower_route,
                output: lower_output,
            },
            SourceRoute {
                source: source.join("A.md"),
                route: upper_route,
                output: upper_output,
            },
        ]),
        Err(AppError::RouteCollision { .. })
    ));
}

#[test]
fn output_paths_stay_under_output_root() {
    let route = Route::from_source(
        std::path::Path::new("content"),
        std::path::Path::new("content/projects/mkpage.md"),
    )
    .unwrap();
    assert_eq!(
        route
            .output_path(std::path::Path::new("public"))
            .unwrap()
            .as_path(),
        std::path::Path::new("public/projects/mkpage/index.html")
    );
}

#[test]
fn unsafe_route_candidates_report_the_candidate_and_reason() {
    let error = Route::from_source(
        std::path::Path::new("content"),
        std::path::Path::new("content/%2e%2e/secret.md"),
    )
    .unwrap_err();
    assert!(matches!(error, AppError::InvalidRoute { .. }));
    assert!(error.to_string().contains("%2e%2e"));
    assert!(error.to_string().contains("unsafe"));
}

#[test]
fn static_assets_cannot_overwrite_generated_output() {
    let source = PathBuf::from("tests/fixtures/routing-static-collision/source/content");
    let static_root = PathBuf::from("tests/fixtures/routing-static-collision/static");
    let page_source = source.join("about.md");
    let temp = tempdir().unwrap();
    let output = temp.path().join("public");
    let route = Route::from_source(&source, &page_source).unwrap();
    let page = SourceRoute {
        source: page_source,
        output: route.output_path(&output).unwrap(),
        route,
    };
    assert!(matches!(
        validate_static_collisions(&[page], &static_root, &output),
        Err(AppError::StaticAssetCollision { .. })
    ));
}
