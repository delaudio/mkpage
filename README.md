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

Optional keyboard-first behavior is documented in `docs/enhancements.md` and is
enabled with the generated runtime asset.

See [configuration](docs/configuration.md) for `mkpage.toml`, path safety, and
command precedence.

Quick start:

```sh
mkpage init .
mkpage dev .
mkpage build --root .
mkpage serve --port 3000
```

`mkpage dev` builds once, serves `public` on `127.0.0.1:3000` by default, and
watches `content`, `layouts`, `data`, `static`, and `mkpage.toml` for rebuilds.

`mkpage serve` serves an existing output directory; pass `--host` / `--port` to
adjust the preview endpoint.

See [docs/init.md](docs/init.md) for starter behavior and next-step guidance.
See [docs/dev.md](docs/dev.md) for command details.
