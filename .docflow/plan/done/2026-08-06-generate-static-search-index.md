# Generate static search index and integrate with command palette

## Owning ADRs

- `adr/0020-generate-static-search-index-and-palette-integration.md`

## Scope

1. Add `include_search` configuration option under `[site]` in `src/config.rs`.
2. Add `search_index.json` generation in `src/compiler.rs` when `include_search` is `true`.
3. Extract search entries (route, title, description, section, tags, headings, content text) from public page metadata and Markdown rendering.
4. Enhance `keyboard-runtime.js` to support command palette search with lazy index fetching, query filtering, result ranking (title > excerpt/content), ARIA live updates, and keyboard/pointer selection.
5. Add unit and golden tests for search index serialization, ranking, and runtime script behavior.

## Exit criteria

1. `cargo test` passes all new and existing tests.
2. `search_index.json` is generated deterministically when `include_search = true`.
3. Output manifest contains `search_index.json`.
4. `cargo fmt --check && cargo clippy -- -D warnings && cargo test` passes clean.
