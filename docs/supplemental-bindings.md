# Supplemental WinUI bindings

`winio-winui-bindings` holds a narrow generated projection for WinUI controls that
are absent from `winio-winui3 0.4.4`. The current canaries are a narrow
`Microsoft.UI.Xaml.Controls.InfoBar` surface, a minimal
`Microsoft.UI.Xaml.Controls.NavigationView` surface, and a numeric-input
`Microsoft.UI.Xaml.Controls.NumberBox` surface.

The projection deliberately reuses `winio-winui3` for shared XAML/base types such
as `DependencyObject`, `UIElement`, `FrameworkElement`, and `Control` through
`overrides-windows-bindgen` references. It does not create a second complete
`Microsoft.UI.Xaml` type universe. Generator and crate tests fail if those shared
base types are accidentally emitted locally.

## Generator and upstream provenance

The generator is Berrysoft's `overrides-windows-bindgen 0.66.0`, the same
generator family used to produce `winio-winui3 0.4.4`. The crates.io VCS metadata
for that release points to `compio-rs/winui3-rs` commit
`c0cc028301fb409d5e21f158a5b86d5661ba8e4c`, path `winui3`.

That upstream source does not commit the WinMD inputs and does not pin the
Windows App SDK metadata version used for generation. Therefore this repository
does not claim that the upstream 0.4.4 bindings were generated from any specific
Windows App SDK release.

This repository instead pins Windows App SDK `2.5.1` as its own reproducible
canary metadata baseline. The `Microsoft.WindowsAppSDK` metapackage resolves to:

- `Microsoft.WindowsAppSDK.Foundation 2.3.12`
- `Microsoft.WindowsAppSDK.InteractiveExperiences 2.1.9`
- `Microsoft.WindowsAppSDK.WinUI 2.3.9`

Foundation and WinUI consume their direct `metadata/*.winmd` inputs.
`InteractiveExperiences` contains multiple versioned metadata sets, so the tool
deterministically selects the numerically newest metadata subdirectory; for the
current package that is `metadata/10.0.18362.0`. This matches the selection
strategy used by Microsoft's Reactor generator and avoids ZIP-entry-order mixing
between the 17763 and 18362 metadata sets.

`crates/winio-winui-bindings/src/bindings.lock.toml` records the umbrella and
component package hashes plus the exact source path and SHA-256 of every WinMD
actually supplied to bindgen. WinMD and NuGet payloads themselves remain outside
the repository.

The 2.5.1 choice is an experimental provenance baseline for this canary, not yet
the library's minimum-runtime compatibility policy. It also matches the V2
runtime line available on the development machine and Winio 0.12's V2-first
runtime bootstrap.

## Canary surface

The InfoBar canary proves only the small surface required by the wrapper:

- `Message` / `SetMessage`
- `IsOpen` / `SetIsOpen`
- `IsClosable` / `SetIsClosable`
- activation and COM/WinRT cast to the existing `winio-winui3` `FrameworkElement`

`overrides-windows-bindgen 0.66.0` reports dependency-closure warnings for other
InfoBar members and inherited control APIs whose dependency types are not part
of this narrow projection, such as action-button/template/statics and close-event
surface. The generator treats those as non-canary omissions, fails if a warning
affects a promised canary method, and verifies required methods inside the
generated implementation for their owning runtime class. This crate must not be
described as a complete projection for these controls.

NavigationView has a minimal canary surface for menu items, item content and
tags, pane display mode, pane open state, selected item, and `SelectionChanged`.
SelectionChanged is supported because the generator references
`Windows.Foundation.TypedEventHandler` to the existing `windows` crate, reusing
the workspace Windows.Foundation projection. The wrapper emits the selected
stable item id through Winio's `ComponentSender` and removes its event token on
drop.
NumberBox adds a distinct numeric-input canary. Its promised surface includes
minimum/maximum/value/small-change, spin-button placement, and native
`ValueChanged` registration/removal; `NumberBoxValueChangedEventArgs` exposes
both old and new `f64` values. The wrapper maps the generated placement enum to
a small public Rust enum and emits typed value-change events through
`ComponentSender`. The Gallery configures `0..=100`, small change `1`, and
initial value `10`. UI Automation observes the native control as
`ControlType.Spinner` / `Microsoft.UI.Xaml.Controls.NumberBox`, verifies its
`RangeValuePattern` metadata, sets the value to `42`, and checks the exact
Winio-owned title `Winio WinUI Extensions — number: 42`.

Templates, styles, dependency-property statics, AutoSuggestBox, InfoBadge,
icons, advanced pane/header/footer APIs, back-requested, ItemInvoked, and other
event surfaces remain intentionally deferred. These closure warnings do not
invalidate the canary contract. This work makes no full WinUI Gallery parity
claim.

Close-button event automation is likewise deferred. Its UIA representation and
localized label have not been established as a stable test contract.

## Reproduction and validation

Metadata acquisition and bindgen run only through the separate bindings tasks.
Normal library and Gallery builds consume committed generated Rust and do not
download metadata or invoke bindgen.

```powershell
mise run bindings        # acquire pinned metadata and regenerate committed Rust
mise run bindings-check  # regenerate to the pinned inputs and byte-compare
mise run ci              # formatting, Clippy, and tests
mise run smoke           # build and run Gallery UI Automation smoke
```

The Gallery additionally proves that the supplemental `InfoBar` activates at
runtime, casts through normal COM/WinRT machinery to the existing
`winio-winui3::Microsoft::UI::Xaml::FrameworkElement`, enters the same Winio
`NativeElement`/Canvas visual tree, and appears through UI Automation. The
existing ToggleButton nullable-state/event regression remains in the same smoke
test.

The Gallery smoke additionally selects the native NavigationView `Details`
item through UI Automation and verifies the exact Winio-owned window title
`Winio WinUI Extensions — navigation: details`.
