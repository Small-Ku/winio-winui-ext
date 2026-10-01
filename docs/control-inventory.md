# WinUI control inventory (Winio 0.12 / winio-winui3 0.4.4)

This document separates three questions that should not be conflated:

1. Is a native WinUI type already projected by locked upstream `winio-winui3 0.4.4`?
2. Is it present in this repository's broad supplemental XAML projection from the pinned Windows App SDK metadata?
3. Has `winio-winui-ext` designed and runtime-tested a Winio wrapper for it?

Projection availability alone is not wrapper completeness or proof that every native lifecycle/API can be hosted correctly.

## High-level Winio widgets and upstream projected equivalents

| Winio 0.12 widget area | Native type(s) already projected by `winio-winui3 0.4.4` | Notes |
| --- | --- | --- |
| Button | `Button`, `Primitives::ToggleButton` | ToggleButton is the current upstream-backed extension wrapper. |
| Check box / radio button | `CheckBox`, `RadioButton` | Upstream projected. |
| Combo box / list box | `ComboBox`, `ListBox` | Upstream projected. |
| Edit / text box / label / link label | `TextBox`, `PasswordBox`, `TextBlock`, `HyperlinkButton` | HyperlinkButton remains a possible native link wrapper. |
| Progress / slider / scroll bar | `ProgressBar`, `Slider`, `Primitives::ScrollBar` | Upstream projected. |
| Scroll view / canvas / tab view | `ScrollViewer`, `Canvas`, `TabView` | Upstream projected. |
| View | `Canvas` as the backend host surface | A hosting surface, not a stand-alone view equivalent. |
| Window / media / web view / WGPU | `Xaml::Window`, `MediaPlayerElement`, `WebView2`, `SwapChainPanel` | Upstream projected, but these have broader lifecycle/runtime integration needs. |

The upstream 0.4.4 projection also contains controls such as `ContentDialog` and `PasswordBox`. `ContentDialog` is now wrapped below; `PasswordBox` still needs a wrapper API/lifecycle review before being exposed here.

## Controls absent upstream but available supplementally

The following examples are absent as public control structs from the compatible `winio-winui3 0.4.4` projection but are present in the current broad supplemental `Microsoft.UI.Xaml` projection generated from the pinned Windows App SDK 2.5.1 metadata:

- `TitleBar`
- `InfoBar`
- `NavigationView`
- `NumberBox`
- `TreeView`
- `AutoSuggestBox`
- `ToggleSwitch`
- `TeachingTip`
- `Expander`
- `FlipView`
- `PersonPicture`
- `CommandBar`
- `MenuBar`
- `CalendarDatePicker`, `DatePicker`, `TimePicker`, `ColorPicker`
- `RatingControl`, `SplitButton`, `DropDownButton`

This list demonstrates supplemental projection availability; it is not a wrapper roadmap promise. The broad generator also follows required dependencies outside the root namespace where bindgen proves they are needed. It does not generate the entire Microsoft metadata universe.

## Current extension wrappers

| Wrapper | Projection source | Current proof |
| --- | --- | --- |
| `IntegratedTitleBar` | supplemental `TitleBar` + upstream XAML `Window` | Public `AsWindow + AsContainer` bridge; pane/back events, chrome visibility and Mica helper; native custom title bar coexists with system caption buttons in UIA. |
| `ToggleButton` | upstream `winio-winui3` | Native nullable three-state round-trip plus UIA TogglePattern event path. |
| `ContentDialog` | upstream `winio-winui3` | Popup-backed wrapper inherits the routed page XamlRoot, owns a Winio-compatible content Canvas, exposes title/button/default-button APIs and typed opened/button/closing-result events; UIA opens the native dialog, verifies Winio-owned content, invokes Primary, and observes `Primary` close result. |
| `AutoSuggestBox` | supplemental control + upstream shared bases | Stable suggestion IDs in native `Tag`; text, suggestion, and query events; UIA submits `TreeView` and observes the query in the Winio window title. |
| `DropDownButton` | supplemental | Native Button.Flyout semantics with a Winio-owned flyout Canvas and open/close events; a private audited IButton ABI shim restores SetFlyout because winio-winui3 0.4.4 omits that accessor when FlyoutBase is outside its projection. UIA verifies native ExpandCollapse state and flyout content. |
| `Expander` | supplemental | Native expand/collapse state and events; exposes a content Canvas as `AsContainer` so Winio children are constructed under the Expander without reparenting; UIA verifies nested content visibility. |
| `InfoBar` | supplemental | Native activation/hosting and visible InfoBar message in UIA. |
| `NavigationView` | supplemental + upstream shared bases | Recursive main/footer descriptors, stable IDs, native symbol icons, settings/pane/header/back APIs and duplicate-chrome suppression; UIA verifies hierarchical routing and TitleBar-driven pane compact/reopen. |
| `NumberBox` | supplemental | Value/range/small-change/spin placement; typed `ValueChanged`; UIA RangeValue sets `42`. |
| `RatingControl` | supplemental | Value, max rating, read-only/clear settings, caption and typed value-change events; Gallery UIA verifies native RangeValue 3→4 with max 5. |
| `SplitButton` | supplemental | Native primary `Click` event plus explicit flyout container; Winio children are created under the flyout Canvas, with flyout open/close events and UIA invoke/expand-collapse coverage. |
| `ToggleSwitch` | supplemental | Native `IsOn`/`SetIsOn`, header text, typed `Toggled(bool)` events, and UIA TogglePattern round-trip in the Gallery. |
| `TeachingTip` | supplemental | Targets an existing Winio widget; native popup close button emits typed `CloseButton` reason through `ComponentSender`. |
| `TreeView` | supplemental | Recursive stable node descriptors; native `Expanding` and `ItemInvoked`; UIA expands root (`root`) and invokes leaf (`leaf`). |

The wrapper layer remains intentionally selective even though the supplemental projection is broad. New wrappers should be driven by downstream API/lifecycle needs and should add their required-method contract plus a native runtime smoke path where practical. They should not require adding another per-control bindgen filter.

The Gallery keeps AutoSuggestBox independent from both `NavigationView.AutoSuggestBox` and `TitleBar.Content`. `NativeElement` inserts each wrapper's native element into the Winio Canvas; moving that same element into another native parent would give two hosts competing ownership of one element. The Gallery instead positions the standalone search in the TitleBar visual band, preserving one owner while matching the shell layout.

See [supplemental bindings](supplemental-bindings.md) for the exact-reference partition, metadata provenance, dependency closure, sanitizer boundary, and reproducibility gates.
