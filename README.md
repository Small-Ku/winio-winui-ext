# winio-winui-ext

Native Windows App SDK / WinUI 3 controls for applications built with [Winio](https://github.com/compio-rs/winio).

`winio-winui-ext` adds Windows-only controls that Winio does not currently expose as high-level components. It uses Winio's public window and container handles; it does not fork or replace the Winio backend.

The workspace also contains a Windows Gallery application. Each sample creates real native WinUI controls, and the smoke tests exercise their UI Automation and event behavior.

## What it provides

- Native controls such as `NavigationView`, `AutoSuggestBox`, `TreeView`, `NumberBox`, `InfoBar`, `CalendarView`, `CommandBar`, `RichEditBox`, and `ItemsView`.
- Container and popup controls, including `SplitView`, `TwoPaneView`, `Flyout`, `ContentDialog`, and `TeachingTip`, that can own Winio children.
- `IntegratedTitleBar` for a native Windows App SDK title bar, system caption buttons, and optional Mica backdrop.
- Typed Rust events and stable item IDs for controls that support selection or actions.

Not every WinUI type has a wrapper. See the [control inventory](docs/control-inventory.md) for implemented controls, deferred controls, and the specific runtime checks performed for each.

## Compatibility

This workspace currently uses:

| Dependency | Version |
| --- | --- |
| `winio` | `0.12.0` |
| `winio-winui3` | `0.4.5` |
| `windows` / Windows support crates | `0.62` family |

The supplementary Rust bindings were previously generated against the `winio-winui3 0.4.4` type inventory. That generation baseline is recorded in [`bindings.lock.toml`](crates/winio-winui-bindings/src/bindings.lock.toml); it is not the current Cargo dependency version. The Windows App SDK `2.5.1` pin identifies the metadata used to generate those bindings, not a separately established minimum runtime version.

Keep the upstream WinRT type dependencies compatible when updating these packages. Do not regenerate bindings or change the projection baseline as part of an unrelated dependency update.

## Use a control

`winio-winui-ext` is not currently published to crates.io (`publish = false`). Add it as a Git or local path dependency while developing.

Inside a Winio `Component::init`, construct a control under its final Winio parent container:

```rust
use winio::prelude::*;
use winio_winui_ext::{NumberBox, NumberBoxSpinPlacement};

init! {
    number: NumberBox = (parent) => {
        size: Size::new(260.0, 32.0)
    }
}
number.set_minimum(0.0)?;
number.set_maximum(100.0)?;
number.set_value(10.0)?;
number.set_spin_button_placement(NumberBoxSpinPlacement::Inline)?;
```

`parent` is the borrowed container passed to your component. The complete example, including `ValueChanged` event handling, is in [the NumberBox Gallery sample](apps/gallery/src/samples/number_box.rs).

## Build and test

On Windows, install the tools declared in [`mise.toml`](mise.toml), then run:

```powershell
mise run doctor
mise run ci
mise run build
mise run smoke
```

- `ci` checks formatting, runs Clippy, and runs the workspace tests.
- `build` builds the Gallery and workspace targets.
- `smoke` launches the Gallery and checks real WinUI controls through Windows UI Automation, including native interactions.

For metadata generation, use the separate tasks. Normal builds use the checked-in Rust bindings and do not download WinMD files.

```powershell
mise run bindings
mise run bindings-check
mise run bindings-test
```

See [supplemental bindings](docs/supplemental-bindings.md) for the generator, input packages, recorded hashes, and verification steps.

## How the Gallery works

The Gallery demonstrates controls on routed pages. A page can contain several `GalleryExample` cards with an example, live options, output, and selectable Rust source. Pages use one Winio scroll owner, while each card creates its controls under the card's native container.

The title bar, navigation, and search are separate Winio-owned controls. Samples use stable route IDs; replacing a page drops its previous controls. [Gallery architecture](docs/gallery-architecture.md) explains the ownership rules and layout.

The Gallery is an integration and regression-test application. It is not intended to reproduce every feature of Microsoft's WinUI Gallery.

## Source and license

This repository is licensed under the [MIT License](LICENSE).

`crates/winio-winui-bindings/src/generated.rs` contains generated Rust API bindings. [`bindings.lock.toml`](crates/winio-winui-bindings/src/bindings.lock.toml) records the Windows App SDK and WebView2 metadata inputs used to produce it. The Microsoft SDKs and runtimes remain separate dependencies; the repository does not include their NuGet packages or runtime binaries.
