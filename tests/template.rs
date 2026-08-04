use std::fs;

use mkpage::{markdown::render as render_markdown, page::parse, template::render};
use tempfile::tempdir;

#[test]
fn nested_layouts_escape_values_and_only_trust_rendered_markdown() {
    let temp = tempdir().unwrap();
    let layouts = temp.path().join("layouts");
    fs::create_dir_all(layouts.join("partials")).unwrap();
    fs::write(layouts.join("base.html"), "<html><body>{% include 'partials/header.html' %}{% block body %}{% endblock %}</body></html>").unwrap();
    fs::write(
        layouts.join("partials/header.html"),
        "<header>{{ page.title }}</header>",
    )
    .unwrap();
    fs::write(
        layouts.join("post.html"),
        "{% extends 'base.html' %}{% block body %}<main>{{ content }}</main>{% endblock %}",
    )
    .unwrap();
    let page = parse(
        "content/post.md".as_ref(),
        b"+++\ntitle = \"<unsafe>\"\n+++\n# Hello\n",
    )
    .unwrap();
    let output = render(&layouts, "post", &page, &render_markdown(&page.body)).unwrap();
    assert!(output.contains("&lt;unsafe&gt;"));
    assert!(output.contains("<h1 id=\"hello\">Hello</h1>"));
}

#[test]
fn missing_or_invalid_layouts_name_the_template_source() {
    let temp = tempdir().unwrap();
    let page = parse("content/post.md".as_ref(), b"body").unwrap();
    let missing = render(temp.path(), "missing", &page, &render_markdown(&page.body)).unwrap_err();
    assert_eq!(missing.code(), "E701");
    assert!(missing.to_string().contains("missing.html"));

    fs::write(temp.path().join("broken.html"), "{% if %}").unwrap();
    let broken = render(temp.path(), "broken", &page, &render_markdown(&page.body)).unwrap_err();
    assert!(broken.to_string().contains("broken.html"));
}
