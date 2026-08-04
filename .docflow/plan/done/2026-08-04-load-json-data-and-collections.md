# Load JSON data and collections

## Owning ADRs

- `adr/0010-load-json-data-and-collections.md`

## Scope

Implement JSON data loading, collection manifests/routes, template context,
diagnostics, tests, and documentation.

## Exit criteria

1. ADR 0010 criteria have focused coverage.
2. Collection values render through the normal template engine.

## Dependencies

- `adr/0010-load-json-data-and-collections.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/9

## Shipped

Shipped at HEAD `e2dc3be2a9cc2e45b9d49dd34b7f571048d17a1f` after local format,
Clippy, test, and release-build gates plus green GitHub Actions on Linux,
macOS, and Windows.
