# Supplemental WinUI bindings

`winio-winui-bindings` is the checked-in supplemental WinUI projection used by `winio-winui-ext`. It exists because `winio-winui3 0.4.4` exposes only the WinUI surface needed by that crate, while downstream applications may need additional native controls.

The projection is broad at the XAML boundary, not broad across all Windows App SDK metadata. Its root filters are:

- `Microsoft.UI.Xaml`
- `Windows.UI.Xaml.Interop`

Additional Microsoft types are generated only when bindgen reports them as dependencies of that root surface. Ordinary `Windows.*` dependencies are referenced from the workspace Windows 0.62 family rather than regenerated locally.

## Shared-type partition

The generator resolves the exact `winio-winui3 0.4.4` source and inventories its public WinRT types. The crates.io VCS metadata for that release points to `compio-rs/winui3-rs` commit `c0cc028301fb409d5e21f158a5b86d5661ba8e4c`, path `winui3`.

Generation uses two related passes. A metadata-surface pass inventories the pinned XAML root without treating upstream types as already satisfied. The final pass then partitions that surface and grows a dependency closure.

For the final pass:

- a root type already present in `winio-winui3` becomes an exact `--reference winio_winui3,full,<type>` and is excluded from local generation;
- a missing Microsoft dependency already present upstream becomes another exact upstream reference;
- a missing Microsoft dependency absent upstream becomes an exact supplemental filter;
- standard `Windows.*` dependencies resolve to `windows`, `windows-collections`, `windows-future`, or `windows-numerics` as appropriate.

The dependency closure is diagnostic-driven rather than maintained as a per-control allowlist. Bindgen is rerun until no dependency warnings remain and no newly emitted Microsoft type overlaps the upstream inventory.

The generator enforces these invariants:

- the complete root metadata inventory is partitioned between exact upstream references and locally generated root types;
- upstream references and all locally generated types are disjoint;
- no ordinary `Windows.*` type is generated locally, with `Windows.UI.Xaml.Interop` being the intentional exception;
- the sanitizer must not change the generated type inventory;
- wrapper contracts assert the runtime types and methods required by InfoBar, NavigationView, NumberBox, TeachingTip, and TreeView.

The generated manifests make that boundary reviewable. `upstream.references.txt` records every exact upstream type used by the final projection; `supplemental.types.txt` records every locally generated type; and `bindings.lock.toml` records counts, hashes, sanitizer statistics, and the final warning count.

At the current pinned baseline the generated provenance reports 3,076 root metadata types, 425 exact upstream references, 3,122 locally generated supplemental types, 380 dependency filters, and zero final bindgen warnings. These numbers are generated evidence, not hard-coded API requirements.

## Generator and metadata provenance

The generator is Berrysoft's `overrides-windows-bindgen 0.66.0`, the same generator family used to produce `winio-winui3 0.4.4`.

The upstream crate does not commit its WinMD inputs and does not identify a Windows App SDK metadata version. This repository therefore does not claim that upstream 0.4.4 was generated from the same WinMD set used here.

This repository pins Windows App SDK `2.5.1` as its reproducible supplemental metadata baseline. The recorded package set currently includes:

- `Microsoft.WindowsAppSDK 2.5.1`
- `Microsoft.WindowsAppSDK.Base 2.0.4`
- `Microsoft.WindowsAppSDK.Foundation 2.3.12`
- `Microsoft.WindowsAppSDK.InteractiveExperiences 2.1.9`
- `Microsoft.WindowsAppSDK.WinUI 2.3.9`
- `Microsoft.Web.WebView2 1.0.3719.77`, resolved from the WinUI package dependency

Foundation and WinUI consume their metadata inputs directly. `InteractiveExperiences` contains multiple versioned metadata sets, so the tool deterministically selects the numerically newest metadata directory; for the current package that is `metadata/10.0.18362.0`. This avoids mixing the 17763 and 18362 sets according to ZIP entry order.

`bindings.lock.toml` records package SHA-256 values plus the exact source path and SHA-256 of every WinMD supplied to bindgen. NuGet packages and WinMD files themselves remain outside the repository. Normal builds use only committed generated Rust and perform no metadata download.

The Windows App SDK 2.5.1 choice is a pinned projection/provenance baseline, not a claim that this crate's minimum supported runtime is exactly 2.5.1.

## Generated-source sanitization

Exact cross-crate references expose a limitation of generated convenience APIs: bindgen can emit inherited forwarding methods on a supplemental derived class that immediately cast to a base class now owned by `winio-winui3`. Those methods are not part of the derived class's own WinRT interface and can become invalid or duplicative once the base type lives in another crate.

The generator therefore performs a bounded post-generation sanitization pass. An inherited forwarder is removed only when its body contains all three structural markers:

- `windows_core::Interface::cast`
- a `winio_winui3::...` target
- `windows_core::Interface::vtable`

It also removes a small number of generated iterator conveniences whose required `First` method is no longer present locally, and unresolved `required_hierarchy!` entries that refer to types absent from the final reference/generated inventory. Regression tests cover these rewrites, and generation fails if sanitization changes the type inventory.

The sanitizer counts are recorded in `bindings.lock.toml`. They are diagnostic evidence rather than public API commitments.

## Wrapper contracts and native smoke coverage

The broad projection is intentionally larger than the wrapper layer. A type appearing in `supplemental.types.txt` does not mean this crate has designed, documented, or tested a Winio wrapper for it.

The current wrapper contracts cover:

- TitleBar: title/subtitle plus native activation for `Window.SetTitleBar`;
- InfoBar: message/open/closable properties;
- NavigationView: menu items, selection, pane state, and `SelectionChanged`;
- NumberBox: value/range/small-change/spin placement and `ValueChanged` old/new values;
- TeachingTip: title/subtitle/open state, target, preferred placement, close-button content, and native `Closed` reason;
- TreeView: root nodes, selection mode, node content/children, `Expanding`, and `ItemInvoked`.

The Gallery UI Automation smoke test proves these controls are native and that their event paths return through `ComponentSender` into Winio state. It currently verifies:

- the native `Microsoft.UI.Xaml.Controls.TitleBar` is registered as the integrated window title bar while system caption controls remain present;
- the InfoBar canary is visible in the native UIA tree;
- ToggleButton changes native toggle state and updates the Winio-owned window title;
- NavigationView selects `Details` through `SelectionItemPattern` and emits the stable `details` ID;
- NumberBox exposes the expected `RangeValuePattern`, accepts `42`, and emits that value;
- TeachingTip opens as a native popup targeted at the ToggleButton, closes through its UIA close-button `InvokePattern`, disappears from the UIA tree, and emits `CloseButton`;
- TreeView expands the root through `ExpandCollapsePattern` and emits stable ID `root`;
- TreeView invokes the leaf through `InvokePattern` and emits stable ID `leaf`.

WinUI's content-mode `TreeViewItemAutomationPeer` currently exposes a generic automation name (`Microsoft.UI.Xaml.Controls.TreeViewNode`) for these canary nodes, so the smoke test identifies the native TreeView container by control type/class and verifies stable identity through the wrapper event result instead of claiming a stable node-name automation contract.

## Reproduction and validation

Metadata acquisition and bindgen run only through the explicit bindings tasks. Normal library and Gallery builds consume committed generated Rust.

```powershell
mise run bindings        # acquire pinned metadata and regenerate committed Rust/manifests
mise run bindings-check  # regenerate and byte-compare all generated artifacts
mise run bindings-test   # inventory/sanitizer regression tests
mise run ci              # formatting, Clippy, and workspace tests
mise run smoke           # build and run the native UI Automation Gallery smoke
mise run dependency-audit
```

`bindings-check` is the primary reproducibility gate: it recomputes the root inventory and diagnostic dependency closure, verifies the projection invariants and wrapper contracts, and byte-compares the generated Rust, provenance lock, upstream-reference manifest, and supplemental-type manifest.
