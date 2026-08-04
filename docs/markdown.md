# Markdown

mkpage uses `pulldown-cmark` with tables, task lists, footnotes, and
strikethrough enabled. Raw inline and block HTML is escaped, not trusted or
sanitized. `javascript:`, `data:`, and `vbscript:` links become `#`.

Rendered Markdown returns headings, links, referenced image assets, and a
plain-text summary. Heading IDs preserve Unicode letters and numbers, adding
`-2`, `-3`, and so on for duplicates. Fenced code is escaped and carries its
declared `language-*` class; unknown languages use the same accessible
`pre`/`code` fallback.
