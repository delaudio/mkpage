---
adr: 0016
title: Define safe project scaffolding for the default starter
status: Implemented
date: 2026-08-05
owner: default-agent
supersedes:
superseded-by:
depends-on:
  - adr/0002-define-the-v0-1-product-contract.md
  - adr/0003-bootstrap-the-rust-cli.md
  - adr/0013-define-semantic-tui-widgets.md
  - adr/0014-define-responsive-terminal-theme-system.md
tags:
  - scaffolding
  - starter
  - dx
  - init
---

# ADR 0016 — Define safe project scaffolding for the default starter

## Context

`mkpage init` is part of the v0.1 command surface but currently performs no action.
Without an implemented initializer, onboarding cost is high and the generated starter
cannot serve as the first real integration checkpoint for widgets, theme, data, and
routing concepts.

## Capability statement

mkpage must create a deterministic, offline, and safe starter project from built-in
assets, including a practical starter surface with canonical configuration, theme
copy, starter layouts, and starter content that remains functional without JavaScript.

## User stories / scenarios

- As a new user, I can run `mkpage init` from an empty path and get a working starter
  project in one command.
- As a maintainer, I can add a template selector to keep initializer evolution explicit
  even when only `default` exists in v0.1.
- As a contributor, I can trust `mkpage init` to never overwrite existing files.
- As a user with no network, I can initialize a project even in offline environments.

## Acceptance criteria

1. `mkpage init [directory]` creates required starter files when the directory is
   missing.
2. `mkpage init` supports `--template default` and returns a deterministic error for
   unknown templates.
3. Initialization never overwrites existing files and refuses to write into a
   non-empty directory.
4. The generated project includes starter content and examples that show route layout,
   reusable widgets, status hints, and at least one authored markdown page.
5. Initialization output includes created and skipped paths in a deterministic order.
6. Generated files are deterministic across runs and locale/time.
7. The CLI usage provides next steps for build and verification.

## Out of scope

- Network-based template downloads.
- Interactive templates with prompts.
- Automatic deployment configuration or CI generation in v0.1.

## Open questions

- Whether additional templates should be introduced in v0.1 or staged after the first
  release cut.

## References

- https://github.com/delaudio/mkpage/issues/15

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-05 | r1 | default-agent | Initial proposal for safe starter initialization. |
| 2026-08-05 | r2 | default-agent | Mark implemented and bound to shipped init scaffolding behavior in CLI. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-05 | Approved scaffold-first implementation path |
