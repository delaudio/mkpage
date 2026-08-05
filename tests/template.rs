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
    let output = render(&layouts, "post", &page, &render_markdown(&page.body), false).unwrap();
    assert!(output.contains("&lt;unsafe&gt;"));
    assert!(output.contains("<h1 id=\"hello\">Hello</h1>"));
}

#[test]
fn missing_or_invalid_layouts_name_the_template_source() {
    let temp = tempdir().unwrap();
    let page = parse("content/post.md".as_ref(), b"body").unwrap();
    let missing = render(
        temp.path(),
        "missing",
        &page,
        &render_markdown(&page.body),
        false,
    )
    .unwrap_err();
    assert_eq!(missing.code(), "E701");
    assert!(missing.to_string().contains("missing.html"));

    fs::write(temp.path().join("broken.html"), "{% if %}").unwrap();
    let broken = render(
        temp.path(),
        "broken",
        &page,
        &render_markdown(&page.body),
        false,
    )
    .unwrap_err();
    assert!(broken.to_string().contains("broken.html"));
}

#[test]
fn widget_macros_render_a_semantic_complete_layout() {
    let temp = tempdir().unwrap();
    let layouts = temp.path().join("layouts");
    fs::create_dir_all(&layouts).unwrap();
    fs::copy(
        "examples/widgets/widgets.jinja",
        layouts.join("widgets.jinja"),
    )
    .unwrap();
    fs::write(layouts.join("page.html"), "{% from 'widgets.jinja' import screen, pane, split, stack, list, tree, table, tabs, article, status_bar, key_hints, dialog %}{% call screen('Site', 'ready') %}{% call pane('Projects') %}{% call split() %}{% call stack() %}{% call list('Projects', true) %}<li><a href='/projects'>Projects</a></li>{% endcall %}{% call pane('Nested', '', 3) %}<p>Nested body</p>{% endcall %}{% endcall %}{% endcall %}{% endcall %}{% call tree('Navigation') %}<li><a href='/'>Home</a></li>{% endcall %}{% call table('Data') %}<tr><th>Key</th></tr>{% endcall %}{% call tabs() %}<li><a href='#main'>Main</a></li>{% endcall %}{% call article('main') %}<p>Text</p>{% endcall %}{% call status_bar() %}ready{% endcall %}{% call key_hints() %}<li><a href='/'>Home</a></li>{% endcall %}{% call dialog('Help') %}<p>Help</p>{% endcall %}{% endcall %}").unwrap();
    let page = parse("content/index.md".as_ref(), b"body").unwrap();
    let output = render(&layouts, "page", &page, &render_markdown(&page.body), false).unwrap();
    assert!(output.contains("<section class=\"mk-pane\""));
    assert!(output.contains("<h3>Nested</h3>") && output.contains("Nested body"),);
    assert!(output.contains("<nav class=\"mk-key-hints\""));
    assert!(output.contains("<a href='/projects'>Projects</a>"));
    assert!(output.contains("<ol class=\"mk-list\""));
    assert!(output.contains("<details class=\"mk-dialog\""));
    assert!(output.contains("<article class=\"mk-article\" id=\"main\">"));
    assert!(output.contains("<div class=\"mk-status-bar\" role=\"status\">ready</div>"));
    for class in [
        "mk-split",
        "mk-list",
        "mk-tree",
        "mk-table",
        "mk-tabs",
        "mk-article",
        "mk-status-bar",
        "mk-dialog",
    ] {
        assert!(output.contains(class), "missing {class}");
    }
}
