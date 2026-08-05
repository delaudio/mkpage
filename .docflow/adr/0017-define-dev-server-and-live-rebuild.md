---
adr: 0017
title: Define a live development server and local preview command
status: Implemented
date: 2026-08-05
owner: default-agent
depends-on:
  - adr/0012-plan-builds-before-output-mutation.md
  - adr/0015-define-progressive-keyboard-enhancements.md
tags:
  - dev
  - serve
  - watch
  - live
  - localhost
---

# ADR 0017 — Define a live development server and local preview command

## Context

Static sites need a fast local iteration loop. `mkpage` currently exposes `mkpage build`
but does not yet provide local runtime feedback, live rebuilds, or local preview.

## Capability statement

`mkpage dev` must provide a local watch-and-rebuild loop with immediate local preview,
and `mkpage serve` must serve an already-generated output directory over HTTP.

## User stories / scenarios

- As an author, I can run `mkpage dev` and see changes reflected without manual
  rebuild commands.
- As a reviewer, I can run `mkpage serve` to inspect generated output in a browser.
- As a maintainer, local serving should be reliable and not require external
  runtimes.
- As a developer, live watch should tolerate harmless file-system noise and keep
  rebuilding incrementally.

## Acceptance criteria

1. `mkpage serve` serves static files from resolved project output path.
2. `mkpage dev` performs initial build, then rebuilds when tracked source files
   change.
3. Changes in `content`, `layouts`, `data`, `static`, and `mkpage.toml` trigger
   rebuild.
4. Dev should emit clear status messages and listen on a configurable host/port.
5. `dev`/`serve` are deterministic and return concrete failures when root/output are
   unavailable.

## Open questions

- Whether `dev` should optionally skip serving and only run rebuild loop.
- Poll interval defaults.

## References

- https://github.com/delaudio/mkpage/issues/16
