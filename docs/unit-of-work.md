# Unit of work

A **unit of work** is the smallest coherent behavior slice that can be understood, implemented, reviewed, and verified as a meaningful change. It is a planning and delivery boundary, not a Rust function, a test case, or a fixed number of changed lines.

## Shape

A good unit of work follows [constraints and acceptance guidance](constraints-and-acceptance.md) to state its observable outcome and applicable constraints. It:

- Delivers one observable behavior or establishes one necessary architectural boundary.
- Has a clear start and end, with inputs, outcomes, and relevant failure behavior that can be described plainly.
- Can be validated on its own, with the appropriate focused tests and checks.
- Changes only the code, UI, tests, and documentation needed for that behavior.
- Leaves the repository in a coherent state; intermediate work should build and remain understandable whenever practical.

Examples:

- Define a typed workspace-selection result and cover its path validation behavior.
- Add the Rust-to-Slint state binding for one workspace status and verify its loading, success, and error display.
- Implement one Git status parser with focused unit tests, then integrate it with repository refresh in a separate slice if the boundary is substantial.

## Sizing

- Prefer one behavior slice per reviewable iteration. A user feature may need several such slices and more than one PR.
- Do not split a behavior merely by file or function if the pieces cannot be meaningfully understood or tested independently.
- Split large work at stable boundaries such as domain logic, persistence/process adapter, UI integration, or a separately useful behavior.
- Keep prerequisites explicit and ordered when one slice depends on another.
- Use the PR size target in [pull request guidance](pull-request-template.md) as a reviewability signal, not as the definition of a unit. A small line count can still mix unrelated behavior; a cohesive slice can exceed the target due to generated or unavoidable mechanical changes.

## Completion check

Before calling a unit complete, be able to state:

1. What behavior or boundary did it add or change?
2. What are its meaningful normal, boundary, and failure outcomes?
3. Which acceptance criteria and focused tests or manual checks establish those outcomes?
4. Does it leave code, UI, and documentation in a coherent state?

If those answers span several unrelated behaviors, consider splitting the work. If a proposed split leaves a piece with no meaningful behavior or verification, keep the pieces together.
