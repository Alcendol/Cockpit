---
name: address-review-feedback
description: Collect all available code review feedback, verify each comment against the current change, fix valid in-scope issues, and improve project guidance when feedback reveals a durable gap. Use when responding to PR or reviewer comments.
---

# Address Review Feedback

Actively process the complete review feedback for the current change. Read `AGENTS.md`, `docs/change-review-workflow.md`, and applicable linked guidance before editing. Keep the review workflow's detailed requirements authoritative.

## Collect the complete feedback set

1. Identify the review target and its source: current PR, review panel, supplied comments, or a local review document. Inspect available project/app context and tools to find all review threads, including resolved or outdated threads when accessible, all inline comments, and summary reviews.
2. Fetch or read every accessible comment and review body before deciding what to fix. If a source is inaccessible, name the source and say the set may be incomplete; continue with all feedback that is available.
3. Deduplicate comments that describe the same underlying issue, while preserving distinct acceptance criteria or scenarios.
4. Build a checklist of each distinct concern and its source/location so none is silently skipped.

## Validate each comment

For each concern:

1. Inspect the current code and diff, nearby call paths, tests, and relevant product or engineering guidance. Do not accept a comment solely because it came from a reviewer.
2. Determine whether it describes a real issue introduced by or still present in the reviewed change. Check the stated scenario and consequences against implementation behavior.
3. Classify it as valid and actionable, already addressed, not reproducible, pre-existing/out of scope, or requiring clarification. Explain the evidence for the classification. Do not change code for invalid or speculative feedback.
4. When a comment depends on an unclear product decision that materially changes behavior, continue independent items and ask for the missing decision before that dependent edit.

## Resolve valid issues

1. Fix all confirmed, actionable issues within the current unit of work, including minor fixes. Preserve intended behavior and keep changes focused.
2. Add or update a focused regression test for each bug fix where feasible. Use the narrowest relevant checks and test runners described by the workflow; do not run the full test suite by default.
3. Re-read the updated diff and re-evaluate every feedback item against the final code. Mark an item resolved only when the relevant behavior is corrected or the evidence shows no change is needed.
4. Do not post replies, resolve remote review threads, push, or make other external changes unless the user has authorized that action. Prepare a concise proposed response or resolution summary when useful.

## Improve durable guidance when warranted

After validating and resolving feedback, consider whether a valid comment reveals a recurring rule, an omitted acceptance condition, a misleading example, or a repeated source of defects. Update the narrowest relevant existing document under `docs/` only when that rule generalizes beyond this change and evidence supports it.

Prefer revising an existing section over adding duplicate guidance. Keep `AGENTS.md` as a short index; put detailed policy in its focused document. Do not turn a one-off preference, unsupported assumption, or local implementation detail into project-wide policy. If current guidance already covers the issue, fix the implementation and do not restate the rule elsewhere. Ensure cross-links and examples remain accurate.

## Handoff

Summarize the feedback sources inspected and whether the set was complete, each concern's disposition, code/tests/docs changed, exact checks run and outcomes, and any item deferred or needing user input. State clearly when a comment was not valid and cite the code or contract that establishes that conclusion.
