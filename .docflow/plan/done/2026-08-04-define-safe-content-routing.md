# Define safe content routing

## Owning ADRs

- `adr/0006-define-safe-content-routing.md`

## Scope

Implement deterministic discovery, route/output types, collision and traversal
validation, tests, and golden fixtures for nested content.

## Exit criteria

1. ADR 0006 criteria 1–4 are covered by pure routing tests.
2. ADR 0006 criterion 5 is covered by fixture additions.

## Dependencies

- `adr/0006-define-safe-content-routing.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/5

## Shipped

Shipped at HEAD `c6dd72024bb959b2d4eb9dec5cb9507ebf80360f` after local format,
Clippy, test, and release-build gates plus green GitHub Actions on Linux,
macOS, and Windows.
