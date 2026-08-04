---
adr: 0004
title: Establish deterministic golden testing
status: Accepted
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0002-define-the-v0-1-product-contract.md, adr/0003-bootstrap-the-rust-cli.md]
tags: [core, testing, deterministic]
---

# ADR 0004 — Establish deterministic golden testing

## Context

mkpage needs executable specifications before compiler capabilities grow.

## Capability statement

Feature work is verified by building isolated fixture sites through the public
library entry point and comparing normalized output against reviewed golden files.

## User stories / scenarios

- As an implementer, I can extend a fixture instead of inventing an isolated integration test.
- As a reviewer, I can see every intentional output change in checked-in golden files.
- As a maintainer, I get deterministic results independent of host locale, timezone, home directory, Node tooling, or test ordering.

## Acceptance criteria

1. Fixtures are immutable inputs under `tests/fixtures` and run in OS-provided temporary directories.
2. A minimal fixture builds through the public library entry point and compares with checked-in golden output.
3. Repeated builds produce byte-identical normalized output.
4. Golden mismatches identify affected paths and invalid fixtures assert error code, message, and source path.
5. Golden updates require an explicit environment flag; normal tests never rewrite snapshots.
6. Contributor documentation explains fixture structure and golden updates.

## Out of scope

- Browser tests, performance tests, and speculative feature fixtures.

## Open questions

- None.

## References

- https://github.com/delaudio/mkpage/issues/3
- adr/0003-bootstrap-the-rust-cli.md

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted deterministic golden-test foundation. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized autonomous issue delivery |
