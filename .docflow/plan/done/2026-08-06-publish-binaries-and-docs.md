# Publish cross-platform binaries, crate metadata, shell completions, and installation documentation

## Owning ADRs

- `adr/0022-publish-cross-platform-binaries-and-distribution.md`

## Scope

1. Add `clap_complete` dependency to `Cargo.toml` and update package metadata.
2. Add `Completions` subcommand to `mkpage` CLI in `src/cli.rs` and `src/lib.rs`.
3. Add GitHub Actions workflow `.github/workflows/release.yml` for multi-platform binary compilation and release.
4. Add `docs/installation.md` for installation guide and shell completions usage.
5. Add unit tests for shell completions CLI command in `tests/cli.rs`.

## Exit criteria

1. `cargo test` passes all tests including completion generation tests.
2. `mkpage completions bash` outputs valid Bash completion script to stdout.
3. `cargo fmt --check && cargo clippy -- -D warnings && cargo test` passes clean.
