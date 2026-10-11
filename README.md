# rcat

A fast, Unix-inspired recursive file concatenation tool written in Rust.

## Features

- **Recursive Traversal:** Walks directory trees using `walkdir`.
- **Smart Extension Filtering:** Include (`-e`) or exclude (`-E`) specific file extensions with case-insensitive, comma-delimited matching.
- **File Banners:** Prepend clean header indicators (`===> path/to/file <===`) with `-H` before each file's contents.
- **Hidden File Control:** Skips dotfiles by default, with an override (`-a`) to include them when needed.

---

## Installation

### Via Crates.io (Recommended)
```bash
cargo install rcat-rs
```

### Via Cargo (From GitHub)
```bash
cargo install --git https://github.com/KingAJK30/rcat.git --tag v1.0.0
```

### Via Nix Flakes (NixOS / Home Manager)

First, add `rcat` to your `flake.nix` inputs:
```nix
inputs.rcat.url = "github:KingAJK30/rcat/v1.0.0";
```

Then add the package to your configuration:

#### For NixOS:
```nix
environment.systemPackages = [
  inputs.rcat.packages.{pkgs.system}.default
];
```

#### For Home Manager
```nix
home.packages = [
  inputs.rcat.packages.{pkgs.system}.default
];
```
*(Note: Ensure `inputs` is available as a module argument)*

---

## Usage

```bash
rcat [OPTIONS] <PATHS>...
```

### Arguments
- **`<PATHS>...`**: Target paths (files or directories)

### Options
- **`-a`, `--all`**: Include hidden files and directories
- **`-H`, `--headers`**: Print file path banners before each file's contents
- **`-e`, `--ext <EXT>`**: Only include files with these extensions (comma-separated)
- **`-E`, `--exclude <EXT>`**: Exclude files with these extensions (comma-separated)
- **`-h`, `--help`**: Print help
- **`-V`, `--version`**: Print version

---

## Examples

```bash
rcat -H -e rs .
```

```bash
rcat -E md,lock src/
```

---

## License

Distributed under the Mozilla Public License 2.0 (MPL 2.0). See `LICENSE` for details.
