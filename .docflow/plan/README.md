# Plan

This folder is the implementation queue. It complements public GitHub issues: issues communicate the roadmap externally, while this queue records the ordered unit of work, owning ADRs, exit criteria, and shipping evidence.

## Layout

- `todo/NNNN-<slug>.md` — pending work, lower numbers first.
- `done/<YYYY-MM-DD>-<slug>.md` — shipped work, in completion order.

Create a todo item before substantive work begins. On shipment, move it to `done/`, add a footer with the HEAD SHA, and update the owning ADR to `Implemented`.

The completion event is a fast-forward to `main` followed by a successful push to `origin`.
