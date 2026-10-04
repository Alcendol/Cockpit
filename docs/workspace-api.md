# Workspace API contract

This document records the intended boundary for Cockpit's workspace-facing behavior. The current application is a starter shell; these are design constraints for implementation, not claims that workspace features already exist.

## Terms

- **Workspace**: a user-selected local directory Cockpit is currently working with.
- **Repository**: a workspace that contains a Git repository Cockpit can inspect.
- **Workspace path**: the canonicalized local filesystem path used to identify the selected directory for the current session.

## Boundary

- UI code requests workspace actions through a narrow Rust-facing interface; it should not implement filesystem or Git behavior itself.
- The Rust layer owns path validation, filesystem access, and Git integration. Keep process invocation behind a small adapter if external Git commands are needed.
- Represent expected failures explicitly (for example, inaccessible directory, non-repository directory, or Git unavailable) so the UI can show actionable feedback.
- Keep operations asynchronous when they can block the UI. Return owned data suitable for UI updates; do not expose borrowed filesystem or process state across the boundary.

## Path and process safety

- Treat a user-selected path and all repository contents as untrusted input.
- Canonicalize and validate paths at the boundary, and handle paths that disappear or become inaccessible after selection.
- Keep the original `PathBuf` (or another validated path value) as the navigation identity; treat names sent to the UI as display text only. Do not reconstruct paths from lossy display strings. If a filename cannot be represented by the UI's string type, handle that entry without failing the listing for its siblings.
- Pass process arguments as separate arguments; never interpolate workspace values into a shell command string.
- Do not run repository hooks, scripts, builds, or arbitrary commands as a side effect of opening or refreshing a workspace.
- Make any future command-execution capability explicit, scoped, and visible to the user before execution.

## Lifecycle and errors

- Workspace selection should have a clear success or failure result.
- For asynchronous workspace operations, apply a result only if it still belongs to the active workspace and requested location; discard stale results after navigation or workspace changes.
- Refresh should not silently switch the active workspace.
- Errors crossing into the UI should be concise and safe to display; detailed diagnostics may be logged without secrets or unnecessary file contents.
- Update this contract as the implementation establishes concrete Rust types and UI callbacks. Add examples of the public API once it exists.
