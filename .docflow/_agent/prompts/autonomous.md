# Autonomous completion prompt

Drive the implementation queue in `.docflow/plan/todo/` to completion, unsupervised, until it is empty or a documented stop condition fires.

Read `AGENTS.md`, `.docflow/CONVENTIONS.md`, the queue README, current focus, the ADR index, the worklog, then the selected queue item and its owning ADRs. Pick the lowest-numbered queue item, implement its acceptance criteria, and run this verify gate before every push:

```sh
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

Use Conventional Commits and include `Rationale:` for ADR changes. Integrate with fast-forward only, push `main`, then move the item to `plan/done/`, add the HEAD SHA, update the ADR status, regenerate the index, worklog, and focus.

Stop cleanly if verification fails for an unknown reason, the queue is empty, an ADR is not accepted, or its criteria are ambiguous.
