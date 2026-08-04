# mkpage v0.1 product contract

## Promise

mkpage is a Rust static-site generator for terminal-minded websites. It compiles
Markdown and declarative layouts into semantic, accessible HTML and CSS, with
optional keyboard-first enhancements that never make the core site depend on
JavaScript.

**Tagline:** Build terminal-minded sites, ship the web.

**README-ready description:** mkpage is a Rust static-site generator for
terminal-minded websites: Markdown and declarative layouts compile to semantic,
accessible HTML and CSS with optional keyboard-first enhancement.

## Who it is for

mkpage is for developers and writers who like the clarity, density, and
keyboard-oriented interaction of terminal software but need a fast, portable,
and inclusive website.

Representative uses:

- a personal site or writing archive with a terminal-inspired information
  architecture;
- project documentation that benefits from compact navigation and keyboard
  shortcuts;
- a product, tool, or changelog site whose identity draws on command-line and
  TUI conventions.

## Core position

**Terminal-minded** means informed by terminal software: clear hierarchy,
compact typography, visible focus, deliberate keyboard flows, textual feedback,
and composable panels. It does not mean reproducing a terminal in a browser.

**TUI-first** means that terminal interaction patterns inspire the authoring and
design vocabulary. A mkpage site is still a web document: its core is semantic
HTML, responsive CSS, normal links, and native browser behavior.

mkpage is neither a terminal emulator nor a renderer for Ratatui
`Buffer<Cell>` values. Native TUIs and generated websites may share concepts,
but they do not share one layout definition unchanged.

## v0.1 contract

### Inputs

- a project configuration file;
- Markdown pages with front matter;
- static assets;
- declarative site, collection, layout, widget, and theme configuration.

### Outputs

- a deployable directory of static HTML, CSS, assets, feeds, and metadata;
- clean URLs and predictable file paths;
- diagnostic output for invalid source, configuration, routing, and links.

### Core concepts

| Term | Meaning |
|---|---|
| Site | One buildable mkpage project and its configuration. |
| Page | A single routable document generated from source content. |
| Section | A hierarchy node that groups related pages. |
| Collection | A named, queryable set of pages such as posts or notes. |
| Layout | A declarative arrangement of semantic regions. |
| Widget | A reusable, semantic interface pattern such as a menu or panel. |
| Theme | CSS tokens and visual rules that style a site or widget. |
| Enhancement | Optional browser behavior layered over working HTML. |

### User experience guarantees

- Generated pages use semantic HTML and support assistive technology.
- Core content and navigation remain usable with JavaScript disabled.
- Keyboard, mouse, and touch users can reach equivalent core interactions.
- Layouts remain usable across supported viewport sizes.
- JavaScript may enhance navigation, focus management, or a command palette;
  it must not be required to read or navigate normal content.

### Runtime and build expectations

- The generator is a Rust CLI and its core build is Node-free.
- Normal pages do not require WebAssembly.
- Generated sites need only a standards-compliant browser and static hosting.
- v0.1 targets macOS, Linux, and Windows for the CLI and modern evergreen
  browsers for generated output.

### Command surface

v0.1 reserves four primary commands:

- `mkpage init` creates a safe starter project.
- `mkpage build` generates the static site.
- `mkpage dev` watches source and provides local development feedback.
- `mkpage serve` serves a generated directory locally.

Exact flags, defaults, and configuration syntax remain implementation decisions.

## Relationship to adjacent tools

- **Minuto:** an earlier, separate project; mkpage is not a compatibility rewrite.
- **Zola:** a useful static-site-generator reference point, not a feature-parity
  target.
- **Ratatui:** an inspiration for terminal interaction language, not a DOM
  rendering target.
- **Ratzilla:** related experimentation around terminal-style web interfaces;
  mkpage remains an HTML/CSS static-site generator.

## Explicit v0.1 non-goals

- Directly render a Ratatui `Buffer<Cell>` into the DOM.
- Provide a terminal emulator or PTY.
- Reuse one layout definition unchanged across native TUI and web targets.
- Require WebAssembly for normal pages.
- Provide server-side rendering or a persistent application server.
- Build a plugin marketplace, visual editor, or CMS.
- Match the feature set of Zola, Astro, or Hugo.

## Open architectural questions

- What declarative layout syntax will authors write?
- Which Markdown parser, template mechanism, and configuration format will the
  Rust implementation adopt?
- Which keyboard enhancements are essential for v0.1 and which belong later?
- How should themes expose customization without compromising semantic output?
