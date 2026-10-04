# Iteration shape guidance

Shape work according to the [unit-of-work guidance](unit-of-work.md), so each change has one clear purpose and can be reviewed without reconstructing a long chain of context.

- Start with the smallest end-to-end slice that demonstrates the intended behavior.
- Keep edits close to the feature. Avoid opportunistic renames, formatting churn, and unrelated refactors.
- Separate mechanical changes from behavior changes when that makes review safer.
- Prefer explicit, narrow interfaces over broad shared frameworks introduced for one caller.
- Preserve a working build where practical; when an intermediate step cannot build, keep that interval short and explain why.
- Describe user-visible behavior and relevant edge cases in the change summary.

For substantial work, split into ordered slices such as: domain/API shape, implementation, UI integration, then polish and verification. Each slice should leave a clear state and should not duplicate work from another slice.
