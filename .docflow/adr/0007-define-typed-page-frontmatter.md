---
adr: 0007
title: Define typed page frontmatter
status: Implemented
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0005-define-project-discovery-and-configuration.md, adr/0006-define-safe-content-routing.md]
tags: [content, frontmatter, safety]
---

# ADR 0007 — Define typed page frontmatter

## Context

Renderers need a complete, validated page model rather than ad-hoc metadata.
The initial implementation must also make publication decisions stable across
timezones and testable without the host clock.

## Capability statement

mkpage parses an optional TOML frontmatter block delimited by `+++` into a
typed page model. Reserved fields are validated before a page reaches a
renderer; user fields live only below `extra`. A caller-supplied build profile
determines draft and future-date visibility.

## User stories / scenarios

- As an author, I can omit frontmatter and still receive a valid page model.
- As an author, I receive a source-aware error for malformed metadata.
- As a publisher, I can reproduce production visibility with a fixed calendar
  date and never depend on local timezone conversion.

## Acceptance criteria

1. TOML `+++` frontmatter parses to a typed page model with reserved and
   extension metadata separated.
2. Date-only values remain calendar dates and build profiles make draft and
   future-date visibility deterministic.
3. Invalid delimiters, Unicode, types, and fields report stable,
   source-aware diagnostics.
4. Production omits drafts and future pages; development includes drafts with
   an explicit renderer marker.
5. Build code receives only fully validated page metadata.

## Out of scope

- YAML compatibility, slug routing overrides, templates, Markdown rendering,
  and publication-time CLI flags.

## Open questions

- Future support for YAML can be added only as a separately documented input
  compatibility capability.

## References

- https://github.com/delaudio/mkpage/issues/6

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted typed TOML frontmatter contract. |
| 2026-08-04 | r2 | default-agent | Implemented and shipped typed frontmatter. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
