# Commit message conventions

Use the Conventional Commits shape:

```text
<type>(<optional scope>): <imperative summary>

<optional body>
```

Common types:

- `feat`: user-visible capability
- `fix`: bug fix
- `docs`: documentation only
- `refactor`: behavior-preserving code restructuring
- `test`: test-only changes
- `build`: build system or dependency changes
- `chore`: maintenance that does not fit another type

Use a short, specific summary in the imperative mood, without a trailing period. Scope is optional; when useful, use a stable area such as `ui`, `workspace`, or `docs`. Explain motivation and notable consequences in the body rather than listing every edited file. Mark breaking changes with `!` after the type/scope and explain the migration in the body.

Examples:

```text
feat(workspace): add repository picker
fix(ui): preserve selection when refreshing changes
docs: describe workspace API contract
```
