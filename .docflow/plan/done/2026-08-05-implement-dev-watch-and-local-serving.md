# Implement live development server and local preview serve

## Owning ADRs

- `adr/0017-define-dev-server-and-live-rebuild.md`

## Scope

Implement `mkpage dev` as a local watch-and-rebuild loop and `mkpage serve` as a
simple static HTTP preview command.

## Exit criteria

1. `mkpage serve` serves files from resolved project output with a fallback to
   `index.html` for directory requests.
2. `mkpage dev` builds on startup and then rebuilds when tracked files change.
3. A minimal command-line interface for host/port is available.
4. Both commands fail fast when project/output is unavailable.
5. Tests cover path mapping and file change detection behavior.

## Dependencies

- `issues/15` and earlier scaffold baseline for init/build outputs.

## GitHub issue

- https://github.com/delaudio/mkpage/issues/16

## Shipped

Shipped and validated with:
- `cargo fmt --all`
- `cargo test --all-targets --all-features`
- `cargo clippy --all-targets --all-features`
- `cargo build --release`
- `mkpage dev` and `mkpage serve` behavior covered by unit and CLI tests
