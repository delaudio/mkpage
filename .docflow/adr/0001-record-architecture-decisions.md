---
adr: 0001
title: Record architecture decisions as ADRs
status: Implemented
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: []
tags: [process, conventions]
---

# ADR 0001 — Record architecture decisions as ADRs

## Context

mkpage needs its significant product and architectural choices to remain discoverable, traceable, and durable rather than living only in chat threads or individual memory.

## Capability statement

The repository records significant capabilities as numbered ADRs under `.docflow/adr/`. The catalogue is the source of truth implementation is expected to match, and the status lifecycle drives the implementation queue. `CONVENTIONS.md` defines the operative authoring, delivery, and audit rules.

## User stories / scenarios

- As a contributor, I can find the reason for a significant decision in the repository.
- As a maintainer, I can trace a decision to its implementation work and commit.
- As a new agent, I can learn the project workflow from the catalogue and handoff files.

## Acceptance criteria

1. Significant decisions are recorded as numbered ADRs following `.docflow/CONVENTIONS.md`.
2. The ADR catalogue is maintained as the source of truth for implemented behavior.
3. ADR authoring, lifecycle transitions, and the plan queue follow `.docflow/CONVENTIONS.md`.

## Out of scope

- Defining individual product capabilities or implementation details.

## Open questions

- None.

## References

- `.docflow/CONVENTIONS.md`
- https://adr.github.io/

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Adopted the documentation-led, ADR-driven method. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Confirmed in chat |
