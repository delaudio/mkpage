use std::fs;

#[test]
fn terminal_theme_reference_contains_responsive_and_a11y_contracts() {
    let css = fs::read_to_string("examples/theme/site.css").expect("theme reference should exist");

    for token in [
        "--mk-color-bg",
        "--mk-color-surface",
        "--mk-color-text",
        "--mk-color-text-muted",
        "--mk-font-family",
        "--mk-font-mono",
        "--mk-space",
        "--mk-split-gap",
        "--mk-focus-outline",
    ] {
        assert!(css.contains(token), "missing token {token}");
    }

    assert!(css.contains(".mk-pane"));
    assert!(css.contains(".mk-split"));
    assert!(css.contains(".mk-tabs"));
    assert!(css.contains("prefers-reduced-motion"));
    assert!(css.contains("@media print"));
}
