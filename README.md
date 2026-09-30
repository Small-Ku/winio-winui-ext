# winio-winui-ext

This workspace is a Windows-only downstream extension crate for [`winio`](https://crates.io/crates/winio). It hosts native WinUI 3 controls as ordinary Winio components through the public `winio-handle` escape hatches. The gallery is a small conformance app and runtime smoke target; it does not claim parity with the WinUI Gallery.

The bridge owns a native `FrameworkElement`, adds it to Winio's native canvas, retains only a weak parent reference, and removes the control when dropped. Wrappers implement Winio's component and widget traits and forward native events through `ComponentSender`. This keeps Winio and its backend unmodified while letting this crate add controls that matter to downstream applications.

## Compatibility

The workspace pins `winio = 0.12.0`, `winio-winui3 = =0.4.4`, and the Windows 0.62 type family. Keep these aligned: the supplemental projection must share the WinRT type universe used by the Winio backend. Do not independently upgrade the projection or introduce a second Windows type universe.

## Build and run

Install/activate the repository's `mise` tools, then use the checked-in tasks. Rust operations normally run through Mr Boxington (`mbx`):

```powershell
mise run doctor
mise run fmt
mise run check
mise run clippy
mise run test
mise run build
mise run smoke
```

`smoke` builds the Gallery and uses Windows UI Automation to verify the integrated native TitleBar plus InfoBar, NavigationView, NumberBox, TeachingTip, ToggleButton, and TreeView behavior. Normal builds consume committed generated Rust only and do not acquire WinMD metadata; metadata regeneration and comparison are separate gates:

```powershell
mise run bindings
mise run bindings-check
```

## Current scope

The reference wrappers are an integrated native WinUI `TitleBar`, a native `Primitives::ToggleButton`, plus supplemental InfoBar, NavigationView, NumberBox, TeachingTip, and TreeView controls.

ToggleButton preserves its two-state interaction default; the Gallery enables three-state mode and verifies a real `None`/indeterminate round-trip. Because the 0.4.4 generated `IsChecked` getter cannot represent a null `IReference<bool>`, that nullable read stays behind one narrow raw `IToggleButton` ABI call.

`IntegratedTitleBar` takes an ordinary Winio window through the public `AsWindow + AsContainer` bridge, extends the XAML client area into the title-bar region, and registers the native Windows App SDK `TitleBar` while preserving system caption controls. NavigationView emits stable selected item IDs through `ComponentSender`. NumberBox exposes value/range/small-change/spin-placement plus typed old/new `ValueChanged` events. TeachingTip targets an existing Winio widget through the public WinUI handle bridge and reports native close reasons from its popup lifecycle. TreeView accepts stable recursive node descriptors and emits stable IDs from native `Expanding` and `ItemInvoked` events. The Gallery drives all of these through their native UI Automation patterns and verifies the resulting Winio-owned window titles.

Supplemental bindings are no longer generated one control at a time. `winio-winui-bindings` builds one pinned broad XAML projection rooted at `Microsoft.UI.Xaml` and `Windows.UI.Xaml.Interop`, automatically references exact types already provided by `winio-winui3 0.4.4`, and follows bindgen dependency diagnostics to generate only missing supplemental dependencies. See [the supplemental bindings notes](docs/supplemental-bindings.md) for the projection boundary, provenance, and validation invariants. See [the control inventory](docs/control-inventory.md) for the distinction between upstream projection availability, supplemental projection availability, and wrapper coverage.

No mass wrapper set, custom-drawn substitutes, packaging, or release workflow is included in this round.
