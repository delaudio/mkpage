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
