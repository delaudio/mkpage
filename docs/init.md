# Init command

`mkpage init` scaffolds a deterministic starter so you can start publishing without
manual wiring.

## Usage

```sh
mkpage init [directory] [--template default]
```

- `directory` defaults to `.`.
- `--template` currently supports `default` in v0.1.
- existing files are never overwritten.
- non-empty directories are refused to protect your local changes.
- output includes created and skipped paths.

The default starter contains:

- `mkpage.toml` with canonical defaults,
- `content/` markdown pages:
  - `index.md`, `about.md`, `uses.md`, `projects/index.md`, `projects/mkpage.md`,
  - `writing/index.md`, `writing/notes-from-terminal-design.md`,
- `layouts/page.html` with terminal-inspired structure,
- `layouts/widgets.jinja`,
- `static/css/site.css`, `static/css/override.css`,
- `data/projects.json`.

## After initialization

```sh
cd <directory>
mkpage build --root .
```

Then open `./public` with:

```sh
mkpage serve
```
