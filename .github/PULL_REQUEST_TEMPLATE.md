<!-- Title: <type>(<optional scope>): <imperative summary> — follow docs/commit-conventions.md -->

## Summary

<!-- What changed and why? -->

## Changelog

<!-- User-visible changes in plain language, or "None". -->

## Validation

<!-- Summarize the pre-PR change review and list checks run with outcomes. Note skipped or blocked checks and why. -->

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo check --all-targets --all-features`
- [ ] Focused unit tests (if applicable):
- [ ] Focused integration tests (if applicable):
- [ ] Manual verification (if applicable):

## Merge and review order

<!-- State the requested review order, or "No special order." Link prerequisite PRs if applicable. -->

## Reviewer note

<!-- Call out notable risk, compatibility impact, migration, security-sensitive behavior, data-loss possibility, or focus area. Write "None" if there is nothing notable. -->
