# Product plan

Cockpit is a lightweight, agent-first coding workspace. Its main job is to help a developer open a project, inspect and control code changes, and make small follow-up edits. The plan prioritizes a useful end-to-end workflow while keeping each delivery small enough to build and review independently.

## Product principles

- Keep the everyday workflow fast and resource-conscious.
- Make reviewing and finishing changes the center of the experience.
- Keep agent-produced changes understandable and under the user's control.
- Make file changes and command execution explicit; never run project commands just because a project was opened.

## Core capabilities

### 1. Project files and basic editing

**Outcome:** Open a local folder as a project, navigate its files, and make small edits.

1. Open a folder and show the active project and its file tree.
2. Navigate directories and open a file in the editor.
3. Edit and save a file, with clear unsaved-change state and protection from accidental loss.
4. Create and delete files with clear feedback and confirmation where deletion could lose user data.
5. Support multiple open files with tabs and predictable tab switching/closing behavior.
6. Provide essential editor usability: line numbers, undo/redo, common keyboard shortcuts, and basic syntax highlighting.

Keep the first editor focused on code review and small edits. Defer extensions, language servers, advanced refactoring, and broad IDE configuration.

### 2. Git change review and actions

**Outcome:** Understand and control project changes, including changes made by an agent.

1. Show repository state and changed files, distinguishing staged, unstaged, untracked, and conflicted files.
2. Show readable diffs with enough surrounding context; support navigation across files and change hunks.
3. Stage and unstage whole files; restore a file's working-tree changes with clear impact feedback.
4. Stage and unstage selected lines or hunks; remove selected lines from the working changes.
5. Detect merge conflicts and provide a guided way to resolve them, then show the resolved result for review.

Keep Git operations explicit and make destructive actions such as restore clear before applying them. Initially focus on the current working tree and index. Branch management, commit history, and other repository administration can be considered separately.

### 3. Integrated terminal

**Outcome:** Run user-requested commands in the active project without leaving Cockpit.

1. Open a terminal whose initial working directory is the active project.
2. Support interactive command input and readable output, including exit status.
3. Support more than one terminal session and let the user close sessions.

Terminal commands are always user initiated and visible. Agent command execution, background task orchestration, and automated project scripts are outside this initial capability.

## Supporting workflow essentials

These are small features that make the core capabilities practical; they do not need to become large standalone subsystems.

- Find text in the current file.
- Search across project files, with results that open the matching file and location.
- Show useful, actionable messages for file, save, Git, and terminal errors.
- Preserve clear state for unsaved editor buffers and staged/unstaged Git changes.

Project-wide search can follow the initial project/file-tree slice if it would delay getting the basic workflow working.

## Suggested delivery order

1. **Project foundation:** choose a folder, validate it, retain active-project state, and render a file tree.
2. **Read and edit files:** open one file, edit/save, show unsaved state, and protect unsaved content when switching or closing.
3. **Editor essentials:** tabs, undo/redo, line numbers, shortcuts, basic syntax highlighting, and find in file.
4. **Git review foundation:** repository detection, status, file list, and contextual diff viewing.
5. **Safe Git actions:** file-level stage/unstage and restore, with clear status updates.
6. **Precise Git actions:** hunk/line staging and removal of selected changes.
7. **Conflict resolution:** conflict detection, guided resolution, and review of the resulting diff.
8. **Integrated terminal:** project-scoped interactive sessions and exit status.
9. **Project-wide search:** searchable files and navigable results.

Each delivery should be an end-to-end slice with a visible user outcome. Reorder after validating the workflow; this is a scope guide, not a commitment to a release schedule.

## Not in the initial scope

- Full IDE capabilities such as extensions, language servers, debugging, and advanced refactoring.
- Branch/commit-history management and remote hosting workflows.
- Automatic agent execution or running project commands when opening a folder.
- Multi-project workspace orchestration.

These may become useful later, but are not required to establish Cockpit's core workflow.
