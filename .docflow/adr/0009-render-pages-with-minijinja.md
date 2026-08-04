---
adr: 0009
title: Render pages with MiniJinja
status: Accepted
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0004-establish-deterministic-golden-testing.md, adr/0007-define-typed-page-frontmatter.md, adr/0008-render-safe-structured-markdown.md]
tags: [content, templates, safety]
---

# ADR 0009 — Render pages with MiniJinja

## Context

Validated pages require reusable layouts and partials without exposing host
state or trusting ordinary template values as HTML.

## Capability statement

mkpage uses MiniJinja for deterministic layouts, nested partials, inheritance,
and a documented stable context. HTML autoescaping is enabled; only Markdown
rendered by mkpage is marked safe through the single `content` context field.

## User stories / scenarios

- As an author, I can reuse nested partials and layouts with escaped metadata.
- As a maintainer, I receive source-aware layout diagnostics.
- As a security reviewer, I can verify templates never receive host paths,
  environment variables, or arbitrary safe HTML.

## Acceptance criteria

1. Layouts, inheritance, and nested partials render deterministically.
2. Public site/page/content/data/build context is documented and bounded.
3. Only generated Markdown is trusted HTML; ordinary values are escaped.
4. Missing templates and syntax errors identify their source templates.
5. Fixtures cover includes, nesting, escaping, and failures.

## Out of scope

- Plugins, arbitrary Rust execution, host environment access, and widgets.

## Open questions

- Additional documented filters can be introduced when a concrete template
  capability requires them.

## References

- https://github.com/delaudio/mkpage/issues/8

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted MiniJinja template contract. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
