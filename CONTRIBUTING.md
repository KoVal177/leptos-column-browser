# Contributing to leptos-column-browser

Thank you for your interest in contributing!

## Development Setup

```bash
# Rust WASM target (one-time)
rustup target add wasm32-unknown-unknown

# Trunk web bundler (one-time)
cargo install trunk
```

## Running Tests

```bash
# Library unit and integration tests (native)
cargo test -p leptos-column-browser --all-features

# WASM compile check
cargo build -p leptos-column-browser --target wasm32-unknown-unknown
```

## Running Examples

```bash
cd examples/file_explorer && trunk serve   # http://localhost:8080
cd examples/api_navigator && trunk serve   # http://localhost:8080
```

## Code Style

```bash
cargo fmt                                                  # format
cargo clippy --all-features -- -D warnings                 # lint
```

This project follows `rustfmt.toml` (edition 2024, max_width 100).
`unsafe_code` is forbidden; `missing_docs` warns — add doc comments to new public items.

## Pull Request Checklist

- [ ] `cargo fmt` passes with no changes
- [ ] `cargo clippy --all-features -- -D warnings` is clean
- [ ] `cargo test -p leptos-column-browser --all-features` passes
- [ ] `cargo build -p leptos-column-browser --target wasm32-unknown-unknown` succeeds
- [ ] Public API changes are documented in `CHANGELOG.md` under `[Unreleased]`
- [ ] New public items have doc comments

## Reporting Bugs

Open an issue on [GitHub](https://github.com/KoVal177/leptos-column-browser/issues)
and include: Rust toolchain version, browser + OS, a minimal reproducible example,
and the full error or unexpected behaviour.
