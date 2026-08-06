# Installing mkpage

`mkpage` is a single, zero-dependency static-site generator binary built for Linux, macOS, and Windows.

## Installation Methods

### Cargo (from crates.io or git)

If you have Rust and Cargo installed:

```bash
cargo install mkpage
```

Or from source:

```bash
git clone https://github.com/delaudio/mkpage.git
cd mkpage
cargo install --path .
```

### Pre-built GitHub Release Binaries

Download pre-compiled release tarballs or ZIP archives from [GitHub Releases](https://github.com/delaudio/mkpage/releases):

- **Linux (x86_64 / aarch64)**: `mkpage-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`
- **macOS (Apple Silicon / Intel)**: `mkpage-v0.1.0-aarch64-apple-darwin.tar.gz` / `x86_64-apple-darwin.tar.gz`
- **Windows (x86_64)**: `mkpage-v0.1.0-x86_64-pc-windows-msvc.zip`

Extract the archive and move `mkpage` into your system `$PATH` (e.g. `/usr/local/bin` or `~/.cargo/bin`).

---

## Shell Completions

`mkpage` generates shell completions for `bash`, `zsh`, `fish`, `powershell`, and `elvish`.

### Zsh

```bash
mkdir -p ~/.zsh/completion
mkpage completions zsh > ~/.zsh/completion/_mkpage
```

Add the following to your `~/.zshrc`:

```bash
fpath=(~/.zsh/completion $fpath)
autoload -U compinit && compinit
```

### Bash

```bash
mkpage completions bash > ~/.local/share/bash-completion/completions/mkpage
```

### Fish

```bash
mkpage completions fish > ~/.config/fish/completions/mkpage.fish
```

### PowerShell

```powershell
mkpage completions powershell >> $PROFILE
```
