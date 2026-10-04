# Naming guidance

Use names that describe purpose and remain consistent across Rust, Slint, and documentation.

## Rust

- Modules, functions, methods, variables, and fields use `snake_case`.
- Types and traits use `UpperCamelCase`.
- Constants and statics use `SCREAMING_SNAKE_CASE`.
- Use domain terms consistently; avoid unexplained abbreviations and vague names such as `data`, `manager`, or `util` when a precise role is known.
- Keep module and file names aligned with their primary responsibility.
- Name boolean-returning checks as predicates, usually with an `is_`, `has_`, or `should_` prefix:
  - `is_` for a state or classification (`is_repository`, `is_empty`).
  - `has_` for possession or presence (`has_changes`, `has_permission`).
  - `should_` for a decision or recommendation (`should_refresh`, `should_retry`).
- Choose the prefix that expresses the question clearly; use another established predicate form such as `contains_`, `supports_`, or `can_` when it is more precise. The name should make it obvious that the function answers a yes/no question.
- Do not use predicate prefixes for commands or operations that perform an action. Use an imperative verb (`open_workspace`, `refresh_status`) and return a result that describes success or failure.
- Apply the same distinction to Slint callbacks and properties: predicates should read as boolean state; event handlers should describe the action they handle or trigger.

## Slint and user-facing terms

- Follow Slint's established syntax and naming conventions; exported components use `UpperCamelCase`.
- Use concise labels that describe the action or state. Keep internal implementation names out of user-facing copy.
- Use one term for one concept across controls, documentation, and Rust APIs (for example, choose either “workspace” or “project” for the same entity and use it consistently).

## Files and concepts

- Use lowercase kebab-case for standalone documentation filenames.
- Name concepts after stable responsibilities, not current implementation details.
- Avoid creating near-synonyms for existing domain concepts. If a term must change, update its usages and docs together.
