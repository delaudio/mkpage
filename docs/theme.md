# Theme system and responsive layout tokens

mkpage ships a terminal-inspired, static CSS reference theme:
`examples/theme/site.css`.

Copy this file into your project as `static/css/site.css` and link it from your
layout (for example in `<head>`).

For optional keyboard-first navigation, include `mkpage-keyboard-v1.js` from your
layout when desired; see [`docs/enhancements.md`](enhancements.md).

The theme uses CSS custom properties prefixed with `--mk-` to keep overrides
local, predictable, and script-free.

## Core tokens

| Token | Meaning |
|---|---|
| `--mk-color-bg` | Main page background. |
| `--mk-color-surface` | Surface/background for cards, panels, and bars. |
| `--mk-color-text` | Primary text color. |
| `--mk-color-text-muted` | Secondary text and supporting labels. |
| `--mk-color-border` | Subtle separators and outlines. |
| `--mk-color-focus` | Focus ring color. |
| `--mk-font-family` | Body font stack. |
| `--mk-font-mono` | Mono font for code-like labels and key hints. |
| `--mk-font-size-body` | Base text size. |
| `--mk-font-size-small` | Secondary text size. |
| `--mk-leading` | Line height. |
| `--mk-space` | Base spacing unit. |
| `--mk-shell-max-width` | Max width for wide screens. |
| `--mk-clip-radius` | Corner radius for panels and surfaces. |
| `--mk-focus-outline` | Focus ring thickness / style. |
| `--mk-split-divider` | Divider color between horizontal siblings. |

## Responsive behavior

- `@media (max-width: 64rem)` collapses split panels into vertical flow and
  makes content stack with clearer vertical rhythm.
- Widget blocks should avoid fixed widths and fixed cell metrics; fluid layout is the
  default.
- On print, non-essential chrome is suppressed and surfaces are flattened to improve
  contrast.

## Accessibility defaults

The theme includes explicit defaults for:

- visible focus indicators (`:focus-visible`);
- reduced motion (`@media (prefers-reduced-motion: reduce)`);
- print (`@media print`) layout.

## Extensibility

To override token values, redefine variables in a higher-precedence CSS file in
your project. Start by copying `examples/theme/site.css` to
`static/css/site.css` and edit the variables there.

The widget class map and breakpoints are designed to work with existing contracts in
`docs/widgets.md` without JavaScript.
