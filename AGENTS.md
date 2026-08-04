# AGENTS.md

## What this repository is

`mkpage` is a Rust static-site generator for terminal-minded websites. It turns Markdown and declarative, TUI-inspired layouts into accessible static HTML and CSS. The repository is documentation-led: its ADR catalogue records the capabilities that implementation must match.

## Repository structure

- `.docflow/CONVENTIONS.md` — authoring and delivery rules; read this first.
- `.docflow/adr/0000-template.md` — canonical capability ADR template.
- `.docflow/adr/NNNN-<kebab-slug>.md` — one ADR per decision, contiguous numbering.
- `.docflow/INDEX.md` — generated ADR catalogue.
- `.docflow/plan/todo/NNNN-<slug>.md` — queued work, lower numbers first.
- `.docflow/plan/done/<YYYY-MM-DD>-<slug>.md` — shipped work.
- `.docflow/_agent/` — single-agent worklog, focus snapshot, and handoff.
- `.docflow/GLOSSARY.md` — shared terms.
- `.docflow/domains/<slug>/README.md` — curated ADR indexes by area.
- `.docflow/goals/G-template.md` — template for outcome goals.

## Hard rules when editing ADRs

- One decision per ADR; split decisions into new ADRs and supersede the old one.
- Status lifecycle: `Proposed → Accepted → Implemented → (Superseded | Deprecated)`.
- ADRs use this section order: Context, Capability statement, User stories / scenarios, Acceptance criteria, Out of scope, Open questions, References, Revision History, Approvals.
- Acceptance criteria are numbered, observable, and testable.
- ADR identifiers and catalogue details are internal; never expose them in product output, public documentation, release notes, or support copy.
- Output must be semantic and accessible HTML.
- The core build must work without JavaScript and must not require Node.js.
- Do not build a terminal emulator or a Ratatui buffer renderer; TUI is an interaction and visual language for generated websites.
- Builds must be deterministic and Node-free.

## Implementation work

- Read the relevant ADR and queue item before changing behaviour.
- Create a `.docflow/plan/todo/` item before substantive implementation work.
- Keep implementation and tests aligned with the owning ADR’s acceptance criteria.
- If work changes a decision, update or supersede its ADR rather than silently diverging.
- Regenerate `.docflow/INDEX.md` after creating an ADR or changing its status.

## Single-agent workflow

A single agent owns the repository at a time. Update `.docflow/_agent/CURRENT_FOCUS.md` as work starts or stops and append one row to `.docflow/_agent/WORKLOG.md` on every commit.

## Delivery

- Commit messages follow Conventional Commits.
- Commits touching ADRs require a `Rationale:` footer.
- Commit signing is optional.
- Integrate directly into `main` with fast-forward only; no merge commits.
- Before pushing, run: `cargo fmt --check && cargo clippy -- -D warnings && cargo test`.
- A queue item is shipped only after fast-forwarding to `main` and successfully pushing to `origin`.
