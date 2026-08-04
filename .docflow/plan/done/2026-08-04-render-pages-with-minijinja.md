# Render pages with MiniJinja

## Owning ADRs

- `adr/0009-render-pages-with-minijinja.md`

## Scope

Implement MiniJinja layouts/partials, stable bounded context, diagnostics,
fixtures, author documentation, and build integration.

## Exit criteria

1. Each ADR 0009 criterion has focused test or golden coverage.
2. The build emits its page through a configured layout rather than reference HTML.

## Dependencies

- `adr/0009-render-pages-with-minijinja.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/8

## Shipped

Shipped at HEAD `879706fa6dd8fbf1cf718dab353339c06960fa88` after local format,
Clippy, test, and release-build gates plus green GitHub Actions on Linux,
macOS, and Windows.
