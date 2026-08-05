# Implement safe project scaffolding and default portfolio starter

## Owning ADRs

- `adr/0016-define-safe-project-scaffolding-starter.md`
- `adr/0013-define-semantic-tui-widgets.md`
- `adr/0014-define-responsive-terminal-theme-system.md`

## Scope

Implemented `mkpage init` as a safe, deterministic scaffolder and a full default
portfolio/blog starter from built-in assets.

## Exit criteria

1. `mkpage init [directory]` is implemented with explicit template selection.
2. Initialization fails safely on non-empty destination directories.
3. Generated starter includes:
   - home dashboard,
   - project collection + project detail example,
   - writing index + article example,
   - about page,
   - uses page,
   - responsive split layout,
   - status bar and key hints in the starter layout,
   - light/dark demonstration through CSS tokens,
   - one structured data example and at least one author-authored markdown page,
   - no-JS accessible shell.
4. Command summary prints created and skipped paths.
5. `mkpage init demo` creates files usable by `mkpage build --root demo`.
6. `tests` cover: template validation, directory safety, deterministic file graph.

## Dependencies

- `issues/7` through `14`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/15

## Notes

- Merge mode is intentionally not implemented in v0.1; if destination exists and is not
  empty, initialization returns a conflict instead of overwriting.

## Shipped

Shipped and validated with:
- `mkpage init <path> --template default`
- `mkpage build --root <path>`
- local `cargo fmt`, `cargo clippy --all-targets --all-features`,
  `cargo test --all-targets --all-features`, `cargo build --release`

Shipped at HEAD of this commit.
