---
adr: 0003
title: Bootstrap the Rust CLI
status: Accepted
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0002-define-the-v0-1-product-contract.md]
tags: [core, cli, quality]
---

# ADR 0003 — Bootstrap the Rust CLI

## Context

mkpage needs a small, production-quality executable foundation before compiler
features can be safely introduced.

## Capability statement

mkpage provides a single Rust crate with a stable command surface, testable
library handlers, structured application errors, configurable diagnostics,
repository standards, and cross-platform CI quality gates.

## User stories / scenarios

- As a contributor, I can discover every planned top-level command through help.
- As an implementer, I can test command parsing and handler dispatch without
  terminating the process.
- As a maintainer, I can rely on CI to enforce formatting, linting, tests, and
  release builds on supported operating systems.

## Acceptance criteria

1. `mkpage --help` documents `init`, `build`, `dev`, and `serve`.
2. Every command has a separate library handler and common options are parsed.
3. Application errors use one type and map to stable, documented exit codes.
4. Tests cover parsing, invalid flag combinations, global options, and error codes.
5. CI runs format, Clippy, tests, and release builds on Linux, macOS, and Windows.
6. The repository contains a README, MIT license, contribution guidance, and security policy.
7. The public library API does not expose CLI implementation dependencies.

## Out of scope

- Compiler behavior, a multi-crate workspace, crate publishing, releases, and
  async runtime dependencies.

## Open questions

- None for the initial CLI boundary.

## References

- https://github.com/delaudio/mkpage/issues/2
- adr/0002-define-the-v0-1-product-contract.md

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted the initial Rust CLI boundary. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
