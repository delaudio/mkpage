---
adr: 0023
title: Dogfood mkpage by migrating personal site federicodelgaudio.com and cut v0.1 release candidate
status: Implemented
date: 2026-08-06
owner: default-agent
depends-on:
  - adr/0016-define-safe-project-scaffolding-starter.md
  - adr/0021-add-conformance-and-quality-release-gates.md
  - adr/0022-publish-cross-platform-binaries-and-distribution.md
tags: [dogfood, release, migration, v0.1, product]
---

# ADR 0023 — Dogfood mkpage by migrating personal site federicodelgaudio.com and cut v0.1 release candidate

## Context

The final release readiness gate for mkpage v0.1 requires validating the compiler against real-world personal site requirements (federicodelgaudio.com) with an unmodified release binary and documenting production deployment, migration findings, and v0.1 release notes.

## Capability statement

mkpage ships:

- Dogfood migration guide and site verification harness (`docs/dogfood-migration.md`).
- Release notes for v0.1 (`docs/release-notes-v0.1.md`) detailing capabilities, non-goals, and known limitations.
- Comprehensive end-to-end release candidate verification.

## User stories / scenarios

- As a maintainer, I can verify that `mkpage` builds complex portfolios with project showcases, technical writing, keyboard navigation, static search, feeds, and sitemaps without custom binary patches.
- As a reader, accessing `federicodelgaudio.com` provides terminal-inspired design, accessibility conformance, and instant keyboard search (`/`).

## Acceptance criteria

1. Personal site structure and content routes build cleanly with `mkpage build`.
2. No-JavaScript, mobile, keyboard, accessibility, and sharing metadata checks pass.
3. Production deployment, rollback procedure, and dogfood findings are documented in `docs/dogfood-migration.md`.
4. Release notes for v0.1 (`docs/release-notes-v0.1.md`) record capabilities, non-goals, and migration guidance.

## Out of scope

- Automatic git tag publication to remote without maintainer approval.

## References

- https://github.com/delaudio/mkpage/issues/21
- https://github.com/delaudio/mkpage/issues/22

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-06 | r1 | default-agent | Initial decision for dogfood site migration and v0.1 release candidate notes. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-06 | Approved dogfood migration and v0.1 release candidate |
