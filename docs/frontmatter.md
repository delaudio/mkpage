# Frontmatter and publication

Frontmatter is optional. When present, it is TOML between opening and closing
lines containing exactly `+++`:

```toml
+++
title = "About"
date = "2026-08-04"
draft = false
tags = ["rust", "tui"]

[extra]
accent = "green"
+++
```

Reserved fields are `title`, `description`, `date`, `updated`, `draft`,
`layout`, `slug`, `tags`, `projects`, `canonical_url`, and `social_image`.
All author-defined values must be under `extra`; unknown top-level fields fail.

`date` and `updated` are quoted `YYYY-MM-DD` calendar dates. They are never
converted through local midnight or a local timezone. Production profiles omit
drafts and pages dated after their supplied calendar date. Development profiles
include both; draft pages receive a visible draft marker in render context.
