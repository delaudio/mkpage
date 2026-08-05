mod support;

use std::fs;

use mkpage::{compiler::build_site, enhancements};

#[test]
fn keyboard_runtime_is_emitted_only_when_enabled() {
    let fixture = support::Fixture::copy("keyboard");
    let mut request = fixture.request();
    request.keyboard_runtime_enabled = false;

    let disabled = build_site(&request).expect("runtime should be optional");
    assert!(
        !disabled
            .generated_files
            .contains(&"js/mkpage-keyboard-v1.js".into())
    );

    request.keyboard_runtime_enabled = true;
    let enabled = build_site(&request).expect("runtime should generate when enabled");

    let runtime = fixture.output.join(enhancements::KEYBOARD_RUNTIME_PATH);
    assert!(runtime.is_file(), "runtime file should be generated");
    assert!(
        enabled
            .generated_files
            .contains(&enhancements::KEYBOARD_RUNTIME_PATH.into())
    );
    assert!(
        !disabled
            .generated_files
            .contains(&enhancements::KEYBOARD_RUNTIME_PATH.into())
    );
}

#[test]
fn keyboard_runtime_output_contract_includes_progressive_attributes_and_configuration() {
    let fixture = support::Fixture::copy("keyboard");
    let mut request = fixture.request();
    request.keyboard_runtime_enabled = true;
    build_site(&request).expect("keyboard fixture should build");

    let index = fs::read_to_string(request.output_dir.join("index.html")).unwrap();
    let script = fs::read_to_string(request.output_dir.join(enhancements::KEYBOARD_RUNTIME_PATH))
        .expect("runtime file should be readable");

    assert!(index.contains("data-mkpage-route-shortcuts"));
    assert!(index.contains("data-mkpage-widget=\"list\""));
    assert!(index.contains("data-mkpage-enhance=\"keyboard\""));
    assert!(index.contains("/js/mkpage-keyboard-v1.js"));

    assert!(script.contains("j") || script.contains("k"));
    assert!(script.contains(r"/"));
    assert!(script.contains("Route shortcuts"));
    assert!(script.contains("data-mkpage-command-palette"));
}

#[test]
fn keyboard_runtime_is_versioned_and_cacheable_by_filename() {
    let fixture = support::Fixture::copy("keyboard");
    let mut request = fixture.request();
    request.keyboard_runtime_enabled = true;
    build_site(&request).expect("keyboard fixture should build");

    assert!(
        request
            .output_dir
            .join(enhancements::KEYBOARD_RUNTIME_PATH)
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.contains("mkpage-keyboard-v1"))
    );
}
