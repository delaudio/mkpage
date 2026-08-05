# Implement responsive terminal theme system

## Owning ADRs

- `adr/0014-define-responsive-terminal-theme-system.md`

## Scope

Define and publish a terminal-inspired, accessible, and responsive default
theme reference (`examples/theme/site.css`) that styles Mkpage’s semantic widget
classes with override-friendly CSS variables and baseline accessibility rules.

## Exit criteria

1. A terminal theme stylesheet exists under `examples/theme/site.css`.
2. The stylesheet defines documented design tokens for color, typography,
   spacing, and motion behavior.
3. The stylesheet includes responsive behavior for split/stack layouts and print
   overrides.
4. The theme file includes explicit a11y defaults for focus visibility,
   reduced-motion, and contrast-minded defaults.
5. At least one tested fixture demonstrates that the shipped theme asset is copied
   through static-site output as expected.

## Dependencies

- `adr/0001-record-architecture-decisions.md`
- `adr/0011-copy-static-assets-without-node.md`
- `adr/0013-define-semantic-tui-widgets.md`

## GitHub issue

- https://github.com/delaudio/mkpage/issues/13

## Delivery notes

- No JavaScript assumptions should be introduced in this issue.
- Responsive behavior must be CSS-only and avoid fixed-grid-only layouts.

## Shipped

Shipped with terminal theme fixture and token coverage in:
- `examples/theme/site.css`
- `examples/theme/README.md`
- `tests/fixtures/theme/`
- `tests/theme.rs`

Shipped at HEAD `dd6854e` after local format, clippy, and full test gates.
