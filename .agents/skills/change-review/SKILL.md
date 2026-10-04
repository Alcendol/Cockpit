---
name: change-review
description: Review the current change against its active integration branch, fix actionable in-scope findings, and run focused verification. Use before handing off code or preparing a pull request.
---

# Change Review

Run the Cockpit change review workflow as an active review-and-fix pass. Read the repository's `AGENTS.md` and [`docs/change-review-workflow.md`](../../../docs/change-review-workflow.md) first; consult linked guidance and the runbook when the changed area requires them. This skill activates and carries out that workflow rather than replacing its detailed guidance.

## Establish the review target

1. Inspect `git status`, preserve unrelated user changes, and identify the complete requested change, including relevant untracked files.
2. If preparing a PR or reviewing one, check whether the branch already has an open PR and confirm the change belongs to its stated scope. Keep independent work separate.
3. Resolve the active integration/base branch from the task, PR target, or repository configuration. Do not assume `main`.
4. Refresh the base branch's remote-tracking ref before review (for example, fetch the configured remote). Compare the topic branch from its merge base with the refreshed ref. Do not merge or rebase merely to review. If the base is unavailable or uncertain, state the assumption and limitation.
5. Review the complete diff and enough surrounding code, callers, tests, and UI to validate each finding.

## Review and resolve

Check the areas in `docs/change-review-workflow.md`, including correctness and compatibility, lint/build safety, naming, duplicate or redundant logic, standard-library/framework built-ins, magic numbers, sensitive-data exposure, workflow security, boundary and failure behavior, filesystem/UI responsiveness, filesystem names, resizing, and regression coverage as applicable.

Find both defects and worthwhile in-scope improvements. A finding must be concrete, introduced by the change, demonstrated by an affected scenario, and actionable. Do not add speculative concerns, unrelated cleanup, or subjective style preferences. Fix confirmed findings that fit the unit of work, including minor issues. Record independent or materially scope-expanding work as a follow-up with its reason.

## Verify and hand off

1. Choose focused formatting, lint, build, unit, integration, and UI checks for the changed behavior. Inspect script selectors before use. Do not run `scripts/test-all.sh` unless the user asks or focused coverage cannot establish a shared-behavior change; explain that reason.
2. Add or update focused regression coverage for bug fixes where feasible. For UI changes, compile and perform targeted visual/interaction checks as the runbook allows.
3. Record exact commands and outcomes. Do not claim checks that were not run; explain unavailable checks and remaining risks.
4. Recheck the final diff for accidental files, generated output, secrets, unrelated churn, and missing documentation.
5. Report actionable findings in severity order with concise file/line context, fixes made, verification performed, and any deferred follow-up. If no findings remain, say what was reviewed and checked.

Before creating a PR, complete the repository review workflow and resolve all in-scope actionable findings. Follow `docs/pull-request-template.md` for the reviewer note and deferred work.
