# Developer runbook

## Requirements

- Rust stable with `rustfmt` and Clippy components.
- Platform build dependencies required by Slint for the host operating system.
- Git. `pre-commit` is optional.

## Build and run

From the repository root:

```sh
cargo check
cargo run
```

The build script compiles `ui/main.slint` using Slint's compiler. `cargo check` therefore validates Rust compilation and Slint syntax/type integration; it does not replace visual inspection of UI behavior and layout.

## Quality checks

Run the appropriate checks before handoff. Repository-wide test runner scripts live in the repository's `scripts/` directory. For routine changes, select only the relevant unit or integration tests:

```sh
./scripts/test-unit.sh
./scripts/test-integration.sh
```

Pass a test-name filter to either focused runner when the suite contains unrelated cases you do not need to run. The integration runner skips when no `tests/` directory exists.

Run the combined test check only when the full suite is requested or focused tests cannot adequately cover a shared behavior change:

```sh
./scripts/test-all.sh
```

The scripts propagate failures and return a non-zero exit code if any selected test fails.

Run the combined formatting, lint, and build check with:

```sh
./scripts/check.sh
```

The equivalent direct Cargo commands are:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --bin cockpit
cargo test --test '*'
```

`cargo test --test '*'` may have no integration test targets in the current starter project. Add integration targets as integration boundaries appear; document platform-dependent skips rather than treating them as passing coverage.

## UI validation

- Compile and type-check the Slint UI through `cargo check --all-targets --all-features` (the build script invokes `slint-build`).
- Run the app with `cargo run` for UI changes and inspect the affected screens at a useful window size.
- Check relevant interaction, empty, loading, and error states, plus resizing behavior where applicable. A successful compile does not establish that the UI is visually correct or usable.
- Add automated UI or component tests where the behavior can be exercised reliably. Keep manual visual verification in the PR validation notes when automation is not practical.

## Git hook

The repository configures checks through `.pre-commit-config.yaml`. Install the optional pre-commit hook with:

```sh
pre-commit install
```

## Troubleshooting

- If Slint fails to build because a system library or compiler is missing, install the platform prerequisite documented by Slint for the host OS, then retry. Do not change application code to work around an environment-only failure.
- If Rust components are missing, install `rustfmt` and `clippy` for the active stable toolchain.
- If a check fails, report the command and relevant diagnostic. Do not present a failed or skipped check as successful.
- If a documented test script is missing, use the direct Cargo commands above and record that the corresponding `scripts/` runner entry point needs implementation.
