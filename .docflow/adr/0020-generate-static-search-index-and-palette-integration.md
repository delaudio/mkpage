---
adr: 0020
title: Generate static search index and integrate with command palette
status: Implemented
date: 2026-08-06
owner: default-agent
depends-on:
  - adr/0012-plan-builds-before-output-mutation.md
  - adr/0015-define-progressive-keyboard-enhancements.md
tags: [content, search, web-ui, enhancements, javascript]
---

# ADR 0020 — Generate static search index and integrate with command palette

## Context

mkpage site visitors require serverless content discovery. Static content and documentation sites need fast title, tag, and excerpt search accessible via the terminal-minded command palette (`/`), without depending on third-party SaaS search providers or Node build steps.

## Capability statement

mkpage generates an optional versioned static search index (`search_index.json`) during build and enriches the command palette enhancement runtime to load and query it lazily.

## User stories / scenarios

- As a reader, pressing `/` or using the command palette allows me to search site content in real time.
- As a reader using keyboard navigation, I can cycle search results with `j`/`k` and arrows and press `Enter` to navigate.
- As a reader using assistive technology, search result count updates are announced politely via ARIA live regions.
- As a maintainer, search indexing can be enabled/disabled via `include_search` under `[site]` in site configuration.
- As a site owner, draft and unlisted pages are excluded from the public search index.

## Acceptance criteria

1. Search index schema is versioned (`version: "1"`) and includes title, description/excerpt, route URL, tags, section, headings, and normalized content text.
2. Drafts, private pages, and unlisted outputs are excluded from search index generation.
3. Build outputs `search_index.json` deterministically when `include_search = true` in site configuration.
4. Output manifest includes `search_index.json` with accurate size and SHA256.
5. Command palette runtime lazily loads `search_index.json` when search is activated.
6. Search ranking prioritizes exact/prefix title matches over excerpt/content body matches.
7. Keyboard (`j`/`k`, arrows, `Enter`, `Esc`), pointer, and touch navigation are fully supported.
8. Empty search states and network/load errors are handled gracefully without breaking palette UI.

## Out of scope

- Hosted/external search services.
- Vector or semantic embeddings.
- Full-text fuzzy regex backends in browser runtime.

## References

- https://github.com/delaudio/mkpage/issues/18

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-06 | r1 | default-agent | Initial decision for static search index and command palette search integration. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-06 | Approved search contract ADR |
