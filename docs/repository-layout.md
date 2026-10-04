# Repository layout

Keep the repository legible and group files by responsibility.

See [architecture](architecture.md) for the intended Rust, Slint, and workspace service boundaries.

```text
.
├── AGENTS.md             # Short agent and contributor entry point
├── docs/                 # Detailed engineering and product contracts
├── src/                  # Rust application code
│   ├── project.rs
│   └── project/tests.rs   # Separate unit-test module for src/project.rs
├── ui/                   # Slint UI definitions
├── tests/                # Rust integration tests, in separate files
├── scripts/              # Repository-wide developer scripts
├── build.rs              # Slint build integration
├── Cargo.toml            # Crate metadata, dependencies, lint policy
├── Cargo.lock            # Locked application dependency graph
├── rustfmt.toml          # Rust formatting configuration
└── README.md             # Project overview and quick start
```

- Put application behavior in `src/`, not in `build.rs`.
- Put reusable UI composition and visual structure in `ui/`; keep business behavior in Rust where practical.
- Follow [UI component guidance](ui-components.md) when adding Slint components or connecting them to Rust.
- Put durable developer or architecture guidance in `docs/` and link it from `AGENTS.md` or `docs/README.md`.
- Keep test cases out of production source files. Put unit tests in separate child files under the corresponding module directory and integration tests in separate files under top-level `tests/`; see [testing strategy](testing-strategy.md).
- Put repository-wide scripts in `scripts/`; place scripts owned by a backend or frontend inside that component's own `scripts/` directory.
- Do not commit build output, local IDE state, credentials, or machine-specific configuration.
- Introduce new top-level directories only when there is a clear ownership boundary and a real need.
