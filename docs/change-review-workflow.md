# Change review workflow

Before handing off implementation work, inspect the change as a reviewer would, comparing the topic branch with the active integration/base branch and running focused checks for affected behavior.

## 1. Establish the comparison

- Inspect `git status` and preserve unrelated user changes.
- Identify the active integration branch from the task/PR target or repository configuration. Do not assume it is named `main`.
- Before every review, refresh the integration branch from its remote. Fetch the remote's configured refs (for example, `git fetch origin`) and compare against the refreshed remote-tracking reference (for example, `origin/main`). If the local integration branch also needs updating, only fast-forward it with `git pull --ff-only <remote> <base>` while that branch is checked out and its worktree is clean; never run that pull while on the topic branch.
- Compare the topic branch with that base using the merge base where available, so the review covers changes introduced by the topic branch and not unrelated base-branch commits.
- Do not merge or rebase the topic branch just to refresh the review baseline. Use the refreshed remote-tracking reference as the comparison base; update the topic branch only when the task separately requires it.
- If the target branch cannot be determined or is not available locally, state the assumed base and limitation rather than silently comparing against an arbitrary branch.
- Review the complete diff, including untracked files that are part of the work. Check for accidental generated output, unrelated formatting, and missing documentation.

## 2. Review checklist

Inspect changed code and its callers for:

- **Lint and type/build safety:** run formatting, Clippy, and compilation checks appropriate to the affected Rust and Slint code. Confirm types and error paths remain sound; do not suppress a useful warning without a clear reason.
- **Naming:** follow [`naming-guidance.md`](naming-guidance.md) and existing domain terms across Rust, Slint, and docs.
- **Duplicate functions:** search for equivalent functions and repeated logic in the affected modules. Prefer reusing a suitable existing function when that improves clarity and does not create awkward coupling.
- **Built-in or standard functionality:** check whether a new helper merely reimplements a Rust standard-library, Slint, or established project capability. Use the built-in where its semantics fit; retain a wrapper when it adds meaningful domain behavior, validation, or a stable boundary.
- **Redundant functions and abstractions:** remove helpers that only forward arguments or duplicate a single caller's logic unless they establish a useful boundary, improve testability, or are intentionally public.
- **Magic numbers:** replace unexplained numeric literals with named constants or documented domain values. Keep obvious values such as indices, bit flags, and zero checks inline when naming them would add noise; explain non-obvious units and thresholds.
- **Sensitive-data exposure:** check logs, errors, UI text, telemetry, fixtures, and debug output for credentials, tokens, personal data, local absolute paths, repository contents, or other secrets. Avoid returning detailed internal diagnostics to users when they may expose sensitive data.
- **Behavior and compatibility:** examine boundary cases, invalid input, failure handling, state transitions, and API/UI contract changes relevant to the diff.
- **Regression coverage:** for a bug fix, confirm there is a focused test reproducing the failure and passing with the fix. If automated reproduction is not feasible, ensure the reason and closest reliable manual verification are recorded.

Review for both defects and worthwhile improvements, including minor issues. Check whether the change should be corrected or improved for correctness, clarity, consistency, naming, duplication, maintainability, UX, or test coverage—even when the issue is not release-blocking. Fix findings that are in scope and can be addressed safely within the unit of work; this includes minor fixes. Do not dismiss a real issue only because it is low severity.

Keep the review focused: do not expand into unrelated cleanup, speculative refactoring, or subjective stylistic preferences that conflict with no documented convention. If a useful improvement is independent or would materially enlarge the change, record it as a follow-up rather than folding it in.

Report actionable findings in severity order and include file/line context when available. Before PR creation, resolve all in-scope actionable findings, including minor ones. If an out-of-scope finding remains, explain why it is deferred and record it in the PR reviewer note. If no finding is identified, say so and state what was actually checked.

## 3. Run focused verification

- Run formatting/lint/build checks needed to establish that the affected Rust and Slint code remains valid. These checks are separate from running every test.
- Select tests from the changed behavior and its dependencies. Use `./scripts/test-unit.sh` for affected unit tests and `./scripts/test-integration.sh` for affected integration behavior. Inspect the scripts' supported selectors before invoking them; pass the narrowest supported filter or target.
- Do not run `./scripts/test-all.sh` for routine change review. Use it only when the user requests the full suite or the change affects shared behavior that cannot be adequately checked with focused targets; explain the reason if it is needed.
- If a focused script lacks a selector or runs its entire category, invoke the equivalent narrow Cargo test target/filter directly where possible and report the exact command. Do not claim unrelated tests were run.
- For Slint/UI changes, compile the UI and perform targeted visual/interaction checks in addition to relevant automated tests. See [`developer-runbook.md`](developer-runbook.md).
- Record commands and outcomes. If no relevant tests exist yet, state that gap and add appropriate tests with the behavior change where feasible.

## 4. Handoff review

Summarize any findings fixed during review, focused checks and tests run, and meaningful risks or limitations. A review pass does not replace the author's normal implementation verification; report the scope of each clearly.
