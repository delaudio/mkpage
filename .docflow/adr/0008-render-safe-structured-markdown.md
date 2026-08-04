---
adr: 0008
title: Render safe structured Markdown
status: Implemented
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0007-define-typed-page-frontmatter.md]
tags: [content, markdown, safety]
---

# ADR 0008 — Render safe structured Markdown

## Context

Page bodies need deterministic semantic HTML and metadata for later template,
feed, search, and link-validation capabilities.

## Capability statement

mkpage uses `pulldown-cmark` for a documented CommonMark/GFM subset and returns
rendered HTML together with headings, links, assets, and plain-text summary.
Raw HTML is escaped rather than trusted or claimed as sanitized; unsafe URL
schemes are neutralized before output.

## User stories / scenarios

- As an author, I can write technical Markdown with tables, task lists, code,
  and deterministic heading anchors.
- As a maintainer, I can validate outbound and internal links from structured
  renderer output.
- As a reader, I never receive executable Markdown URL schemes or raw HTML.

## Acceptance criteria

1. The supported parser and Markdown subset are documented and tested.
2. Rendered output, heading IDs, and link metadata are deterministic.
3. Raw HTML is escaped and unsafe URL schemes never become output links.
4. Structured results include heading tree, links, assets, and summary.
5. Code rendering retains a safe accessible fallback with language metadata.

## Out of scope

- Arbitrary embedded HTML, browser-side syntax highlighting, and final
  template/layout composition.

## Open questions

- A full server-side token highlighter may be introduced after the base theme.

## References

- https://github.com/delaudio/mkpage/issues/7

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted structured Markdown rendering contract. |
| 2026-08-04 | r2 | default-agent | Implemented and shipped safe structured Markdown. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
