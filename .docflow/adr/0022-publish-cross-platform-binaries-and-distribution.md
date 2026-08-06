---
adr: 0022
title: Publish cross-platform binaries, shell completions, crate metadata, and installation documentation
status: Implemented
date: 2026-08-06
owner: default-agent
depends-on:
  - adr/0003-bootstrap-the-rust-cli.md
  - adr/0021-add-conformance-and-quality-release-gates.md
tags: [dx, release, publishing, metadata, completions, CI]
---

# ADR 0022 — Publish cross-platform binaries, shell completions, crate metadata, and installation documentation

## Context

To release `mkpage` v0.1 for terminal-minded users across Linux, macOS, and Windows, the repository must provide complete Crate metadata, shell completion generation (`mkpage completions <shell>`), automated multi-target binary release workflows, and explicit installation documentation.

## Capability statement

mkpage ships:

- Complete `Cargo.toml` package metadata (readme, license, keywords, repository, documentation).
- Shell completion generation CLI subcommand (`mkpage completions <shell>`) supporting `bash`, `zsh`, `fish`, `powershell`, and `elvish`.
- GitHub Actions automated release pipeline (`.github/workflows/release.yml`) for multi-platform binary compilation and packaging.
- Comprehensive user installation documentation (`docs/installation.md`).

## User stories / scenarios

- As a user, I can install `mkpage` via `cargo install mkpage` or download pre-compiled binary archives for Linux (x86_64/aarch64), macOS (x86_64/aarch64), and Windows (x86_64).
- As a terminal user, running `mkpage completions zsh > ~/.zsh/completion/_mkpage` provides shell autocompletion.
- As a maintainer, tagging a release `v*` automatically builds and attaches release artifacts.

## Acceptance criteria

1. `Cargo.toml` includes complete package metadata and documentation links.
2. `mkpage completions <shell>` generates valid completion scripts to stdout for `bash`, `zsh`, `fish`, `powershell`, and `elvish`.
3. GitHub Actions release workflow `.github/workflows/release.yml` defines dry-run and release jobs for Linux, macOS, and Windows binaries.
4. `docs/installation.md` documents installation, shell completion setup, and release procedures.
5. Unit tests verify `mkpage completions` command output deterministically.

## Out of scope

- Proprietary third-party package registries or unverified OS package manager submissions prior to v0.1 release tag.

## References

- https://github.com/delaudio/mkpage/issues/20

## Revision History

| Date | Revision | Author | Change |
|---|---|---|---|
| 2026-08-06 | r1 | default-agent | Initial decision for cross-platform binary publication, shell completions, and release docs. |

## Approvals

| Role | Name | Date | Signature |
|---|---|---|---|
| Product owner | Federico Del Gaudio | 2026-08-06 | Approved publishing and distribution ADR |
