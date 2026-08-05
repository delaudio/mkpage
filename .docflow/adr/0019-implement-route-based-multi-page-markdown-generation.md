adr: 0019
title: Implement route-based multi-page markdown generation
status: Implemented
date: 2026-08-05
owner: default-agent
depends-on:
  - adr/0006-define-safe-content-routing.md
  - adr/0012-plan-builds-before-output-mutation.md
tags:
  - compiler
  - routing
  - build
  - testing
---

# ADR 0019 — Implement route-based multi-page markdown generation

## Context

`mkpage` previously generated a single HTML page from a default entry content path. That model prevents a true site structure and conflicts with TUI-like portfolio narratives that need `/projects`, `/projects/<slug>` and similar clean URLs.

## Capability statement

The build pipeline must discover all Markdown pages under `content/`, derive deterministic clean-URL output paths, and write one page per source while preserving existing safety, draft, and layout behavior.

## User stories / scenarios

- As an author, I can add `content/about.md` and get `/about/` in the built output.
- As an author, I can add nested content like `content/projects/mkpage.md` and get `/projects/mkpage/`.
- As an author, I can still use draft/future visibility rules and not emit disallowed pages.
- As a maintainer, I can run deterministic builds and have golden tests that prove route mapping for all Markdown files.

## Acceptance criteria

1. The compiler discovers Markdown candidates via the routing subsystem and ignores non-markdown files.
2. Each route is rendered through the normal page pipeline and emitted as `index.html` in a clean URL.
3. Draft/future filtering continues to use `BuildProfile` visibility rules.
4. Existing static asset copying and runtime emission behavior remains unchanged and collision-safe.
5. Build artifact manifest and stale cleanup continue to track managed files.
6. Golden tests validate multi-page builds and static/runtime path collision handling.

## Out of scope

- Implementing Markdown collections, search indexes, pagination, or tag archives.
- Introducing server-side page fallback strategies beyond static clean URLs.

## Open questions

- None.

## References

- https://github.com/delaudio/mkpage/issues/19

## Revision History

- Implemented on 2026-08-05.

## Approvals

- Owner: default-agent
