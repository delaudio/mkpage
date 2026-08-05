# Templates

mkpage uses MiniJinja. Layout names are logical paths such as `post` or
`blog/post`; they resolve to `.html` files below `layouts/`. Includes and
inheritance use the same relative template namespace.

The public context is intentionally bounded:

- `site`: public site settings;
- `site.base_url` and `site.trailing_slash` from `[site]` configuration;
- `page`: validated metadata, headings, and link metadata;
- `content`: rendered Markdown, the only trusted HTML value;
- `data`: validated structured data;
- `build`: non-secret build metadata.

All ordinary values are HTML-escaped. Templates never receive absolute paths,
environment variables, or secrets. Adding a public context field is a
compatibility decision and must be documented deliberately.
