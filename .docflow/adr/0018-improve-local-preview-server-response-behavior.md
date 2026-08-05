adr: 0018
title: Improve local preview server response behavior
status: Implemented
date: 2026-08-05
owner: default-agent
depends-on:
  - adr/0017-define-dev-server-and-live-rebuild.md
tags:
  - dev
  - serve
  - http
  - testing
---

# ADR 0018 — Improve local preview server response behavior

## Context

`mkpage dev` and `mkpage serve` already provide local HTTP delivery, but response
handling in `serve` is minimal and lacks protocol-level polish needed for stable
integration with tooling and predictable diagnostics.

## Capability statement

Local serving must return more standards-aware responses, preserve method
semantics for `HEAD`, and provide deterministic behavior on unsupported
methods.

## User stories / scenarios

- As a reviewer, I can call `mkpage serve` and receive predictable error
  responses for unsupported methods.
- As an author, I can request `HEAD /` in local preview and receive headers
  without an unnecessary response body.
- As a maintainer, we can validate server behavior via automated tests without
  requiring external network access.

## Acceptance criteria

1. `mkpage serve` returns an HTTP response with `Allow: GET, HEAD` for unsupported
   methods.
2. `HEAD` requests return headers and status codes with zero response body bytes.
3. Invalid request paths still fail safely and remain covered by regression tests.
4. Serve behavior is tested with deterministic request-handler assertions for
   methods and `HEAD` semantics.

## Out of scope

- Runtime changes to the compiler build pipeline.
- Long-lived process management for parallel workers.

## Open questions

- Which additional headers should be added in addition to `Allow` for future
  browser interoperability.

## References

- https://github.com/delaudio/mkpage/issues/17

## Revision History

- Implemented on 2026-08-05.

## Approvals

- Owner: default-agent
