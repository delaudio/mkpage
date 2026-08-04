# Render safe structured Markdown

## Owning ADRs

- `adr/0008-render-safe-structured-markdown.md`

## Scope

Implement the parser integration, safe rendering policy, structured output,
anchor generation, metadata extraction, fixtures, and documentation.

## Exit criteria

1. Each ADR 0008 acceptance criterion has unit or golden coverage.
2. The build pipeline renders validated page bodies rather than preformatted text.

## Dependencies

- `adr/0008-render-safe-structured-markdown.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/7

## Shipped

Shipped at HEAD `6922d794bc9f978dbbeb7829756d89c90c1d9efd` after local format,
Clippy, test, and release-build gates plus green GitHub Actions on Linux,
macOS, and Windows.
