# Testing strategy

Choose tests to prove the behavior of each [unit of work](unit-of-work.md). Tests should provide confidence at the narrowest layer that exercises the relevant contract, with broader tests added when they verify a distinct integration boundary.

## Test layers

### Unit tests

Use unit tests for deterministic logic within a module: parsing, validation, state transitions, formatting, selection rules, and error mapping. Keep them fast and isolated from the filesystem, Git processes, network, clock, and UI when those are not part of the behavior under test. Inject or wrap external dependencies when needed to make outcomes controllable.

### Integration tests

Use integration tests when confidence depends on components working together across a module or process boundary, such as workspace discovery, Git command adapters, persistence, or application-facing service behavior. Use temporary directories and controlled fixtures. Do not use a developer's real repositories or personal files as test data.

### UI tests and manual checks

Test UI-facing behavior at the highest practical level. Verify state-to-view mapping, user actions, and visible outcomes, including relevant empty, loading, invalid, and failure states. Where automated UI interaction is not reliable or available, compile the Slint UI and perform targeted manual visual/interaction checks; report those separately from automated tests.

## Coverage expectations

For every changed behavior, identify its meaningful cases rather than aiming for a raw test count. Cover as applicable:

- Normal and representative inputs.
- Boundary values and empty state.
- Invalid or malformed input.
- Expected failure paths and recovery or retry behavior.
- State transitions and invariants.
- Security-relevant inputs, especially paths, process arguments, and data shown or logged.

Use equivalence classes and boundary analysis for large or unbounded input spaces; it is neither practical nor required to enumerate every possible input. Prefer table-driven tests where several cases exercise the same rule. Avoid brittle tests that depend on incidental private implementation details.

## Test placement and fixtures

- Keep test cases in files separate from production implementation files. Do not append a `#[cfg(test)] mod tests { ... }` block containing test cases to the implementation file. Rust needs a small `#[cfg(test)] mod tests;` declaration in the implementation module to include its separate unit-test file; keep that declaration free of test case bodies.
- For Rust unit tests that need access to private module behavior, declare the test submodule under `#[cfg(test)]` and put its contents in a separate child file. For example, `src/project.rs` can declare `#[cfg(test)] mod tests;` and keep the cases in `src/project/tests.rs`. This retains unit-test access to the module while keeping production and test code in separate files.
- Put Rust integration tests in separate files under `tests/` when they should exercise the crate through its public or application-facing boundary. Use descriptive filenames by behavior or feature, such as `tests/project_open.rs`.
- Keep test file and test function names descriptive and consistent with [naming guidance](naming-guidance.md).
- Keep test fixtures minimal, deterministic, and free of credentials or personal data. Generate temporary files and repositories at runtime where feasible.
- Do not duplicate the same assertion across layers unless each test establishes a distinct contract.
- When fixing a bug, add a regression test that reproduces the reported or discovered failure and passes with the fix. Prefer a test that fails against the pre-fix behavior. If the bug cannot be reproduced in an automated test, explain why and record the closest reliable manual verification.

## Running tests

For routine work, run only the relevant cases using the focused runners in `scripts/`:

```sh
./scripts/test-unit.sh <test-filter>
./scripts/test-integration.sh <test-filter>
```

Inspect runner behavior and test names first; pass the narrowest supported filter. The full-suite runner (`./scripts/test-all.sh`) is for explicit full-suite requests or changes whose shared behavior cannot be adequately verified with focused tests. Record exact commands and outcomes, including when a relevant test layer has no coverage yet.

Every behavior change should gain coverage in the appropriate layer as part of the change where feasible. If infrastructure or testability blocks coverage, document the concrete limitation and the highest-confidence validation performed.
