---
adr: 0005
title: Define project discovery and configuration
status: Accepted
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0002-define-the-v0-1-product-contract.md, adr/0004-establish-deterministic-golden-testing.md]
tags: [core, configuration, safety]
---

# ADR 0005 — Define project discovery and configuration

## Context

Every compiler capability needs one stable way to find a project, resolve
paths, and reject unsafe configuration before output is written.

## Capability statement

mkpage uses one versioned, Serde-backed `mkpage.toml` file. Explicit CLI paths
override project configuration, which overrides documented defaults. Discovery
walks upward from the working directory; resolved paths are absolute internally
and configuration paths are lexical rather than symlink-resolved.

## User stories / scenarios

- As an author in a nested directory, I can build the nearest mkpage project.
- As an automation user, I can select a root or config path explicitly.
- As a maintainer, unsafe source/output relationships fail before writes.

## Acceptance criteria

1. `mkpage.toml` is the only canonical default configuration filename.
2. Discovery, `--root`, and `--config` have tested and documented precedence.
3. Versioned configuration resolves source, layout, data, static, and output paths absolutely.
4. Unknown fields, unsupported versions, invalid relationships, and missing configuration produce source-aware diagnostics.
5. Output cannot be project root, filesystem root, home, or overlap source paths.
6. Defaults, lexical symlink policy, and effective configuration diagnostics are documented and tested.

## Out of scope

- Environment-variable configuration, aliases for old filenames, and compiler content processing.

## Open questions

- Include/exclude pattern semantics will be finalized with source discovery.

## References

- https://github.com/delaudio/mkpage/issues/4
- adr/0004-establish-deterministic-golden-testing.md

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted project-discovery and configuration contract. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
