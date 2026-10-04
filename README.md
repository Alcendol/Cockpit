# Cockpit

A lightweight desktop cockpit for reviewing and finishing code changes.

Cockpit is built with Rust and [Slint](https://slint.dev/) and targets macOS, Windows, and Linux.

## Prerequisites

- Rust stable with `rustfmt` and Clippy components
- Platform build dependencies required by Slint
- [pre-commit](https://pre-commit.com/) (optional; the Git hook runs the same checks directly)

## Run

```sh
cargo run
```

## Quality checks

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
```

Install the optional pre-commit hook:

```sh
pre-commit install
```
