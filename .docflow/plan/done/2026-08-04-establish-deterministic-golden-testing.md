# Establish deterministic golden testing

## Owning ADRs

- `adr/0004-establish-deterministic-golden-testing.md`

## Scope

Create the isolated fixture, normalization, golden comparison, expected-failure,
and contributor-documentation foundation used by future compiler work.

## Exit criteria

1. ADR 0004 criteria 1–3 are exercised by minimal and repeated-build tests.
2. ADR 0004 criteria 4–5 are exercised by diagnostics and golden-update tests.
3. ADR 0004 criterion 6 is documented in `tests/fixtures/README.md`.

## Dependencies

- `adr/0004-establish-deterministic-golden-testing.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/3

## Shipped

Shipped at HEAD `f46ae51` on 2026-08-04. CI run `30898854931` passed on Linux, macOS, and Windows.
