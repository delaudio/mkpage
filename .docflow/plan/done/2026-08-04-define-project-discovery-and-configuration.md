# Define project discovery and configuration

## Owning ADRs

- `adr/0005-define-project-discovery-and-configuration.md`

## Scope

Implement the canonical configuration model, discovery/CLI precedence, absolute
path resolution, safety validation, diagnostics, tests, and documentation.

## Exit criteria

1. ADR 0005 criteria 1–4 are covered by model, discovery, and diagnostic tests.
2. ADR 0005 criterion 5 is covered by path-safety tests before writes.
3. ADR 0005 criterion 6 is described in contributor documentation and available
   through verbose build diagnostics.

## Dependencies

- `adr/0005-define-project-discovery-and-configuration.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/4

## Shipped

Shipped at HEAD `f468526` on 2026-08-04. CI run `30899729611` passed on Linux, macOS, and Windows.
