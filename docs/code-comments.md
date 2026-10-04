# Code comments

Comments should explain information that code alone cannot make clear: intent, invariants, non-obvious constraints, or a deliberate tradeoff.

- Prefer clear names and simple structure over comments that narrate each line.
- Add comments for non-obvious safety assumptions, path and process boundaries, concurrency invariants, and externally imposed constraints.
- Keep comments accurate as behavior changes; remove stale comments in the same change.
- Rust public APIs should use documentation comments (`///`) when their behavior is not self-evident or they are intended for use outside the defining module.
- Use Slint comments sparingly for design constraints or non-obvious layout decisions, not to repeat property names.
- Do not leave commented-out code, temporary debugging notes, or TODOs without enough context to act on them. Track substantial follow-up work in an issue or explicit task instead.
- Never put secrets, personal data, or sensitive environment details in comments.
