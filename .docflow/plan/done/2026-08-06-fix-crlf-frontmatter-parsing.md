# Fix CRLF frontmatter parsing on Windows

## Owning ADRs

- `adr/0007-define-typed-page-frontmatter.md`

## Scope

1. Accept both `\n` and `\r\n` line endings for the opening `+++` frontmatter
   delimiter in `src/page.rs`.
2. Locate the closing `+++` delimiter by scanning for a start-of-line match
   followed by `\r\n`, `\n`, or end-of-input, instead of a fixed `"\n+++\n"`
   pattern.
3. Add a regression test (`crlf_frontmatter_is_parsed_correctly`) covering a
   full CRLF-delimited frontmatter block.

## Why

Windows CI (`windows-latest`) checks out repository content with
`core.autocrlf` converting `\n` to `\r\n`. The starter content shipped by
`mkpage init` was failing `default_starter_passes_accessibility_html_and_seo_conformance`
on Windows because the frontmatter parser only recognized a bare `\n`
delimiter, producing a false "malformed frontmatter delimiter" diagnostic on
otherwise valid input. This violated the deterministic, cross-platform build
guarantee in AGENTS.md.

## Exit criteria

1. `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-targets --all-features` passes clean.
2. `default_starter_passes_accessibility_html_and_seo_conformance` passes
   regardless of line-ending style used to check out the repository.
