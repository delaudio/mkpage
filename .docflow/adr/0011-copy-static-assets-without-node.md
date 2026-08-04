---
adr: 0011
title: Copy static assets without Node
status: Implemented
date: 2026-08-04
owner: default-agent
depends-on: [adr/0005-define-project-discovery-and-configuration.md, adr/0006-define-safe-content-routing.md]
tags: [core, assets, css, safety]
---

# ADR 0011 — Copy static assets without Node

## Context

Sites need styles and binary assets without introducing a JavaScript toolchain.

## Capability statement

mkpage deterministically copies non-symlink static files byte-for-byte under
the output root, rejects collisions with generated pages, and uses plain CSS
under `static/css/` as the Node-free v0.1 workflow.

## User stories / scenarios

- Authors can ship nested CSS, images, and binaries unchanged.
- Unsafe or colliding assets fail before becoming output.

## Acceptance criteria

1. Copy order and asset bytes are deterministic.
2. Symlinks and output escapes are rejected or ignored by policy.
3. Assets never overwrite generated HTML.
4. The CSS workflow is documented and Node-free.

## Out of scope

- Bundling, minification, hashing, image optimization, and external hooks.

## Open questions

- Rust-native CSS minification can be evaluated after a production CSS corpus exists.

## References

- https://github.com/delaudio/mkpage/issues/10

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted Node-free static asset contract. |
| 2026-08-04 | r2 | default-agent | Implemented and shipped Node-free static assets. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
