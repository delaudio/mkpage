# Conventions

## Project

Project name: mkpage.

Description: A Rust static-site generator for terminal-minded websites, inspired by TUI patterns but producing accessible static HTML/CSS.

Language: en-US throughout.

Architecture: begin as a monolithic Rust CLI with clearly separated internal modules. Do not introduce a Cargo workspace without a documented need.

Artefact root: `.docflow/`. ADRs, plans, the index, and this file live under this root; `AGENTS.md` and `CLAUDE.md` remain at repository root.

Assessment depth: full.

## ADR files

ADR filenames use `NNNN-kebab-case-slug.md`, with contiguous numeric numbering and no reserved gaps. Each ADR contains one decision. If a decision splits, create successor ADRs and supersede the original.

Status lifecycle: `Proposed → Accepted → Implemented → (Superseded | Deprecated)`.

| Status | Meaning |
|---|---|
| Proposed | Decision drafted, not yet approved. |
| Accepted | Decision approved; implementation may be queued. |
| Implemented | Shipped by the recorded completion event. |
| Superseded | Replaced by a named successor ADR. |
| Deprecated | No longer applicable and has no successor. |

The first persisted status is `Proposed`. Cross-references use paths relative to `.docflow/`, for example `adr/0001-record-architecture-decisions.md`.

## ADR shape

This repository uses the capability-first record model. Every ADR follows this section order: Context, Capability statement, User stories / scenarios, Acceptance criteria, Out of scope, Open questions, References, Revision History, Approvals. Acceptance criteria must be numbered and testable.

## Product boundaries

- Generated output is semantic, accessible HTML.
- JavaScript is optional progressive enhancement only; the core build and output must work without it.
- The project does not implement a terminal emulator or render Ratatui buffers. It translates terminal-inspired interaction and visual patterns for the web.
- Builds are deterministic and require no Node.js toolchain.

## ADR privacy

ADRs are internal artefacts. ADR numbers, titles, and catalogue references must not appear in product output, public documentation, release notes, or support communications. They may appear in code comments, commits, plans, and internal documentation.

## Single-agent rules

A single agent owns the repository at a time. `.docflow/_agent/` holds the live focus and durable worklog; file locks are not used.

## Plan folder

Pending work is kept in `plan/todo/`, with lower numbers running first. Shipped work moves to `plan/done/` using a date-prefixed filename. Every substantive implementation item names its owning ADRs, scope, dependencies, and testable exit criteria.

The completion event is: fast-forwarded to `main` and successfully pushed to `origin`. When an item ships, move it to `plan/done/`, add a footer naming the HEAD SHA, advance owning ADRs from `Accepted` to `Implemented`, and regenerate `INDEX.md`.

## Git contract

- Use Conventional Commits.
- Include a `Rationale:` footer in every commit that changes an ADR.
- Commit signing is optional.
- Do not use ADR revision tags or `Co-Authored-By` trailers.
- Integrate direct-to-main with fast-forward only; do not create merge commits.
- Run `cargo fmt --check && cargo clippy -- -D warnings && cargo test` locally before pushing.

## Optional layers

- `domains/` groups ADRs by area without creating a separate numbering namespace.
- `GLOSSARY.md` defines terms that might otherwise drift.
- `goals/` contains active outcome goals. Create a goal from `goals/G-template.md` when an outcome is agreed.
