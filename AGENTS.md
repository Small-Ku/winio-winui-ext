# Repository guidance

## Scope

winio-winui-ext extends upstream Winio with Windows App SDK / WinUI capabilities that are missing from the upstream public surface.

- Keep the dependency on upstream Winio.
- Do not fork or copy the Winio backend to add controls here.

Keep backend-neutral abstractions out of this crate.

- If a capability needs a cross-backend API, put it in a separate backend-neutral extension.
- Do not disguise a backend-neutral capability, such as generic component layering, as a WinUI-only control wrapper.

## Capability batches

Develop and review coherent capability batches. Do not land one `main` commit per projected WinUI type.

Each batch must have one clear runtime or API goal. It may include related wrappers, projection additions, Gallery samples, tests, and documentation changes.

Keep exploration local or on a branch while feasibility or value is unknown.
Open the first correct authority PR when a minimal runtime/canary slice shows all three conditions:
- the direction is technically feasible;
- it is materially useful;
- no known blocker is likely to overturn it.
Continue broader control coverage, polish, UIA expansion, cleanup, and review fixes on that PR. PR-ready does not mean implementation-complete or accepted.
Treat PR commits as review and evidence carriers. Additive review commits are expected.
The maintainer uses `Merge-NyaGitHubPullRequest` to squash the accepted tree.

Irreversible-number caution applies to duplicate, successor, or replacement PRs. Do not use it to delay the first correct PR.
Use small checkpoint commits while investigating. Keep unrelated cleanup out of the batch.

Do not disturb a dirty worktree that may belong to another session or agent.

- Never stash, clean, reset, switch away from, or overwrite foreign in-progress work.
- Use another worktree, or stop and inspect ownership first.

## Generated boundary

- `crates/winio-winui-bindings/src/generated.rs` is generated output.
- `crates/winio-winui-bindings/src/supplemental.types.txt` is the maintained supplemental projection inventory that drives generation.

- Do not hand-edit generated bindings.
- Change the projection inventory or generator, then run the binding tasks.
- Commit the generated diff with the capability that needs it.
- Keep handwritten ownership, events, conversions, and policy in `crates/winio-winui-ext`.

Use these gates when bindings change:

- mise run bindings
- mise run bindings-check
- mise run bindings-test

## Runtime acceptance

Compile-only projection is not sufficient evidence for a WinUI capability.

For a user-facing control or interaction:

1. expose a typed, ownership-safe API in winio-winui-ext;
2. add or extend a routed Gallery sample when the behavior can be represented there;
3. verify meaningful UI Automation peers, geometry, and state where UIA exposes them;
4. exercise the real interaction path for behavior that depends on pointer routing, focus, popup/flyout lifecycle, scrolling, clipping, or native composition;
5. verify resize/reflow and route replacement when ownership or layout can change.

Use `mise run ci` for the Rust baseline and `mise run smoke` for Gallery runtime acceptance.

A passing UIA `InvokePattern` is not proof of pointer hit-testing or stacking order. Use physical pointer input for those cases.

If a capability cannot be driven or observed reliably in the current runtime harness, record the concrete blocker instead of shipping a compile-only wrapper as implemented.

## Gallery ownership

Construct Winio children beneath their final native owner from the start. Do not reparent an already-owned Winio native element into another WinUI content host.

`GalleryPage` remains the page-level scroll owner. `GalleryExample` remains the card-local root.
Routed samples must keep stable sample IDs and generation-tagged output. This prevents delayed events from dropped samples from mutating the active route.

## Validation

Prefer the repository tasks over ad-hoc cargo command variants:

- mise run ci — format, clippy, unit tests;
- mise run build — Gallery/build acceptance;
- mise run smoke — Windows Gallery runtime/UIA acceptance;
- binding tasks above when projection changes.

Document a defer decision with the observed runtime/API constraint. Do not infer support from metadata presence alone.
