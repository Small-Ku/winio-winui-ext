# WinUI control inventory

The workspace uses Winio `0.12.0` and `winio-winui3 0.4.5`. The bindings generator recorded the upstream `0.4.4` type inventory. The tables below distinguish three questions:

1. Does the generator's recorded `winio-winui3 0.4.4` type inventory already contain the native WinUI type?
2. Does this repository's generated XAML binding file contain the type?
3. Does `winio-winui-ext` implement and test a Winio wrapper for it?

A generated type does not guarantee that a Winio wrapper exists or that its native interactions work.

## High-level Winio widgets and upstream projected equivalents

| Winio 0.12 widget area | Native type(s) already projected by `winio-winui3 0.4.4` | Notes |
| --- | --- | --- |
| Button | `Button`, `Primitives::ToggleButton` | ToggleButton is the current upstream-backed extension wrapper. |
| Check box / radio button | `CheckBox`, `RadioButton` | Upstream projected. |
| Combo box / list box | `ComboBox`, `ListBox` | Upstream projected. |
| Edit / text box / label / link label | `TextBox`, `PasswordBox`, `TextBlock`, `HyperlinkButton` | Winio's `LinkLabel` uses the native `HyperlinkButton`; the existing high-level link widget already covers this behavior. |
| Progress / slider / scroll bar | `ProgressBar`, `Slider`, `Primitives::ScrollBar` | Upstream projected. |
| Scroll view / canvas / tab view | `ScrollViewer`, `Canvas`, `TabView` | Winio's high-level ScrollView is backed by `ScrollViewer`; see the separate native ScrollView parity note below. |
| View | `Canvas` as the backend host surface | A hosting surface, not a stand-alone view equivalent. |
| Window / media / web view / WGPU | `Xaml::Window`, `MediaPlayerElement`, `WebView2`, `SwapChainPanel` | Upstream projected, but these have broader lifecycle/runtime integration needs. |

The upstream 0.4.4 projection also provides `ContentDialog` and `PasswordBox` directly; both now have extension wrappers below.

## Additional native types in generated bindings

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
- `CommandBarFlyout`
- `MenuBar`
- `RichEditBox`
- `MenuFlyout`, `Flyout`
- `CalendarDatePicker`, `CalendarView`, `DatePicker`, `TimePicker`, `ColorPicker`
- `ProgressRing`, `RatingControl`, `RepeatButton`, `SplitButton`, `DropDownButton`
- `BreadcrumbBar`, `InfoBadge`, `RadioButtons`, `SelectorBar`
- `TwoPaneView`
- `SplitView`, `ToggleSplitButton`, `Viewbox`, `Image`
- `SwipeControl`, `SwipeItem`, `SwipeItems`

This list records generated types, not promises to implement wrappers for each. The broad generator also follows required dependencies outside the root namespace where bindgen proves they are needed. It does not generate the entire Microsoft metadata universe.

## Implemented wrappers and tests

| Wrapper | Type source | Implementation and tests |
| --- | --- | --- |
| `IntegratedTitleBar` | supplemental `TitleBar` + upstream XAML `Window` | Public `AsWindow + AsContainer` bridge; pane/back events, chrome visibility and Mica helper; native custom title bar coexists with system caption buttons in UIA. |
| `ToggleButton` | upstream `winio-winui3` | Native nullable three-state round-trip plus UIA TogglePattern event path. |
| `PasswordBox` | upstream `winio-winui3` | Header, placeholder, password character, max length and reveal-button state plus typed password-change notification. Gallery UIA verifies `IsPassword`, sets a value through ValuePattern, and reports only the resulting length rather than the secret. |
| `RichEditBox` | supplemental control + upstream `Microsoft.UI.Text` document | Plain-text and RTF convenience APIs sit on top of the native `RichEditTextDocument`, while `document()` exposes ranges, selection, formatting, undo/redo, hyperlinks/images, and stream APIs without inventing a second document model. `TextChanged` is payload-free so the wrapper does not copy the entire document on every edit. Gallery initializes formatted RTF, exercises `Selection()`, focuses the native editor, injects real keyboard input with Win32 `SendInput`, observes the typed change event, and reads `Keyboard edit` back through UIA TextPattern. |
| `ContentDialog` | upstream `winio-winui3` | Popup-backed wrapper inherits the routed page XamlRoot, owns a Winio-compatible content Canvas, exposes title/button/default-button APIs and typed opened/button/closing-result events; UIA opens the native dialog, verifies Winio-owned content, invokes Primary, and observes `Primary` close result. |
| `Flyout` | supplemental | Popup-backed component owns a Winio-compatible content Canvas, anchors with `show_at(BorrowedWidget)`, emits typed opened/closed events, and hides deterministically on drop. Gallery UIA opens it, interacts with a Winio-owned `ToggleButton` inside the native popup, observes the child event/state, then verifies route replacement removes the popup. |
| `AutoSuggestBox` | supplemental control + upstream shared bases | Stable suggestion IDs in native `Tag`; text, suggestion, and query events; UIA submits `TreeView` and observes the query in the Winio window title. |
| `BreadcrumbBar` | supplemental | Descriptor-backed path items are boxed into a reusable non-agile `Microsoft.UI.Xaml.Interop.IBindableIterable` adapter; Gallery UIA verifies three native `BreadcrumbBarItem` peers are realized and invokes Projects to observe stable `path:projects` output. |
| `CalendarDatePicker` | supplemental | Reuses `time::Date` and the DatePicker WinRT conversion for nullable date selection, range/calendar/format/flyout state and typed optional old/new date events. Gallery UIA opens the native `CalendarView`, selects the next day without locale-specific labels, and observes `2026-10-01 → 2026-10-02`. |
| `CalendarView` | supplemental | Inline native calendar with a deliberate single-date contract over `time::Date`; exposes selected/display/min/max dates, calendar/day format, bounded month-view presentation properties, and typed optional old/new selection events. Gallery gives the dense native template a 360px primary height and a 480px minimum side-by-side width, stacking Output below when that width is unavailable; UIA then proves the initially selected day and next day are both visibly realized before selecting `2026-10-01 → 2026-10-02`. |
| `DatePicker` | supplemental | Reuses `time::Date` for calendar-day values, with header/calendar/range/field-visibility APIs and typed old/new date events. Gallery UIA opens the native picker, advances month and day without locale-specific labels, accepts, and observes `2026-10-01 → 2026-11-02`. |
| `DropDownButton` | supplemental | Native Button.Flyout semantics with a Winio-owned flyout Canvas and open/close events; a private audited IButton ABI shim restores SetFlyout because winio-winui3 0.4.4 omits that accessor when FlyoutBase is outside its projection. UIA verifies native ExpandCollapse state and flyout content. |
| `ColorPicker` | supplemental | Reuses Winio `Color` for RGBA get/set, alpha and compact-surface visibility controls, and typed old/new color events. Gallery UIA edits the native `HexTextBox` from `#3366CC` to `#112233` and observes `#FF112233`. |
| `CommandBar` | supplemental | Stable descriptor-backed primary and secondary `AppBarButton` actions; native CommandBar owns the command vectors while wrapper click subscriptions emit stable action IDs. UIA invokes a visible primary action, opens `MoreButton`, invokes a secondary overflow action, and observes both IDs. |
| `CommandBarFlyout` | supplemental | Reuses `CommandBarAction` for stable primary/secondary command IDs, owns native `AppBarButton` command vectors and click tokens, exposes `show_at`/`hide`, `AlwaysExpanded`, and typed opened/closed/action events. Gallery UIA verifies flyout-specific `MenuItem` automation semantics, invokes Copy, opens `MoreButton`, invokes secondary Delete in the same popup lifetime, and observes both IDs. |
| `MenuBar` | supplemental | Descriptor-backed top-level `MenuBarItem` menus with stable `MenuFlyoutItem` action IDs and typed click events. UIA expands File and Help, invokes actions in both flyouts, and observes their IDs. |
| `MenuFlyout` | supplemental | Reuses `MenuBarAction` for stable IDs, owns native `MenuFlyoutItem` entries and click tokens, and exposes popup lifecycle plus typed action events. Gallery UIA opens the native popup, invokes Rename through its `MenuItem` peer, and observes `action:rename`. |
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
| `SplitView` | supplemental | Owns separate Pane and Content Canvas hosts; exposes open/display/placement/length state and typed opening/opened/closing/closed events. Gallery UIA closes and reopens the native pane and observes the ordered lifecycle events. |
| `Viewbox` | supplemental | Owns a Canvas without imposing a hidden extent; callers set content size when they need the Canvas to have a measurable native extent. Exposes stretch and stretch-direction configuration. Gallery sets 360×180 content and UIA observes the scaled child in a 240×120 host. |
| `Image` | supplemental | Native `ImageSource` is the advanced source escape hatch, with `BitmapImage`/URI convenience APIs, Stretch/NineGrid, and typed opened/failed lifecycle events. Gallery decodes an embedded 4×2 PNG from an `InMemoryRandomAccessStream`, round-trips Source back to `BitmapImage`, observes native `ImageOpened` plus 4×2 decoder dimensions, and verifies the native Image peer is visibly realized without network or file-resource dependencies. |
| `TabView` | upstream `winio-winui3` | Owns its `TabViewItem` instances, stores stable IDs separately from visible titles, preserves selection by ID when replacing descriptors, validates selected indexes, and emits typed selection changes. Gallery UIA selects a tab by its descriptor-derived AutomationId. |
| `RatingControl` | supplemental | Value, max rating, read-only/clear settings, caption and typed value-change events; Gallery UIA verifies native RangeValue 3→4 with max 5. |
| `RepeatButton` | supplemental + upstream required bases | Text content plus native `Delay`/`Interval` timing and typed Click events; the wrapper casts the supplemental control to upstream ContentControl/ButtonBase rather than adding an ABI shim. Gallery smoke performs a real mouse-down, holds for 700 ms with 150 ms delay / 80 ms interval, requires multiple Click events, releases the mouse, and proves the count remains stable after queued events drain. |
| `SplitButton` | supplemental | Native primary `Click` event plus explicit flyout container; Winio children are created under the flyout Canvas, with flyout open/close events and UIA invoke/expand-collapse coverage. |
| `ToggleSplitButton` | supplemental | Reports native primary-click and checked-state events, and owns a flyout Canvas. UIA toggles checked state, expands the flyout, and confirms opening it preserves the checked state. |
| `TimePicker` | supplemental | Uses `std::time::Duration` for time-of-day values, exposes header/clock/minute-increment APIs and typed old/new time events. Gallery UIA opens the native picker, selects hour 11 and minute 45, accepts, and observes `10:30 → 11:45`. |
| `ToggleSwitch` | supplemental | Native `IsOn`/`SetIsOn`, header text, typed `Toggled(bool)` events, and UIA TogglePattern round-trip in the Gallery. |
| `TeachingTip` | supplemental | Targets an existing Winio widget; native popup close button emits typed `CloseButton` reason through `ComponentSender`. |
| `TreeView` | supplemental | Recursive stable node descriptors; native `Expanding` and `ItemInvoked`; UIA expands root (`root`) and invokes leaf (`leaf`). |
| `GridView` | supplemental | Stable-ID tile descriptors are materialized as wrapper-owned native `GridViewItem` UIElements and supplied through the reusable non-agile `IBindableIterable`; replacement retention policy is covered by a deterministic helper test, and Gallery UIA verifies realization and `SelectionItemPattern` selection. |
| `ItemsView` | supplemental | Descriptor-backed text values use the native default string presentation and default `StackLayout`; stable IDs are tracked by source index, with single selection and typed selection changes. Gallery UIA enumerates native `ItemContainer` peers in source order, selects the second with `SelectionItemPattern`, and verifies the selected peer and stable-ID output. See Microsoft's [ItemsView guidance](https://learn.microsoft.com/en-us/windows/apps/develop/ui/controls/itemsview). |

### Other controls considered

| Control | Classification | Reason |
| --- | --- | --- |
| `HyperlinkButton` | Already covered by Winio | Winio's `LinkLabel` is implemented with the native WinUI `HyperlinkButton`; an extension wrapper would duplicate the high-level widget. |
| `RepeatButton` | Implemented | Its defining repeat-on-hold behavior now has real pointer evidence: smoke moves the system pointer to the native button, holds mouse-down across the configured delay/interval, observes multiple typed Click events, then verifies repeating stops after mouse-up. |
| `RelativePanel` | Not appropriate for current layout model | Winio positions children itself. A relationship-oriented panel API would need to own child arrangement and express sibling/panel constraints; a thin wrapper with stringly attached properties would conflict with Winio layout and provide an unsafe contract. |
| `CalendarView` | Implemented | The native multiple-selection/blackout surface is intentionally excluded for now: the wrapper fixes `SelectionMode::Single`, owns the selected-date vector as a 0/1 value, reuses the shared date conversion, and has visible inline UIA selection evidence. |
| `ToolTip` | Already covered by Winio | Winio exposes the `ToolTip` trait and the WinUI backend implements it through `ToolTipService` on ordinary widgets; no separate control wrapper is needed for normal tooltip behavior. |
| `Pivot` | Deferred: collection and design guidance | It owns a collection of pivot items and is explicitly discouraged by current Gallery metadata in favor of SelectorBar. |
| `AppBarButton`, `AppBarToggleButton`, `AppBarSeparator` | No separate wrapper needed in current surface | `CommandBar` descriptors already own `AppBarButton` actions. A toggle or separator-specific contract may be useful later, but this sweep has no distinct behavior request beyond those descriptors. |
| `Flyout`, `MenuFlyout`, `CommandBarFlyout` | Implemented | Popup ownership is now explicit: generic `Flyout` owns a Winio content Canvas, while Menu/CommandBar flyouts own stable descriptor-backed native command collections and invocation tokens. Gallery smoke exercises popup lifecycle, Winio-owned interactive content, primary/overflow commands, stable action IDs, and route cleanup. |
| `Image` | Implemented | Source ownership is explicit: `ImageSource` remains the public native escape hatch while `BitmapImage` and URI loading are conveniences. Offline stream-backed runtime evidence covers decoder/open lifecycle, source runtime-class round-trip, dimensions, and visible native hosting. |
| `RichEditBox` | Implemented | The wrapper exposes native `RichEditTextDocument` as an explicit escape hatch; plain text and RTF are conveniences rather than the whole document model. Runtime smoke proves native Unicode keyboard editing and TextPattern readback. |
| `ScrollView` | Base coverage in Winio; native parity deferred | Winio already provides a ScrollView backed by WinUI ScrollViewer. The Gallery's Microsoft.UI.Xaml.Controls.ScrollView adds pan/zoom and ScrollController semantics that are not represented by that base component and need separate input/controller coverage. |
| `PagerControl` | Deferred: projection boundary | Current Gallery metadata describes numbered page navigation, but the shared supplemental projection does not currently export the type; adding a per-control bindgen filter would violate the projection rule for this sweep. |
| `PullToRefresh` | Deferred: input/lifecycle evidence | Its defining behavior is a touch pull gesture that runs a refresh lifecycle against a collection; the current smoke has no touch-gesture driver. |
| `TableView` | Deferred: composite table behavior | ItemsSource is now available, but a useful table wrapper still needs a coherent contract for virtualized row selection, editable cells, sorting, grouping, and columns, plus runtime coverage for those interactions. |
| `WrapPanel` | Deferred: projection boundary | It appears in current Gallery metadata but is absent from the checked-in supplemental Rust projection. No one-off bindgen filter is added for this layout type. |

### Collection controls considered

| Control | Classification | Reason |
| --- | --- | --- |
| `GridView` | Implemented | Winio has no high-level tile selector. Its descriptor-backed wrapper owns `GridViewItem` UIElements, places stable IDs in `Tag` and descriptor-derived `AutomationId`, and exposes single selection plus typed events. A policy helper test verifies that replacement retains only a still-present selected ID; the Gallery smoke proves native item realization and selection through UIA. |
| `ItemsView` | Implemented | The default `ItemTemplate` is null and the default layout is `StackLayout`; the control supports a valid ItemsSource-only string presentation. Its wrapper exposes descriptor-backed source replacement, single selection, validated indexes, selected ID lookup by index, and typed `SelectionChanged`. Gallery smoke verifies default item realization and selection through native `ItemContainer`, `SelectionItemPattern`, and `SelectionPattern` peers. |
| `ItemsRepeater` | Deferred: low-policy virtualization and accessibility contract | ItemsSource-only use is valid and defaults to string presentation with `StackLayout`. ItemsRepeater has no built-in selection or default accessibility experience; a useful wrapper would need to define realized-element lifecycle, scrolling/virtualization ownership, and screen-reader item positioning instead of only exposing `set_items`. That policy contract is outside this sweep. See Microsoft's [ItemsRepeater guidance](https://learn.microsoft.com/en-us/windows/apps/develop/ui/controls/items-repeater). |
| `ListView` | Already covered by Winio | Winio 0.12 `ListBox` already provides the high-level string list, single/multiple selection, mutation, and selection event scenario. This sweep found no bounded native ListView capability gap that justifies duplicating that API. |
| `SemanticZoom` | Deferred: composed semantic-zoom views | The control requires owned zoomed-in and zoomed-out child views implementing `ISemanticZoomInformation`, which GridView alone does not provide as a pair, plus zoom transition and UIA behavior. A bounded composition API and runtime proof remain absent. |

These classifications were checked against locked Winio 0.12 / `winio-winui3 0.4.4`, the current local supplemental projection, and the current official Gallery control metadata. Current metadata reference: [ControlInfoData.json](https://github.com/microsoft/WinUI-Gallery/blob/main/WinUIGallery/SampleSupport/Data/ControlInfoData.json).

### Controls still deferred

`SwipeControl` remains deferred for this batch. Its action items are exposed through four native directional `SwipeItems` collections, and `SwipeItem.Invoked` requires per-item event-token ownership. The current Gallery smoke harness has no pointer-gesture driver to reveal the native actions before invoking them, so it cannot provide the required real-runtime invocation evidence without adding a new synthetic-input path.

`BreadcrumbBar`, `GridView`, and `ItemsView` now use the reusable non-agile `IBindableIterable` adapter in `src/bindable.rs`. Gallery smoke verifies each control's native item realization and meaningful interaction. `GridView` owns UIElement-backed item objects, while `ItemsView` uses plain text data with its documented default item presentation. ItemsSource support does not by itself define the custom lifecycle and accessibility policy needed by `ItemsRepeater` or the composed semantic-zoom views needed by `SemanticZoom`; those boundaries are recorded above.

The wrapper layer remains intentionally selective even though the supplemental projection is broad. New wrappers should be driven by downstream API/lifecycle needs and should add their required-method contract plus a native runtime smoke path where practical. They should not require adding another per-control bindgen filter.

The Gallery keeps AutoSuggestBox independent from both `NavigationView.AutoSuggestBox` and `TitleBar.Content`. `NativeElement` inserts each wrapper's native element into the Winio Canvas; moving that same element into another native parent would give two hosts competing ownership of one element. The Gallery instead positions the standalone search in the TitleBar visual band, preserving one owner while matching the shell layout.

See [supplemental bindings](supplemental-bindings.md) for the exact-reference partition, metadata provenance, dependency closure, sanitizer boundary, and reproducibility gates.
