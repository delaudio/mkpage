# Dogfood mkpage by migrating federicodelgaudio.com and cut the v0.1 release

## Owning ADRs

- `adr/0023-dogfood-migration-and-v0-1-release-candidate.md`

## Scope

1. Create `docs/dogfood-migration.md` documenting personal site migration verification, route mapping, deployment, and rollback procedures.
2. Create `docs/release-notes-v0.1.md` documenting v0.1 capabilities, non-goals, and known limitations.
3. Verify end-to-end build reproducibly with default starter and fixtures.

## Exit criteria

1. Documentation exists and is complete.
2. All conformance tests and unit tests pass cleanly.
3. `cargo fmt --check && cargo clippy -- -D warnings && cargo test` passes clean.
