---
adr: 0002
title: Define the v0.1 product contract
status: Accepted
date: 2026-08-04
owner: default-agent
supersedes:
superseded-by:
depends-on: [adr/0001-record-architecture-decisions.md]
tags: [product, v0.1, scope]
---

# ADR 0002 — Define the v0.1 product contract

## Context

mkpage occupies a space between generic static-site generators, terminal-themed
CSS frameworks, terminal emulators, and browser-hosted Ratatui applications.
Without an explicit product boundary, implementation could drift toward a
Ratatui-to-DOM renderer, a general web framework, or a Zola clone.

## Capability statement

mkpage defines a focused v0.1 contract for compiling Markdown and declarative,
terminal-minded layouts into semantic, accessible static HTML and CSS. The
contract names the supported concepts, user expectations, browser enhancement
model, CLI surface, compatibility expectations, and deliberate non-goals.

## User stories / scenarios

- As a terminal-minded site author, I can understand whether mkpage is the
  right tool before creating a site.
- As an implementer, I can distinguish required v0.1 behavior from features
  intentionally deferred.
- As a user of the generated site, I can use core content and navigation
  without JavaScript and with keyboard, mouse, or touch input.

## Acceptance criteria

1. `docs/product.md` states the product promise, intended users, and at least
   three representative use cases.
2. The document defines terminal-minded, TUI-first, semantic HTML, and the
   v0.1 concepts used by follow-on work.
3. The v0.1 input/output contract, browser and build dependencies, expected
   commands, and platform compatibility are stated.
4. Required capabilities and explicit non-goals are separate and unambiguous.
5. The contract requires semantic HTML, no-JavaScript usability, keyboard /
   mouse / touch parity, and responsive layouts.
6. A README-ready description and tagline are included.
7. Open architectural questions are recorded without deciding them implicitly.

## Out of scope

- A Ratatui buffer-to-DOM renderer, terminal emulator, PTY, or shared native
  TUI/web layout runtime.
- WebAssembly as a normal-page requirement, server-side rendering, or a
  persistent application server.
- A plugin marketplace, visual editor, CMS, or feature parity with Zola, Astro,
  or Hugo.

## Open questions

- Which declarative layout syntax best balances expressiveness and stability?
- Which template engine and Markdown parser best serve the Rust implementation?
- Which keyboard enhancements belong in v0.1 rather than a later release?

## References

- https://github.com/delaudio/mkpage/issues/1
- adr/0001-record-architecture-decisions.md

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-04 | r1 | default-agent | Accepted product contract for v0.1. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-04 | Authorized implementation in chat |
