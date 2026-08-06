# Release Notes — mkpage v0.1.0

`mkpage` is a static-site generator for terminal-minded websites. It compiles Markdown, structured TOML/JSON data, MiniJinja templates, and semantic TUI-inspired design components into accessible, deterministic static HTML and CSS.

---

## Core Capabilities (v0.1.0)

- **Deterministic Static Site Generation**: Zero non-determinism, Node-free, fast Rust build pipeline.
- **Terminal Design System & Widgets**: Pre-built responsive CSS design system (`css/site.css`) featuring TUI-inspired widget macros (`panel`, `list`, `table`, `statusBar`, `commandPalette`).
- **Progressive Keyboard Enhancement**: Optional lightweight runtime (`js/mkpage-keyboard-v1.js`) for `j`/`k` navigation, `g`-prefixed route shortcuts, and `/` command palette search. No-JavaScript core content access remains 100% complete.
- **Serverless Search Index**: Build-time deterministic `search_index.json` generation with lazy-loaded command palette search runtime.
- **Discovery Artifacts**: Optional automated `metadata.json`, `feed.xml` (RSS 2.0), and `sitemap.xml` output.
- **Development Experience**: `mkpage dev` with file watch, instant rebuild, local preview server, and SSE live reload. `mkpage init` scaffold starter.
- **Shell Completions**: Native `mkpage completions <shell>` generation for Bash, Zsh, Fish, PowerShell, and Elvish.
- **Conformance & Safety**: Automated WCAG accessibility, HTML5, payload budget, link integrity, and output path containment gates.

---

## Product Non-Goals & Scope Limits

`mkpage` v0.1 explicitly excludes:

- Terminal emulators or Ratatui cell-buffer canvas renderers.
- Node.js or JavaScript build toolchain requirements for user site builds.
- CMS, WYSIWYG visual editor, or dynamic server application state.
- Hosted search service dependencies or vector databases.

---

## Installation & Documentation

- [Installation Guide](installation.md)
- [Configuration Reference](configuration.md)
- [Templates & Widgets](templates.md)
- [Dogfood Migration Guide](dogfood-migration.md)
