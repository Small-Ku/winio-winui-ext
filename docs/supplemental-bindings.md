# Supplemental WinUI bindings

`winio-winui-ext` uses the generated Rust bindings in `crates/winio-winui-bindings/src/generated.rs` to call WinUI types that `winio-winui3` does not expose. Consumers compile the checked-in Rust file; they do not need the WinMD files or binding generator.

The current Cargo dependency is `winio-winui3 0.4.5`. The checked-in bindings were generated using the `0.4.4` type inventory as a baseline. See `bindings.lock.toml` for the recorded generator inputs and versions. Do not mistake that generation baseline for the current Cargo dependency.

## Which types the generator includes

The generator starts with two WinMD namespace filters:

- `Microsoft.UI.Xaml`
- `Windows.UI.Xaml.Interop`

It then adds the Microsoft types required by those roots. Other standard `Windows.*` types use the workspace's `windows`, `windows-collections`, `windows-future`, or `windows-numerics` crates instead of locally generated definitions.

Not every generated type has a Winio wrapper. The [control inventory](control-inventory.md) records wrapper coverage separately.

### Avoiding duplicate upstream types

The generator reads the pinned `winio-winui3 0.4.4` source. That published version points to `compio-rs/winui3-rs` commit `c0cc028301fb409d5e21f158a5b86d5661ba8e4c`, under `winui3`.

Generation runs two passes:

1. Read all types under the root filters from the pinned metadata, without excluding upstream types.
2. Generate the missing types, referring to types already supplied by `winio-winui3` rather than defining them again. Repeat when bindgen reports additional missing dependencies.

The second pass applies these rules:

- Use an exact `--reference winio_winui3,full,<type>` for types already present upstream.
- Add a supplemental filter for required Microsoft types not present upstream.
- Resolve standard `Windows.*` dependencies through the workspace Windows crates.
- Stop only when bindgen reports no unresolved dependencies.

The generator checks that every root type belongs to exactly one set: an upstream reference or a generated definition. It also rejects duplicate types, unexpected locally generated `Windows.*` types, and changes to the type list during source cleanup. `Windows.UI.Xaml.Interop` is the deliberate exception to the ordinary `Windows.*` rule.

These files record the result:

| File | Purpose |
| --- | --- |
| `upstream.references.txt` | Exact types supplied by `winio-winui3` |
| `supplemental.types.txt` | Types defined in the generated Rust file |
| `bindings.lock.toml` | Package versions and hashes, input WinMD hashes, generation counts, and warning counts |
| `generated.rs` | Rust types compiled by the workspace |

The current lock records 3,076 metadata root types, 425 upstream references, 3,122 locally generated types, 380 dependency filters, and zero final bindgen warnings. These are results of this generation run, not fixed API requirements.

## Where the metadata comes from

`tools/bindings` uses `overrides-windows-bindgen 0.66.0` and a pinned Windows App SDK `2.5.1` package set:

| NuGet package | Recorded version |
| --- | --- |
| `Microsoft.WindowsAppSDK` | `2.5.1` |
| `Microsoft.WindowsAppSDK.Base` | `2.0.4` |
| `Microsoft.WindowsAppSDK.Foundation` | `2.3.12` |
| `Microsoft.WindowsAppSDK.InteractiveExperiences` | `2.1.9` |
| `Microsoft.WindowsAppSDK.WinUI` | `2.3.9` |
| `Microsoft.Web.WebView2` | `1.0.3719.77` |

The tool resolves component versions from the Windows App SDK umbrella package. It reads the WebView2 version from the WinUI component's NuGet dependencies.

`InteractiveExperiences` contains multiple metadata directories. The generator chooses the numerically latest directory; the pinned package uses `metadata/10.0.18362.0`. The Base package is recorded even though it contains no WinMD input.

`bindings.lock.toml` records each NuGet package's SHA-256 and every WinMD input's path and SHA-256. The repository does not commit the NuGet packages or original WinMD files. The upstream `winio-winui3 0.4.4` package does not identify its original Windows App SDK metadata version, so the generator does not claim that both projections used identical metadata.

Windows App SDK `2.5.1` is the input version used by this generator. It does not establish the minimum Windows App SDK runtime version supported by the wrapper library.

## Why generated methods are removed

When a derived WinUI class is defined locally but its base class comes from `winio-winui3`, bindgen can generate inherited convenience methods that cast to an upstream base and call its vtable. Those forwarded methods may duplicate another crate's methods or reference unavailable types.

`tools/bindings` removes a generated inherited forwarder only when its body contains all three markers:

- `windows_core::Interface::cast`
- a `winio_winui3::...` target
- `windows_core::Interface::vtable`

The generator also removes generated iterator helpers with missing local `First` methods, and `required_hierarchy!` entries referring to unavailable types. Its tests cover these changes and reject unexpected changes to the generated type inventory. The lock records the count of removed items.

Do not hand-edit `generated.rs`. Modify the generator or its maintained input files and regenerate the output instead.

## What the native tests check

The generator checks that the Rust projection supplies the types and methods required by `TitleBar`, `AutoSuggestBox`, `InfoBar`, `NavigationView`, `NumberBox`, `TeachingTip`, and `TreeView`.

The Gallery smoke test separately checks these native runtime behaviors:

- `TitleBar` registers with the window while system caption controls remain visible.
- `InfoBar` appears in the UI Automation tree.
- `AutoSuggestBox` accepts and submits `TreeView`; the Winio window title reflects the submitted value.
- `ToggleButton` updates its native toggle state and Winio output.
- `NavigationView` selects the nested `Details` item and reports its stable ID.
- `NumberBox` accepts `42` through `RangeValuePattern` and reports the change.
- `TeachingTip` opens as a popup, closes through its UIA button, and reports `CloseButton`.
- `TreeView` expands `root` and invokes `leaf`, reporting their IDs.

The Gallery keeps the search box under its existing Winio Canvas rather than moving it into `NavigationView.AutoSuggestBox`. Winio has already attached that native element to its parent.

Content-mode `TreeViewItemAutomationPeer` reports the generic name `Microsoft.UI.Xaml.Controls.TreeViewNode` in these samples. The smoke test identifies the container by its native control type/class and checks node identity through the wrapper's emitted IDs, not through a presumed node-name automation contract.

## Regenerate and check

Run these tasks on the supported Windows development environment:

```powershell
mise run bindings        # regenerate Rust bindings and recorded inputs
mise run bindings-check  # regenerate and compare with committed files
mise run bindings-test   # run generator and cleanup regression tests
mise run ci              # check Rust formatting, Clippy, and unit tests
mise run smoke           # run Windows Gallery runtime/UIA checks
mise run dependency-audit
```

`bindings-check` compares the regenerated Rust source, metadata lock, upstream references, and supplemental type list against committed files. Normal builds use the existing Rust output and do not download metadata.
