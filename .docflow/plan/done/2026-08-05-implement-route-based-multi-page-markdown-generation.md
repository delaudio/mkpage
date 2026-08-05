# Implement route-based multi-page markdown build

## Owning ADRs

- `adr/0019-implement-route-based-multi-page-markdown-generation.md`

## Scope

Refactor build execution from single-page output to route-based output discovery and deterministic, clean-URL HTML emission for all markdown content pages.

## Exit criteria

1. Build discovers all markdown files under `source/content` using the routing subsystem.
2. Build emits one HTML page per source content path using the derived clean URL `candidate.output` path.
3. Draft/future filtering is preserved and applied from existing `BuildProfile`.
4. Static assets and enhancement runtime emission remain compatible and collision-safe.
5. Golden tests validate multi-page generation and runtime/static collision behavior.
6. `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets --all-features` pass.

## Dependencies

- `issues/15` (safe project scaffolding and baseline build pipeline)
- Routing and manifest conventions defined in existing ADRs

## GitHub issue

- https://github.com/delaudio/mkpage/issues/19

## Delivery notes

- Route derivation now comes from `routing::discover` to avoid duplicated path logic in compiler.
- Runtime emission remains optional and is validated against static asset collisions.
- Existing golden coverage was extended with route-mapping assertions and stale-output checks.

## Shipped

Shipped with commit `HEAD` on 2026-08-05 after:

- `cargo fmt`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`
