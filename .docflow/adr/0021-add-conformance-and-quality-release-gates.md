---
adr: 0021
title: Add accessibility, HTML, SEO, payload, and output-safety conformance release gates
status: Implemented
date: 2026-08-06
owner: default-agent
depends-on:
  - adr/0004-establish-deterministic-golden-testing.md
  - adr/0012-plan-builds-before-output-mutation.md
tags: [core, testing, conformance, security, accessibility, quality]
---

# ADR 0021 — Add accessibility, HTML, SEO, payload, and output-safety conformance release gates

## Context

mkpage promises production-grade accessibility, Node-free static compilation, zero leaked local credentials or absolute paths, and strict output-root safety. Automated conformance gates must enforce these invariants systematically during release and CI testing.

## Capability statement

mkpage ships an integrated conformance test suite (`tests/conformance.rs`) that runs under standard `cargo test` and verifies:

- HTML structure, language attributes, viewport tags, unique IDs, and valid XML output.
- Accessibility standards (landmarks, heading levels, alt attributes, ARIA live regions, progressive JS enhancement).
- Link integrity (no broken internal links, anchor tag validation, external scheme safety).
- Payload budgets (strict byte limits on CSS, runtime JS, HTML starter templates).
- Security guards (path traversal prevention, output-root containment, leak scanning for absolute filesystem paths and secrets).

## User stories / scenarios

- As a maintainer, running `cargo test` automatically executes conformance gates without external dependencies or Node.js.
- As a contributor, regressing accessibility markup, output safety, or payload budget triggers an explicit test failure naming the violated invariant.

## Acceptance criteria

1. Automated conformance suite runs under `cargo test` on Linux, macOS, and Windows.
2. Default starter scaffold output passes HTML structure, viewport, language, and accessibility checks.
3. Payload budgets enforce limits for generated CSS, keyboard runtime JS, and HTML templates.
4. Security checks assert path traversal guards, output-root boundary rules, and absence of leaked local absolute paths (`/Users/`, `C:\`, `env:`).
5. Internal link, anchor ID, sitemap XML, and feed RSS formats are validated programmatically.

## Out of scope

- External third-party web auditing services (Lighthouse cloud APIs).
- Browser-based automated Selenium/Playwright testing in default `cargo test` runner.

## References

- https://github.com/delaudio/mkpage/issues/19

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-06 | r1 | default-agent | Initial decision for automated conformance release gates. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-06 | Approved conformance gates ADR |
