---
adr: 0012
title: Plan builds before output mutation
status: Accepted
date: 2026-08-04
owner: default-agent
depends-on: [adr/0006-define-safe-content-routing.md, adr/0011-copy-static-assets-without-node.md]
tags: [core, build, manifest, safety]
---

# ADR 0012 — Plan builds before output mutation

## Context

The build must combine pages and assets without partial or unsafe output state.

## Capability statement

mkpage validates and plans all generated files before writing a controlled
output tree, then writes a deterministic manifest used for safe stale cleanup.

## User stories / scenarios

- A failed input validation leaves a prior successful output untouched.
- Repeated builds remove stale managed files but retain unmanaged files.

## Acceptance criteria

1. Build plans detect collisions before writes.
2. A manifest records relative path, owner, kind, size, and stable hash.
3. Only manifest-managed stale files are removed.
4. Unsafe output/clean targets are rejected.

## Out of scope

- Incremental caching and cross-process atomic replacement guarantees.

## Open questions

- Platform-specific atomic directory swaps can be added after baseline staging.

## References

- https://github.com/delaudio/mkpage/issues/11

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted planned build contract. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
