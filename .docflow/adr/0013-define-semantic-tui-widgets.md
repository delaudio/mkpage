---
adr: 0013
title: Define semantic TUI widgets
status: Proposed
date: 2026-08-04
owner: default-agent
depends-on: [adr/0009-render-pages-with-minijinja.md]
tags: [content, widgets, accessibility, tui]
---

# ADR 0013 — Define semantic TUI widgets

## Context

mkpage needs a small, recognisable set of terminal-inspired layout primitives
without giving up valid, responsive, accessible HTML.

## Capability statement

mkpage provides documented declarative widget macros that render semantic HTML
and remain useful without JavaScript.

## User stories / scenarios

- As an author, I compose a terminal-minded page without writing Rust or
  reproducing inaccessible visual markup.
- As a reader, I can use links and content with keyboard, pointer, touch,
  print, or no JavaScript.

## Acceptance criteria

1. Screen, Pane, Split, Stack, List, Tree, Table, Tabs, Article, StatusBar,
   KeyHints, and Dialog have a written semantic and responsive contract.
2. Every interactive widget has example rendered HTML and keyboard, pointer,
   and touch paths.
3. Core content and links work without JavaScript and focus order follows
   document order unless documented otherwise.
4. Widget output has no fixed viewport or character-cell requirement and
   documents reduced-motion and print behaviour.
5. Invalid nesting is rejected or explicitly documented, and one complete
   layout uses only the primitive set.

## Out of scope

- Ratatui compatibility, terminal emulation, canvas rendering, and arbitrary
  stateful web components.

## Open questions

- None for the v0.1 macro contract.

## References

- https://github.com/delaudio/mkpage/issues/12

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Initial semantic widget contract. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
