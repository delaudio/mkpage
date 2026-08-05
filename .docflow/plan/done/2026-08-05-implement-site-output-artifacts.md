# Implement optional site metadata/feed/sitemap outputs

## Owning ADRs

- `adr/0001-record-architecture-decisions.md` (documentation-led evolution and delivery discipline).

## Scope

Add optional generation of static site metadata/artifacts to the build pipeline:

- `metadata.json`
- `feed.xml`
- `sitemap.xml`

Artifacts are controlled by site configuration and are not enabled by default.

## Exit criteria

1. Add `include_metadata`, `include_feed`, `include_sitemap` to `[site]`.
2. Generate optional outputs only when enabled.
3. Use `base_url` and `trailing_slash` in generated artifact values.
4. Ensure generated outputs are represented in generated-files manifest.
5. Ensure default build behavior remains unchanged unless flags are enabled.

## Delivery notes

- Added metadata capture from parsed page metadata.
- Kept feed/sitemap output XML-compatible and deterministic.
- Kept build behavior deterministic and Node-free.

## Shipped

- Added site-context injection to templates (`site.base_url`, `site.trailing_slash`).
- Added generator helpers and tests for optional artifact creation.
- Updated configuration docs for new flags.
- Validation passed with `cargo fmt --check`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features`.
