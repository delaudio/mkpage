# Improve local preview server response behavior

## Owning ADRs

- `adr/0018-improve-local-preview-server-response-behavior.md`

## Scope

Refine `mkpage serve`/`dev` local HTTP behavior with improved response
semantics and regression tests.

## Exit criteria

1. `mkpage serve` returns `Allow: GET, HEAD` for non-GET/HEAD methods.
2. `HEAD` responses for files and errors are header-only.
3. Invalid path cases and safe path checks remain enforced.
4. New tests exercise request handling for `serve`/`dev` responses (including methods
   and `HEAD` semantics).
5. Documentation for dev/serve is updated if needed.

## Dependencies

- `issues/16` and existing dev/serve implementation.

## GitHub issue

- https://github.com/delaudio/mkpage/issues/17

## Shipped

- Added response `Allow` headers for non-GET/HEAD methods and ensured header-only
  `HEAD` responses.
- Hardened invalid-path handling and added regression tests for request handling.
- Validation passed with `cargo fmt --check`, `cargo clippy --all-targets --all-features`,
  `cargo test --all-targets --all-features`, `cargo build --release`.

Shipped at this commit.
