# Add accessibility, HTML, SEO, payload, and output-safety conformance release gates

## Owning ADRs

- `adr/0021-add-conformance-and-quality-release-gates.md`

## Scope

1. Create `tests/conformance.rs` test suite covering:
   - Accessibility & HTML conformance (valid HTML5 doctype, lang attribute, meta viewport, landmark structures, alt text, ARIA attributes).
   - Link & Route integrity (internal links, sitemap XML schema, RSS feed XML format, unique route check).
   - Payload budget limits (CSS asset size <= 50KB, keyboard runtime JS <= 20KB, starter output HTML size limits).
   - Security & Output Safety (path traversal rejection, symlink boundary checks, absolute path leak prevention, secret scanning).
2. Document conformance commands in `docs/` and verify local execution.

## Exit criteria

1. All conformance tests pass under `cargo test`.
2. `cargo fmt --check && cargo clippy -- -D warnings && cargo test` passes clean.
