use std::fs;

use mkpage::{
    data::{CollectionManifest, collection, load, validate_generated_routes},
    error::AppError,
    template::render_collection,
};
use tempfile::tempdir;

#[test]
fn nested_json_data_loads_under_deterministic_logical_keys() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("projects")).unwrap();
    fs::write(temp.path().join("site.json"), r#"{"name":"mkpage"}"#).unwrap();
    fs::write(
        temp.path().join("projects/items.json"),
        r#"[{"slug":"one"}]"#,
    )
    .unwrap();
    let data = load(temp.path()).unwrap();
    assert_eq!(data["site"]["name"], "mkpage");
    assert_eq!(data["projects.items"][0]["slug"], "one");
}

#[test]
fn array_and_object_collections_have_stable_routes() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("items.json"),
        r#"[{"name":"Hello World"},{"name":"Two"}]"#,
    )
    .unwrap();
    fs::write(
        temp.path().join("projects.json"),
        r#"{"mkpage":{"title":"mkpage"},"minuto":{"title":"minuto"}}"#,
    )
    .unwrap();
    let data = load(temp.path()).unwrap();
    assert_eq!(
        collection(&data, "items", "/work/{slug}/", Some("name"))
            .unwrap()
            .iter()
            .map(|item| item.route.as_str())
            .collect::<Vec<_>>(),
        vec!["/work/hello-world/", "/work/two/"]
    );
    assert_eq!(
        collection(&data, "projects", "/projects/{slug}/", None)
            .unwrap()
            .iter()
            .map(|item| item.slug.as_str())
            .collect::<Vec<_>>(),
        vec!["minuto", "mkpage"]
    );
}

#[test]
fn invalid_collection_sources_slugs_and_patterns_fail_before_rendering() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("items.json"),
        r#"[{"name":"same"},{"name":"same"}]"#,
    )
    .unwrap();
    fs::write(temp.path().join("scalar.json"), "1").unwrap();
    let data = load(temp.path()).unwrap();
    assert!(matches!(
        collection(&data, "missing", "/x/{slug}/", None),
        Err(AppError::Data { .. })
    ));
    assert!(matches!(
        collection(&data, "scalar", "/x/{slug}/", None),
        Err(AppError::Data { .. })
    ));
    assert!(matches!(
        collection(&data, "items", "/x/{slug}/", Some("name")),
        Err(AppError::Data { .. })
    ));
    assert!(matches!(
        collection(&data, "items", "../x/{slug}", Some("name")),
        Err(AppError::Data { .. })
    ));
}

#[test]
fn manifests_and_collection_items_render_through_the_normal_template_engine() {
    let manifest: CollectionManifest = toml::from_str("source = 'projects'\nlayout = 'project'\noutput = '/projects/{slug}/'\nslug_field = 'name'\n").unwrap();
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("projects.json"),
        r#"[{"name":"mkpage","summary":"<safe?>"}]"#,
    )
    .unwrap();
    fs::write(
        temp.path().join("project.html"),
        "{{ collection.route }} {{ item.summary }} {{ data.projects[0].name }}",
    )
    .unwrap();
    let data = load(temp.path()).unwrap();
    let item = collection(
        &data,
        &manifest.source,
        &manifest.output,
        manifest.slug_field.as_deref(),
    )
    .unwrap()
    .remove(0);
    let nested = serde_json::to_value(&data).unwrap();
    let output = render_collection(temp.path(), &manifest.layout, &item, &nested).unwrap();
    assert_eq!(
        output,
        "&#x2f;projects&#x2f;mkpage&#x2f; &lt;safe?&gt; mkpage"
    );
}

#[test]
fn generated_routes_share_case_insensitive_collision_validation() {
    let temp = tempdir().unwrap();
    fs::write(temp.path().join("items.json"), r#"[{"name":"About"}]"#).unwrap();
    let data = load(temp.path()).unwrap();
    let items = collection(&data, "items", "/{slug}/", Some("name")).unwrap();
    assert!(matches!(
        validate_generated_routes(&items, &["/about/".into()]),
        Err(AppError::Data { .. })
    ));
}
