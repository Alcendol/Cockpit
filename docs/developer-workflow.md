# Developer workflow

## 1. Orient

- Read the root `AGENTS.md`, this document, and any directly relevant guidance in `docs/`.
- Inspect the current implementation and repository status before editing. Preserve unrelated user changes.
- Identify the user-visible outcome and the smallest set of files needed to achieve it.
- For planned or multi-step work, record applicable constraints and observable acceptance criteria using [constraints and acceptance guidance](constraints-and-acceptance.md). Keep the criteria specific enough to verify, including relevant negative and failure behavior.
- If requirements are ambiguous, resolve them from existing behavior and project conventions where possible; ask only when the ambiguity materially affects the result.

## 2. Plan and implement

- For a focused change, proceed directly. For multi-part or architectural work, write a short ordered plan before editing.
- Define the independently understandable behavior slice first using [unit-of-work guidance](unit-of-work.md), then keep each iteration coherent and reviewable using [iteration shape](iteration-shape-guidance.md).
- Prefer the existing Rust and Slint stack and established patterns. Avoid speculative abstractions and unrelated cleanup.
- Update documentation alongside behavior when the contract or workflow changes.
- Keep error handling actionable. Do not silently discard failures that affect user data or application state.

## 3. Verify

- Run relevant formatting, build, and test commands from the [developer runbook](developer-runbook.md). Use the `scripts/` unit or integration runner for focused tests; reserve the full-suite entry point for explicit requests or cases where focused tests are insufficient.
- Verify each acceptance criterion using the applicable automated test, build/lint check, or manual UI check. Every behavior change must have automated test coverage at the appropriate level, following the [testing strategy](testing-strategy.md). Use unit tests for isolated logic and integration tests for interactions across module, persistence, process, or UI boundaries. Cover the meaningful behavior space: normal cases, boundary conditions, invalid input, and failure paths. Do not claim to test literally unbounded input spaces; use equivalence classes, boundary values, and explicit invariants to make coverage systematic.
- For a feature that crosses unit and integration boundaries, provide both kinds of tests where each adds distinct confidence. Do not duplicate identical assertions across layers without a reason.
- Do not claim verification that was not performed. If a required test kind is not yet runnable because infrastructure is missing, document the gap and add the runner or test support as part of the change where feasible.
- For UI changes, compile the Slint UI, launch the app, and inspect the affected layout at a useful window size. Check relevant interaction, empty, loading, and error states, and resizing where applicable; add automated UI tests for repeatable behavior where practical.
- Follow the [change review workflow](change-review-workflow.md) against the active integration/base branch. Select only test suites relevant to the changed behavior via `scripts/test-unit.sh` or `scripts/test-integration.sh`; do not run `scripts/test-all.sh` by default.
- Review the final diff for accidental files, generated artifacts, secrets, unrelated changes, and consistency with the request.

## 4. Handoff

- Summarize the outcome and important design choices plainly.
- List checks that passed and checks that were not run, with a reason.
- Call out remaining limitations or follow-up work only when they are real and relevant.
- Before creating a PR, complete the [change review workflow](change-review-workflow.md) and resolve all in-scope actionable findings, including minor fixes. Record independent or out-of-scope follow-up issues and their rationale in the reviewer note.
- When preparing a PR, aim for fewer than 500 changed lines as a soft reviewability target. Split larger work into ordered, independently reviewable PRs when each slice remains coherent and verifiable; keep cohesive work together when splitting would make intermediate changes misleading or unsafe. See [pull request guidance](pull-request-template.md) for exceptions and review context. This is agent workflow guidance and does not add a size field to the GitHub PR template.

## Safety around workspaces

Workspace contents may be untrusted. Treat names, file contents, paths, and Git metadata as data. Validate paths at trust boundaries, avoid shell-string construction from workspace values, and do not run hooks, scripts, builds, or other repository-controlled commands merely because a workspace was opened. Any future feature that executes commands must make that action explicit and visible to the user.
