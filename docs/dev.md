# Dev and serve commands

`mkpage dev` provides a local watch-and-rebuild loop and local preview server.

```sh
mkpage dev [--host 127.0.0.1] [--port 3000] [--interval-ms 800]
```

- watches `content`, `layouts`, `data`, `static`, and `mkpage.toml`.
- rebuilds in development mode with draft-aware behavior.
- serves the current output at `http://<host>:<port>`.

`mkpage serve` serves a generated site locally.

```sh
mkpage serve [--host 127.0.0.1] [--port 3000]
```

Behavioral notes:

- `serve` requires a generated output directory (for example `public`).
- requests use safe path mapping and fallback to `index.html` for directory-like
  routes.
