---
adr: 0006
title: Define safe content routing
status: Implemented
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0004-establish-deterministic-golden-testing.md, adr/0005-define-project-discovery-and-configuration.md]
tags: [core, routing, safety]
---

# ADR 0006 — Define safe content routing

## Context

Content paths must become deterministic public routes without escaping source
or output boundaries or producing platform-dependent collisions.

## Capability statement

mkpage discovers supported content deterministically and maps validated relative
paths to directory-style URLs and `index.html` outputs. Route and output path
types remain distinct from raw strings; collisions and traversal fail before writes.

## User stories / scenarios

- As an author, I receive stable clean URLs from normal content paths.
- As a maintainer, I see both owners of a route collision before output changes.
- As a security reviewer, I can prove no route can escape the output root.

## Acceptance criteria

1. Source examples map to documented clean URLs and output paths.
2. Discovery, URL construction, and collision detection are deterministic and pure.
3. Traversal, absolute paths, platform prefixes, and case-only collisions fail with source-aware diagnostics.
4. Validated outputs always remain beneath the configured output root.
5. Nested-content and collision fixtures cover the behavior.

## Out of scope

- Redirects, collection-generated virtual pages, and development-server fallback.

## Open questions

- Slug/frontmatter overrides arrive with the page model.

## References

- https://github.com/delaudio/mkpage/issues/5

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted safe source-routing contract. |
| 2026-08-04 | r2 | default-agent | Implemented and shipped safe source routing. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
