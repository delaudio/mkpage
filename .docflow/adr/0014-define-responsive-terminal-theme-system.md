---
adr: 0014
title: Define responsive terminal theme system
status: Implemented
date: 2026-08-05
owner: default-agent
depends-on:
  - adr/0011-copy-static-assets-without-node.md
  - adr/0013-define-semantic-tui-widgets.md
tags: [theme, css, accessibility, responsiveness, usability]
---

# ADR 0014 — Define responsive terminal theme system

## Context

mkpage’s widgets are semantic and reusable, but authors need a first-party,
terminal-minded visual system that requires no JavaScript and can be customized
without changing Markdown or widget contracts.

## Capability statement

mkpage should provide a reference terminal-inspired theme (and documented token
set) that:

- styles widget classes from the semantic contracts;
- works responsively from narrow to wide viewports;
- exposes semantic, tokenized customization points; and
- includes accessibility defaults for focus, reduced-motion, and print rendering.

## User stories / scenarios

- As an author, I can start with a useful terminal-inspired visual baseline by
  copying a starter CSS file.
- As a reader, I can consume content on narrow, wide, and print contexts with
  readable layout and clear focus affordances.
- As a maintainer, I can override colors, spacing, and density by changing CSS
  custom properties instead of editing widget templates.

## Acceptance criteria

1. A documented starter theme exists in-repo under versioned artifacts.
2. The starter theme includes CSS custom properties for at least typography,
   spacing, focus, color, borders, surface, density, and transitions.
3. The starter theme styles at least the widget classes in ADR 0013 and includes
   responsive behavior for Split-style horizontal layouts.
4. The starter theme includes `prefers-reduced-motion` and `print` adaptations
   that do not depend on JavaScript.
5. Theme behavior is test-covered through repository checks or fixtures and
   static asset copying tests.

## Out of scope

- New JavaScript enhancement layers.
- A CLI theme switcher or runtime compile-time theme pipeline.
- Server-side CSS processing and bundling.

## Open questions

- Which default accent values should target light mode in future versions?
- Should `mkpage init` scaffold theme files in a follow-up issue?

## References

- https://github.com/delaudio/mkpage/issues/13

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-05 | r1 | default-agent | Initial responsive terminal theme decision proposal. |
| 2026-08-05 | r2 | default-agent | Implemented terminal theme reference CSS, fixture, and test coverage. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-05 | Approved implementation path |
