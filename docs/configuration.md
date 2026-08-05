# Configuration

mkpage reads one canonical project configuration file: `mkpage.toml`. There
are no aliases for older experimental names.

## Discovery and precedence

Without flags, mkpage walks from the current directory toward the filesystem
root and uses the nearest directory containing `mkpage.toml`.

1. `--config <path>` selects a configuration file. A relative value is resolved
   from `--root` when it is supplied, otherwise from the current directory.
2. `--root <path>` selects the project root and its `mkpage.toml`.
3. Without either flag, upward discovery selects the nearest `mkpage.toml`.

Configuration values override built-in defaults. Environment variables are not
used as a second configuration system.

Run `mkpage --verbose build` to print the resolved project root and configuration
path before compiler behavior is introduced.

## Schema v1

`version = 1` is required. Unknown fields fail deliberately, so spelling errors
do not silently change a build.

```toml
version = 1

[paths]
source = "content"
layouts = "layouts"
data = "data"
static_files = "static"
output = "public"

[site]
base_url = "https://example.com"
trailing_slash = "always" # or "never"
include_metadata = false
include_feed = false
include_sitemap = false

[theme]
name = "terminal"

[enhancements]
keyboard = true
```

Defaults are `content`, `layouts`, `data`, `static`, `public`, the `terminal`
theme, keyboard enhancements enabled, and an `always` trailing-slash policy.

Site artifact generation is opt-in:

- `include_metadata = true` writes `metadata.json` with one route record per built
  page.
- `include_feed = true` writes `feed.xml`.
- `include_sitemap = true` writes `sitemap.xml`.

All configured paths are made absolute lexically from the configuration file’s
directory; they do not have to exist while parsing. mkpage does not dereference
symlinks during this phase, so validation is predictable and does not inspect
the filesystem beyond reading the configuration itself. Source, layout, data,
and static inputs cannot overlap the output directory. Output cannot be the
project root, filesystem root, or home directory.
