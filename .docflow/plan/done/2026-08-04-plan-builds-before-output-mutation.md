# Plan builds before output mutation

## Owning ADRs

- `adr/0012-plan-builds-before-output-mutation.md`

## Scope

Implement in-memory planning, deterministic manifest, stale managed cleanup,
safe output validation, summary, diagnostics, and end-to-end fixtures.

## Exit criteria

1. ADR 0012 criteria are covered by integration tests.
2. A failed build never writes a success manifest.

## Dependencies

- `adr/0012-plan-builds-before-output-mutation.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/11

## Shipped

Shipped at HEAD `9dfd568ae33c2ff97e5c880d337ea8e7e58bdeee` after local format,
Clippy, test, and release-build gates plus green GitHub Actions on Linux,
macOS, and Windows.
