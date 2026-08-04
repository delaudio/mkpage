# Bootstrap the Rust CLI

## Owning ADRs

- `adr/0003-bootstrap-the-rust-cli.md`

## Scope

Create the single Rust crate, CLI dispatch boundary, application errors, logging,
tests, repository documentation, and three-platform CI. Do not implement static
site compilation behavior.

## Exit criteria

1. ADR 0003 criteria 1–4 pass through CLI help and automated tests.
2. ADR 0003 criterion 5 is present in `.github/workflows/ci.yml`.
3. ADR 0003 criteria 6–7 are satisfied by repository documentation and public
   library boundaries.

## Dependencies

- `adr/0003-bootstrap-the-rust-cli.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/2

## Shipped

Shipped at HEAD `3c7e139` on 2026-08-04. CI run `30898133118` passed on Linux, macOS, and Windows.
