# mkpage

Build terminal-minded sites, ship the web.

mkpage is a Rust static-site generator that compiles Markdown and declarative
layouts into semantic, accessible HTML and CSS with optional keyboard-first
enhancement.

## Status

v0.1 is under active development. See [the product contract](docs/product.md)
for scope and non-goals.

## Development

mkpage requires stable Rust. Run the quality gate locally:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
```

The command surface currently reserves `init`, `build`, `dev`, and `serve`.

See [configuration](docs/configuration.md) for `mkpage.toml`, path safety, and
command precedence.
