# Pull request guidance

Use the repository template at `.github/PULL_REQUEST_TEMPLATE.md` for each pull request. Keep it useful for reviewers: describe the result, the user or maintenance impact, how it was checked, and any meaningful risk.

## PR title

Use the same Conventional Commit shape as commit messages: `<type>(<optional scope>): <imperative summary>`. Keep the title concise and specific, without a trailing period. Examples: `feat(workspace): add repository picker`, `fix(ui): preserve selection on refresh`, or `docs: describe PR review workflow`. Use `!` for a breaking change and explain the impact in the PR description. See [commit conventions](commit-conventions.md).

Before creating the PR, complete the [change review workflow](change-review-workflow.md) against the target integration branch. Resolve all in-scope actionable findings, including minor fixes. If independent or out-of-scope findings remain, explain why they are deferred and describe any relevant risk in the reviewer note.

## PR size

Aim for fewer than 500 changed lines per PR, counting additions and deletions. Treat this as a reviewability guideline, not a hard limit: generated files, lockfile churn, and mechanical changes can inflate the count without adding equivalent review effort. Keep those changes focused and call out large generated or mechanical diffs so reviewers know how to assess them.

If a behavioral change pushes the diff past roughly 500 lines, look for independently reviewable slices that can be split into separate PRs. Keep required dependencies in a clear order and link prerequisite PRs. Do not split changes when doing so would leave intermediate PRs misleading, unsafe, or difficult to validate; instead explain why the larger PR is cohesive and identify the best review order.

PR size is more than a line count. A narrow change with clear boundaries is easier to review than a smaller diff that mixes unrelated behavior, formatting, renames, or generated output. Keep unrelated cleanup out of feature PRs, avoid unnecessary formatting churn, and summarize any unavoidable bulk changes.

## Changelog

Record user-visible changes in plain language. Write “None” when there is no user-visible change, such as for internal maintenance or documentation-only work. Do not repeat the full file list; reviewers can inspect the diff.

## Validation

List the checks actually run and their outcomes, including the focused change review. Include manual verification for UI or workflow changes. If a relevant check was skipped or blocked, name it and explain why. Never imply a check passed when it was not run.

## Merge and review order

- Keep the PR focused and explain any dependency on another PR.
- If review should follow a particular order, state it explicitly (for example, “review the API/types first, then the UI integration”). Otherwise write “No special order.”
- If the PR depends on another change, link that PR and explain whether it must merge first.
- Do not merge until required review and validation are complete. Use the repository’s configured merge strategy and required checks.

## Reviewer note

Add a note when the change has a notable risk, compatibility impact, migration concern, security-sensitive behavior, data-loss possibility, or area needing special scrutiny. Be specific about what to inspect and why. If there is nothing notable, write “None.” A reviewer note supplements the code review; it does not replace clear code, tests, or validation.
