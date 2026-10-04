# Engineering guidance

This directory contains the detailed working agreements referenced by the repository's [`AGENTS.md`](../AGENTS.md). Read the docs relevant to your task; the developer workflow and iteration shape apply to every change.

Slash-activatable agent skills for these workflows live in [`../.agents/skills/`](../.agents/skills/). Use `/change-review` to carry out change review and `/address-review-feedback` to collect, validate, and resolve reviewer feedback.

| Document | Use it for |
| --- | --- |
| [Product plan](product-plan.md) | Core capabilities, scope boundaries, and suggested delivery order |
| [Project files and basic editing](features/project-files-and-editing.md) | Goal, problem, solution, use cases, acceptance criteria, and scope for core feature 1 |
| [Developer workflow](developer-workflow.md) | Planning, implementation, verification, and handoff |
| [Change review workflow](change-review-workflow.md) | Review against the integration branch and select focused checks and tests |
| [Iteration shape](iteration-shape-guidance.md) | Keeping changes small, coherent, and reviewable |
| [Unit of work](unit-of-work.md) | Defining independently implementable and verifiable slices |
| [Constraints and acceptance](constraints-and-acceptance.md) | Recording applicable constraints and mapping acceptance criteria to verification |
| [Testing strategy](testing-strategy.md) | Choosing test layers and meaningful behavior coverage |
| [Naming guidance](naming-guidance.md) | Rust, Slint, file, and domain naming |
| [UI components](ui-components.md) | Slint component composition and Rust/UI responsibilities |
| [Repository layout](repository-layout.md) | Where code and documentation belong |
| [Architecture](architecture.md) | Application layers, responsibilities, and data flow |
| [Lockfile policy](lockfile-policy.md) | Dependency and `Cargo.lock` changes |
| [Code comments](code-comments.md) | When and how to comment code |
| [Commit conventions](commit-conventions.md) | Commit message format and scope |
| [Pull request guidance](pull-request-template.md) | PR size, changelog, validation, review order, and reviewer notes |
| [Developer runbook](developer-runbook.md) | Setup, running, checks, and troubleshooting |
| [Workspace API](workspace-api.md) | Contract for opening and representing a user workspace |
