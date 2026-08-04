use mkpage::markdown::render;

#[test]
fn renders_documented_gfm_syntax_and_structured_metadata() {
    let rendered = render(
        "# Caffè ☕\n\n## Caffè ☕\n\n- [x] done\n\n| A | B |\n| - | - |\n| 1 | 2 |\n\n```rust\nlet x = 1 < 2;\n```\n\n[internal](/about/) [external](https://example.test) ![logo](/logo.svg)\n",
    );
    assert!(rendered.html.contains("<h1 id=\"caffè\">Caffè ☕</h1>"));
    assert!(rendered.html.contains("<h2 id=\"caffè-2\">Caffè ☕</h2>"));
    assert!(rendered.html.contains("type=\"checkbox\""));
    assert!(rendered.html.contains("<table>"));
    assert!(rendered.html.contains("class=\"language-rust\""));
    assert!(rendered.html.contains("1 &lt; 2"));
    assert_eq!(rendered.headings.len(), 2);
    assert!(
        rendered
            .links
            .iter()
            .any(|link| link.internal && link.url == "/about/")
    );
    assert!(
        rendered
            .links
            .iter()
            .any(|link| link.outbound && link.url == "https://example.test")
    );
    assert_eq!(rendered.assets, vec!["/logo.svg"]);
}

#[test]
fn escapes_raw_html_and_neutralizes_unsafe_links() {
    let rendered = render("<script>alert(1)</script> [bad](javascript:alert(1))");
    assert!(rendered.html.contains("&lt;script&gt;"));
    assert!(!rendered.html.contains("<script>"));
    assert!(!rendered.html.contains("javascript:"));
    assert!(rendered.links.is_empty());
}
