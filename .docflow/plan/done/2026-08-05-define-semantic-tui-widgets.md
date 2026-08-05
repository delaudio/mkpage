# Define semantic TUI widgets

## Owning ADRs

- `adr/0013-define-semantic-tui-widgets.md`

## Scope

Document the widget contract, provide MiniJinja macros and fixtures for all
widgets in ADR 0013, including the interactive progressive-enhancement
contracts for Tabs and Dialog plus the KeyHints pointer/touch fallback
contract.

## Exit criteria

1. ADR 0013 acceptance criteria have implementation or contract-test evidence.
2. A complete sample layout renders from the supplied primitives.

## Dependencies

- `adr/0009-render-pages-with-minijinja.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/12

## Shipped

Shipped at HEAD `13f4b5952c0cb7f4a3a4f5d9e1b9c4f8f7e6d4a1` after local format, clippy, test, and release-build gates plus local issue 12 closure.
