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

The upstream 0.4.4 projection also provides `ContentDialog` and `PasswordBox` directly; both now have extension wrappers below.

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
- `ProgressRing`, `RatingControl`, `SplitButton`, `DropDownButton`
- `BreadcrumbBar`, `InfoBadge`, `RadioButtons`, `SelectorBar`

This list demonstrates supplemental projection availability; it is not a wrapper roadmap promise. The broad generator also follows required dependencies outside the root namespace where bindgen proves they are needed. It does not generate the entire Microsoft metadata universe.

## Current extension wrappers

| Wrapper | Projection source | Current proof |
| --- | --- | --- |
| `IntegratedTitleBar` | supplemental `TitleBar` + upstream XAML `Window` | Public `AsWindow + AsContainer` bridge; pane/back events, chrome visibility and Mica helper; native custom title bar coexists with system caption buttons in UIA. |
| `ToggleButton` | upstream `winio-winui3` | Native nullable three-state round-trip plus UIA TogglePattern event path. |
| `PasswordBox` | upstream `winio-winui3` | Header, placeholder, password character, max length and reveal-button state plus typed password-change notification. Gallery UIA verifies `IsPassword`, sets a value through ValuePattern, and reports only the resulting length rather than the secret. |
| `ContentDialog` | upstream `winio-winui3` | Popup-backed wrapper inherits the routed page XamlRoot, owns a Winio-compatible content Canvas, exposes title/button/default-button APIs and typed opened/button/closing-result events; UIA opens the native dialog, verifies Winio-owned content, invokes Primary, and observes `Primary` close result. |
| `AutoSuggestBox` | supplemental control + upstream shared bases | Stable suggestion IDs in native `Tag`; text, suggestion, and query events; UIA submits `TreeView` and observes the query in the Winio window title. |
| `CalendarDatePicker` | supplemental | Reuses `time::Date` and the DatePicker WinRT conversion for nullable date selection, range/calendar/format/flyout state and typed optional old/new date events. Gallery UIA opens the native `CalendarView`, selects the next day without locale-specific labels, and observes `2026-10-01 → 2026-10-02`. |
| `DatePicker` | supplemental | Reuses `time::Date` for calendar-day values, with header/calendar/range/field-visibility APIs and typed old/new date events. Gallery UIA opens the native picker, advances month and day without locale-specific labels, accepts, and observes `2026-10-01 → 2026-11-02`. |
| `DropDownButton` | supplemental | Native Button.Flyout semantics with a Winio-owned flyout Canvas and open/close events; a private audited IButton ABI shim restores SetFlyout because winio-winui3 0.4.4 omits that accessor when FlyoutBase is outside its projection. UIA verifies native ExpandCollapse state and flyout content. |
| `ColorPicker` | supplemental | Reuses Winio `Color` for RGBA get/set, alpha and compact-surface visibility controls, and typed old/new color events. Gallery UIA edits the native `HexTextBox` from `#3366CC` to `#112233` and observes `#FF112233`. |
| `CommandBar` | supplemental | Stable descriptor-backed primary and secondary `AppBarButton` actions; native CommandBar owns the command vectors while wrapper click subscriptions emit stable action IDs. UIA invokes a visible primary action, opens `MoreButton`, invokes a secondary overflow action, and observes both IDs. |
| `MenuBar` | supplemental | Descriptor-backed top-level `MenuBarItem` menus with stable `MenuFlyoutItem` action IDs and typed click events. UIA expands File and Help, invokes actions in both flyouts, and observes their IDs. |
| `Expander` | supplemental | Native expand/collapse state and events; exposes a content Canvas as `AsContainer` so Winio children are constructed under the Expander without reparenting; UIA verifies nested content visibility. |
| `FlipView` | supplemental | Descriptor-backed native `FlipViewItem` pages, selected index/ID, touch-animation setting and typed selection changes. Gallery UIA uses `ItemContainerPattern` to realize/select virtualized Second page and verifies the native selection plus stable output. |
| `InfoBadge` | supplemental | Exposes native numeric value; the Gallery reads it back after setting a visible count. |
| `RadioButtons` | supplemental | Owns native `RadioButton` options with stable IDs, selected index/ID and typed selection changes. |
| `SelectorBar` | supplemental | Owns `SelectorBarItem` entries with stable IDs, selected index/ID and typed selection changes. |

| `InfoBar` | supplemental | Native activation/hosting and visible InfoBar message in UIA. |
| `NavigationView` | supplemental + upstream shared bases | Recursive main/footer descriptors, stable IDs, native symbol icons, settings/pane/header/back APIs and duplicate-chrome suppression; UIA verifies hierarchical routing and TitleBar-driven pane compact/reopen. |
| `NumberBox` | supplemental | Value/range/small-change/spin placement; typed `ValueChanged`; UIA RangeValue sets `42`. |
| `PersonPicture` | supplemental | Display name, initials, badge number/text/glyph, group state and small-image preference. Gallery composes its visible output from native property getters after setting `Ada Lovelace`, `AL` and badge `3`. |
| `PipsPager` | supplemental | Page count/current index, max visible pips, previous/next button visibility and wrap mode with typed selected-index changes. Gallery UIA invokes native `NextPageButton` and `PreviousPageButton` and observes `page:1 → page:2 → page:1`. |
| `ProgressRing` | supplemental | Active/indeterminate state and determinate minimum/maximum/value. Gallery UIA verifies native RangeValue `65`, toggles the live Indeterminate option on and back off, and observes the sample output change. |
| `RatingControl` | supplemental | Value, max rating, read-only/clear settings, caption and typed value-change events; Gallery UIA verifies native RangeValue 3→4 with max 5. |
| `SplitButton` | supplemental | Native primary `Click` event plus explicit flyout container; Winio children are created under the flyout Canvas, with flyout open/close events and UIA invoke/expand-collapse coverage. |
| `TimePicker` | supplemental | Uses `std::time::Duration` for time-of-day values, exposes header/clock/minute-increment APIs and typed old/new time events. Gallery UIA opens the native picker, selects hour 11 and minute 45, accepts, and observes `10:30 → 11:45`. |
| `ToggleSwitch` | supplemental | Native `IsOn`/`SetIsOn`, header text, typed `Toggled(bool)` events, and UIA TogglePattern round-trip in the Gallery. |
| `TeachingTip` | supplemental | Targets an existing Winio widget; native popup close button emits typed `CloseButton` reason through `ComponentSender`. |
| `TreeView` | supplemental | Recursive stable node descriptors; native `Expanding` and `ItemInvoked`; UIA expands root (`root`) and invokes leaf (`leaf`). |

`BreadcrumbBar` remains unwrapped in this batch: its `ItemsSource` requires a XAML-bindable collection. The safe generic read-only vector view available in the workspace does not realize native item peers, and supplying the required bindable collection would expand this pass into a custom ABI collection implementation.

The wrapper layer remains intentionally selective even though the supplemental projection is broad. New wrappers should be driven by downstream API/lifecycle needs and should add their required-method contract plus a native runtime smoke path where practical. They should not require adding another per-control bindgen filter.

The Gallery keeps AutoSuggestBox independent from both `NavigationView.AutoSuggestBox` and `TitleBar.Content`. `NativeElement` inserts each wrapper's native element into the Winio Canvas; moving that same element into another native parent would give two hosts competing ownership of one element. The Gallery instead positions the standalone search in the TitleBar visual band, preserving one owner while matching the shell layout.

See [supplemental bindings](supplemental-bindings.md) for the exact-reference partition, metadata provenance, dependency closure, sanitizer boundary, and reproducibility gates.
