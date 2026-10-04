# UI components

Cockpit uses Slint, a declarative, component-based UI toolkit. A Slint component packages UI elements, properties, callbacks, and behavior into a reusable unit. `MainWindow` in `ui/main.slint` is the current exported root component; the project may add focused components as the UI grows.

## Component boundaries

- Keep `MainWindow` responsible for application-level composition and window configuration. Move a distinct, reusable visual responsibility into its own component when that improves clarity or gives it a focused interface.
- Prefer components organized around a recognizable UI responsibility (for example, a repository selector or change list) rather than splitting every small arrangement of widgets into a component.
- Keep a component's public interface small. Expose only the properties and callbacks needed by its parent; keep implementation details private.
- Place shared components in a clear location under `ui/` when they are reused. Keep a one-off component near its owning screen until reuse or size justifies a shared module/file.
- Follow [naming guidance](naming-guidance.md): exported component types use `UpperCamelCase`; boolean state reads as a predicate; callbacks describe actions.

## Frontend state and behavior

- Slint is the frontend layer and may contain UI state and presentation logic: properties, callbacks, functions, bindings, and conditional rendering.
- Keep small, local conditions in the UI, such as choosing between content and “No Data Available” for an empty collection.
- When a page's UI file grows because layout and substantial frontend behavior are mixed together, split the page into same-folder Slint files. Keep the page/view structure in its UI file and move cohesive frontend state or presentation behavior into a companion `.slint` component/helper file. Compose or import the companion through Slint's normal component mechanisms.
- Make a split when it clarifies responsibility or reduces file growth, not for every condition or a few lines of bindings. Keep related page-specific files together; promote a component to a shared UI location only when it is actually reused.
- Give each piece of frontend state one clear owner. Pass state through explicit properties and user intent through callbacks; avoid maintaining duplicate copies without a defined synchronization direction.
- Keep filesystem, Git, process, persistence, and other operating-system or application-service responsibilities in Rust. Connect those through explicit, typed component properties and callbacks rather than reproducing those responsibilities in Slint.
- Do not perform potentially slow filesystem or process work directly in a Slint callback. Run it away from the UI event loop, then dispatch owned results back through Slint's event-loop API. Before applying a result, verify that it still corresponds to the current workspace and requested location.
- Represent loading, empty, success, and error states explicitly when relevant to the page or component.
- Keep components deterministic from their declared inputs where practical; this makes them easier to reuse, preview, and test.

## Composition and layout

- Compose larger screens from standard Slint widgets and Cockpit components; avoid duplicating substantial markup for the same visual behavior.
- Use layout containers and constraints rather than fixed positioning when content or window size can vary.
- Consider minimum and useful window sizes, text expansion, empty content, and resizing when designing a component.
- Bound dynamic text such as filesystem paths with wrapping or elision so long values do not set an excessive minimum width or hide required controls. Inspect the layout at a narrow window size as well as a typical size.
- Keep visual styling consistent with existing UI tokens and patterns. Introduce shared style properties or helpers when repeated values acquire a meaningful design role; do not create a design system for isolated values without a use case.
- Keep accessibility and interaction states visible: controls should have clear labels, disabled states should reflect actual availability, and focus/keyboard behavior should remain usable.

## Rust integration and verification

- Rust handles application services and external side effects; Slint handles frontend state and presentation behavior. Keep the boundary explicit without moving ordinary UI conditions into Rust.
- Avoid exposing internal Rust implementation types directly to UI unless they form a stable, UI-appropriate model. Convert service/domain results into concise presentation state at the boundary.
- Keep page-specific Slint companion files in the same folder as their page UI file. For example:

  ```text
  ui/
  └── workspace/
      ├── page.slint          # Page composition and layout
      ├── page-state.slint    # Cohesive page-specific frontend state/behavior
      └── components/         # Components used only within this page
  ```

- Update `build.rs` or component compilation setup only when required by the Slint structure; the current build script compiles `ui/main.slint`.
- For a UI change, compile and type-check with `cargo check --all-targets --all-features`, then launch the app and inspect the affected component at a useful window size. Check relevant interactions and states; see [testing strategy](testing-strategy.md) and [developer runbook](developer-runbook.md).

This guide describes conventions for future UI work. The current starter interface is still a single root component and does not yet define a shared component library or design-token system.
