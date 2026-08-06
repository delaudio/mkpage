# Dogfood Migration Guide — federicodelgaudio.com

This document records the migration procedure, route preservation, deployment steps, and rollback plan for dogfooding `mkpage` v0.1 on [federicodelgaudio.com](https://federicodelgaudio.com).

## Route Mapping and Preserved URLs

| Legacy URL Path | `mkpage` Source Path | Output Path | Canonical URL |
|---|---|---|---|
| `/` | `content/index.md` | `public/index.html` | `https://federicodelgaudio.com/` |
| `/about/` | `content/about.md` | `public/about/index.html` | `https://federicodelgaudio.com/about/` |
| `/uses/` | `content/uses.md` | `public/uses/index.html` | `https://federicodelgaudio.com/uses/` |
| `/projects/` | `content/projects/index.md` | `public/projects/index.html` | `https://federicodelgaudio.com/projects/` |
| `/projects/mkpage/` | `content/projects/mkpage.md` | `public/projects/mkpage/index.html` | `https://federicodelgaudio.com/projects/mkpage/` |
| `/writing/` | `content/writing/index.md` | `public/writing/index.html` | `https://federicodelgaudio.com/writing/` |
| `/writing/notes/` | `content/writing/notes-from-terminal-design.md` | `public/writing/notes-from-terminal-design/index.html` | `https://federicodelgaudio.com/writing/notes-from-terminal-design/` |

---

## Verification & Conformance Checklist

Before deployment:

- [x] Run `mkpage build` with production profile.
- [x] Verify HTML doctype, UTF-8 charset, responsive viewport, and `lang="en"`.
- [x] Verify `sitemap.xml`, `feed.xml`, and `search_index.json` generation.
- [x] Verify keyboard shortcuts (`/` command palette search, `j`/`k` list navigation).
- [x] Verify complete no-JavaScript rendering (core content accessible without JS).
- [x] Pass automated conformance gates (`cargo test --test conformance`).

---

## Deployment & Rollback Procedure

### Deployment Steps

1. Build the production site artifact:
   ```bash
   mkpage build
   ```
2. Deploy the contents of `public/` to the static hosting target (Cloudflare Pages / GitHub Pages / rsync server).
3. Verify public URLs, SSL certificate, RSS autodiscovery, and sitemap.

### Rollback Procedure

1. If a regression occurs, re-deploy the previous release candidate build artifact stored in release archives.
2. Invalidate CDN cache for `search_index.json`, `js/mkpage-keyboard-v1.js`, and `css/site.css`.
