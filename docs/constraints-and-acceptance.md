# Constraints and acceptance criteria

Agentic implementation is more reliable when the intended result and its boundaries are explicit. For each meaningful [unit of work](unit-of-work.md), identify the constraints that apply and define observable acceptance criteria before or during implementation. Keep them proportional to the change: record relevant constraints, not a boilerplate checklist of irrelevant possibilities.

## Constraints

Constraints are conditions the solution must preserve. Consider the categories below and record only those that affect the work:

- **Behavior:** existing behavior to preserve, supported states, user-visible outcomes, and compatibility expectations.
- **Data and security:** trust boundaries, path handling, privacy, secret exposure, destructive actions, and data retention.
- **UI and accessibility:** layout/resizing, keyboard and focus behavior, labels, loading/empty/error states, and platform expectations.
- **Performance and resources:** responsiveness, bounded work, memory/resource ownership, and cancellation needs.
- **Architecture:** Rust/Slint responsibility boundaries, public API stability, dependency direction, and reuse requirements.
- **Platform and dependencies:** supported operating systems/toolchains, dependency restrictions, and lockfile expectations.
- **Operational constraints:** logging, diagnostics, recovery, and user-visible failure behavior.

For each applicable constraint, say what must be true and how a reviewer can verify it. If a category does not apply, omit it rather than writing “N/A” repeatedly. Do not invent requirements that the user or project has not established; flag material ambiguity and continue independent work where possible.

## Acceptance criteria

Write acceptance criteria as observable outcomes, not implementation steps. A criterion should be specific enough that an implementer and reviewer can agree whether it passed. Include applicable cases such as:

- Expected behavior for representative valid input.
- Empty, boundary, or unusual but supported input.
- Invalid input and expected error or recovery behavior.
- State changes and invariants that must hold.
- Relevant UI presentation and interaction outcomes.
- Compatibility or security properties that must remain true.

Use a criterion-to-verification map for non-trivial changes:

| Acceptance criterion | Verification |
| --- | --- |
| A valid workspace path becomes the active workspace. | Focused service integration test using a temporary directory. |
| An inaccessible path leaves the current workspace unchanged and shows an actionable error. | Integration test for state preservation plus targeted UI state check. |

These examples are illustrative only; use the real feature behavior. A verification can be an automated test, relevant lint/build check, or targeted manual check. Prefer automated tests for repeatable behavior. Manual verification should identify the action and observed result, not just say “tested manually.”

## Coverage and limits

- Cover every acceptance criterion with at least one appropriate verification method; use multiple layers when they establish different contracts.
- Cover meaningful normal, boundary, invalid, and failure cases that follow from the criteria. Use equivalence classes and boundary analysis for large or unbounded input spaces rather than claiming exhaustive enumeration.
- For every bug fix, add a regression test for the reported or discovered failure, preferably one that fails against the pre-fix behavior and passes with the fix. If automated reproduction is not feasible, document why and specify the closest reliable manual verification.
- Do not add tests that only duplicate implementation details or assertions already established at a more appropriate layer.
- If a criterion cannot be verified because a capability, fixture, platform, or test harness is missing, state the gap, explain the limitation, and identify the closest available check. Add the needed test support when feasible within the unit of work.
- If criteria change during implementation, update the plan and verification map so the final result is evaluated against the agreed behavior.

## Completion gate

Before handoff, confirm that:

1. Applicable constraints are documented and preserved.
2. Each acceptance criterion has a recorded verification outcome.
3. Focused tests were selected according to [testing strategy](testing-strategy.md); do not run the full suite by default.
4. Any unmet criterion, unavailable verification, or known limitation is explicit in the handoff.

Passing all available checks does not convert an unverified criterion into a pass. Report the evidence and its limits plainly.
