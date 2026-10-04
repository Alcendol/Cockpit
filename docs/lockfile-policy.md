# Lockfile policy

Cockpit is an application, so `Cargo.lock` is a tracked part of the repository and should remain committed.

- When dependencies or dependency constraints change, update `Cargo.lock` with Cargo and include it in the same change.
- Do not hand-edit the lockfile. Do not delete it to resolve a build problem.
- Avoid unrelated lockfile churn. If a toolchain update or intentional dependency refresh changes many entries, state why in the change summary.
- Prefer the smallest dependency update that satisfies the requirement; review newly added transitive dependencies and platform requirements.
- Do not pin or update dependencies solely to silence a warning without understanding the compatibility and security impact.
- Verify dependency changes with the relevant Cargo checks in the [developer runbook](developer-runbook.md).
