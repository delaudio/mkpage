---
adr: 0010
title: Load JSON data and collections
status: Accepted
date: 2026-08-04
owner: default-agent
depends-on: [adr/0005-define-project-discovery-and-configuration.md, adr/0006-define-safe-content-routing.md, adr/0009-render-pages-with-minijinja.md]
tags: [content, data, collections]
---

# ADR 0010 — Load JSON data and collections

## Context

Sites need repeatable local data and generated pages without embedding records
inside templates or adding a query language.

## Capability statement

mkpage loads deterministic nested JSON data and plans array/object collections
from a small manifest. Slugs and output patterns are validated before rendering;
collection items render through the normal bounded template engine.

## User stories / scenarios

- Authors can keep projects in `data/projects.json` and render one page each.
- Invalid sources, patterns, and duplicate slugs fail before output mutation.

## Acceptance criteria

1. Nested JSON keys and collection order are deterministic.
2. Array and object sources yield validated collection items.
3. Items render with route, original key, item data, and site data context.
4. Missing/scalar sources, duplicate slugs, and unsafe patterns fail early.

## Out of scope

- YAML, remote data, scripts, query languages, and pagination.

## Open questions

- YAML remains a future compatibility decision when a concrete site needs it.

## References

- https://github.com/delaudio/mkpage/issues/9

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted JSON data and collection contract. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
