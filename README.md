# winio-winui-ext

This workspace is a Windows-only downstream extension crate for [`winio`](https://crates.io/crates/winio). It hosts native WinUI 3 controls as ordinary Winio components through the public `winio-handle` escape hatches. The gallery is a small conformance app and runtime smoke target; it does not claim parity with the WinUI Gallery.

The bridge owns a native `FrameworkElement`, adds it to Winio's native canvas, retains only a weak parent reference, and removes the control when dropped. Wrappers implement Winio's component and widget traits and forward native events through `ComponentSender`. This keeps Winio and its backend unmodified while letting this crate add controls that matter to downstream applications.

## Compatibility

The workspace pins `winio = 0.12.0`, `winio-winui3 = =0.4.4`, and the Windows 0.62 type family. Keep these aligned: the generated WinUI projection/runtime generation must match the Winio backend. Do not independently upgrade the projection or introduce a second Windows type universe.

## Build and run

Install/activate the repository's `mise` tools, then use the checked-in tasks. Rust operations run through Mr Boxington (`mbx`):

```powershell
mise run doctor
mise run fmt
mise run check
mise run clippy
mise run test
mise run build
mise run smoke
```

`smoke` builds the Gallery and uses Windows UI Automation to verify the visible supplemental InfoBar canary as well as the native ToggleButton event path. Normal builds use committed generated Rust only and do not acquire WinMD metadata; metadata regeneration and comparison are separate gates:

```powershell
mise run bindings
mise run bindings-check
```

## Current scope

The reference wrappers are a native WinUI `Primitives::ToggleButton` and a supplemental InfoBar canary. ToggleButton preserves its two-state interaction default; the Gallery enables three-state mode and verifies a real `None`/indeterminate round-trip. Because the 0.4.4 generated `IsChecked` getter cannot represent a null `IReference<bool>`, that nullable read stays behind one narrow raw `IToggleButton` ABI call. InfoBar is generated from a pinned Windows App SDK metadata baseline and reuses shared types from `winio-winui3 0.4.4`; details and provenance are in [the supplemental bindings notes](docs/supplemental-bindings.md). See [the control inventory](docs/control-inventory.md) for projected controls and the canary status.

No mass wrapper set, custom-drawn substitutes, packaging, or release workflow is included in this round.
