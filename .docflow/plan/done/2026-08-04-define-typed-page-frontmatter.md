# Define typed page frontmatter

## Owning ADRs

- `adr/0007-define-typed-page-frontmatter.md`

## Scope

Implement TOML frontmatter parsing, the typed page model, deterministic build
profiles, diagnostics, build integration, tests, and author documentation.

## Exit criteria

1. Each ADR 0007 acceptance criterion has focused unit or fixture coverage.
2. The fixture build consumes validated metadata and excludes frontmatter body
   delimiters from rendered output.

## Dependencies

- `adr/0007-define-typed-page-frontmatter.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/6

## Shipped

Shipped at HEAD `f441bb31ffda712f9d1666222acc6aa5f73fe600` after local format,
Clippy, test, and release-build gates plus green GitHub Actions on Linux,
macOS, and Windows.
