# Structured data and collections

mkpage loads JSON files below `data/`. Directories become dotted logical keys:
`data/projects/items.json` is available as `projects.items`. Files are sorted
lexicographically, and duplicate logical keys fail.

A collection selects an array or object data source, a layout, an output pattern
containing `{slug}`, and optionally a slug field. Array keys are original
indices; object keys are original property names. If no slug field is specified,
the original key is used. Slugs, route patterns, and collection collisions are
validated before any output is written.

YAML and remote data are intentionally unsupported in v0.1’s data capability.
