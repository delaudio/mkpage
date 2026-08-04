# Fixture sites

Fixtures are immutable source inputs. The golden harness copies one into an
OS-provided temporary directory, invokes `mkpage::compiler::build_site`, and
compares normalized output to the checked-in `golden/` tree.

Set `MKPAGE_UPDATE_GOLDENS=1` only after reviewing an intentional output change;
the normal test command never rewrites golden files.

Reserved fixture names for later capabilities: `nested-pages`,
`markdown-frontmatter`, `layout-partial`, and `static-assets`. They will be
activated by their owning compiler issues rather than carrying speculative tests.
