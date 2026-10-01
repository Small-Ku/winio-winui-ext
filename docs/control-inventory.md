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
- `TwoPaneView`
- `ToggleSplitButton`, `Viewbox`
- `SwipeControl`, `SwipeItem`, `SwipeItems`

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
| `ProgressBar` | upstream `winio-winui3` | Determinate/indeterminate, minimum/maximum/value, error and paused states. Gallery UIA verifies native RangeValue `65` and toggles indeterminate state in the running control. |
| `TwoPaneView` | supplemental | Owns two native Canvas pane containers for Winio children; exposes adaptive mode/configuration, pane priority, and width/height thresholds. Gallery hosts Winio controls in both panes and toggles native paused progress state. |
| `Viewbox` | supplemental | Owns a Canvas for Winio children and exposes stretch/content sizing. Gallery UIA checks the Winio child bounds after native scaling (360×180 content into a 240×120 host). |
| `TabView` | upstream `winio-winui3` | Owns its `TabViewItem` instances, stores stable IDs separately from visible titles, preserves selection by ID when replacing descriptors, validates selected indexes, and emits typed selection changes. Gallery UIA selects a tab by its descriptor-derived AutomationId. |
| `RatingControl` | supplemental | Value, max rating, read-only/clear settings, caption and typed value-change events; Gallery UIA verifies native RangeValue 3→4 with max 5. |
| `SplitButton` | supplemental | Native primary `Click` event plus explicit flyout container; Winio children are created under the flyout Canvas, with flyout open/close events and UIA invoke/expand-collapse coverage. |
| `ToggleSplitButton` | supplemental | Reports native primary-click and checked-state events, and owns a flyout Canvas. UIA toggles checked state, expands the flyout, and confirms opening it preserves the checked state. |
| `TimePicker` | supplemental | Uses `std::time::Duration` for time-of-day values, exposes header/clock/minute-increment APIs and typed old/new time events. Gallery UIA opens the native picker, selects hour 11 and minute 45, accepts, and observes `10:30 → 11:45`. |
| `ToggleSwitch` | supplemental | Native `IsOn`/`SetIsOn`, header text, typed `Toggled(bool)` events, and UIA TogglePattern round-trip in the Gallery. |
| `TeachingTip` | supplemental | Targets an existing Winio widget; native popup close button emits typed `CloseButton` reason through `ComponentSender`. |
| `TreeView` | supplemental | Recursive stable node descriptors; native `Expanding` and `ItemInvoked`; UIA expands root (`root`) and invokes leaf (`leaf`). |

### Candidates from the bounded control sweep

| Control | Classification | Reason |
| --- | --- | --- |
| `HyperlinkButton` | Already covered by Winio | Winio's `LinkLabel` is implemented with the native WinUI `HyperlinkButton`; an extension wrapper would duplicate the high-level widget. |
| `RepeatButton` | Deferred: input evidence | Its distinct behavior is repeated `Click` while a pointer remains pressed. The current UIA smoke can invoke once but has no held-pointer input driver to prove repeat timing. |
| `SplitView` | Already covered by existing shell controls | Its pane/navigation shell role is covered by the Gallery's `NavigationView` and integrated `TitleBar`; a second shell wrapper does not add a distinct downstream control primitive in this sweep. |
| `RelativePanel` | Deferred: layout ownership | Correct use depends on relationships between already-hosted XAML children. The current Winio host inserts widgets into its Canvas, and moving them into a RelativePanel would violate the single-parent ownership rule; a panel-owned child construction API needs a reusable relationship model. |
| `CalendarView` | Deferred: native date selection contract | It is a full inline calendar rather than a popup picker, but its selected-date vector, range, blackout dates, and item realization surface need an explicit bounded contract and runtime selection evidence beyond the existing CalendarDatePicker's popup calendar. |
| `ToolTip` | Deferred: attached-property ownership | Native ToolTip is attached to another element and has hover/press timing; the existing smoke harness cannot reliably prove the attached popup lifecycle without adding input/timing support. |
| `Pivot` | Deferred: collection and design guidance | It owns a collection of pivot items and is explicitly discouraged by current Gallery metadata in favor of SelectorBar. |
| `AppBarButton`, `AppBarToggleButton`, `AppBarSeparator` | Covered by `CommandBar` descriptors | The existing descriptor-backed CommandBar already owns native AppBarButton actions. Separate wrappers would duplicate its action model; toggle/separator-specific extensions need a concrete consumer gap. |
| `MenuFlyout`, `CommandBarFlyout` | Deferred: popup/item ownership | They require stable-ID flyout item ownership and invocation-token lifecycle that is not provided by the existing single-content DropDownButton/SplitButton flyout hosts. |
| `Image` | Deferred: source/resource contract | A wrapper needs an explicit source model and ownership behavior for URI, bitmap, and stream-backed image sources; introducing an ad hoc source/resource type is outside this mechanical pass. |
| `RichEditBox` | Deferred: incomplete document contract | A text-only wrapper would expose editing while hiding document formatting, hyperlinks, and image behavior, producing an incomplete public surface. |
| `ScrollView` | Deferred: input/runtime evidence | The native control's useful distinction is pan/zoom and ScrollController state/events; this needs input and controller coverage beyond the current page ScrollViewer host. |
| `PagerControl` | Deferred: projection boundary | Current Gallery metadata describes numbered page navigation, but the shared supplemental projection does not currently export the type; adding a per-control bindgen filter would violate the projection rule for this sweep. |
| `PullToRefresh` | Deferred: input/lifecycle evidence | Its defining behavior is a touch pull gesture that runs a refresh lifecycle against a collection; the current smoke has no touch-gesture driver. |
| `TableView` | Deferred: collection/virtualization ABI | It combines virtualized rows, stable row selection, editable cells, sorting, grouping, and columns, which requires the bindable collection/item realization contract excluded from this pass. |
| `WrapPanel` | Deferred: projection boundary | It appears in current Gallery metadata but is absent from the checked-in supplemental Rust projection. No one-off bindgen filter is added for this layout type. |

These classifications were checked against locked Winio 0.12 / `winio-winui3 0.4.4`, the current local supplemental projection, and the current official Gallery control metadata. Current metadata reference: [ControlInfoData.json](https://github.com/microsoft/WinUI-Gallery/blob/main/WinUIGallery/SampleSupport/Data/ControlInfoData.json).

### Previously deferred controls

`SwipeControl` remains deferred for this batch. Its action items are exposed through four native directional `SwipeItems` collections, and `SwipeItem.Invoked` requires per-item event-token ownership. The current Gallery smoke harness has no pointer-gesture driver to reveal the native actions before invoking them, so it cannot provide the required real-runtime invocation evidence without adding a new synthetic-input path.

`BreadcrumbBar` remains unwrapped in this batch: its `ItemsSource` requires a XAML-bindable collection. The safe generic read-only vector view available in the workspace does not realize native item peers, and supplying the required bindable collection would expand this pass into a custom ABI collection implementation.

The wrapper layer remains intentionally selective even though the supplemental projection is broad. New wrappers should be driven by downstream API/lifecycle needs and should add their required-method contract plus a native runtime smoke path where practical. They should not require adding another per-control bindgen filter.

The Gallery keeps AutoSuggestBox independent from both `NavigationView.AutoSuggestBox` and `TitleBar.Content`. `NativeElement` inserts each wrapper's native element into the Winio Canvas; moving that same element into another native parent would give two hosts competing ownership of one element. The Gallery instead positions the standalone search in the TitleBar visual band, preserving one owner while matching the shell layout.

See [supplemental bindings](supplemental-bindings.md) for the exact-reference partition, metadata provenance, dependency closure, sanitizer boundary, and reproducibility gates.
