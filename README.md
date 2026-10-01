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

`smoke` builds the Gallery and uses Windows UI Automation to verify the integrated native TitleBar plus AutoSuggestBox, InfoBar, NavigationView, NumberBox, TeachingTip, ToggleButton, and TreeView behavior. Normal builds consume committed generated Rust only and do not acquire WinMD metadata; metadata regeneration and comparison are separate gates:

```powershell
mise run bindings
mise run bindings-check
```

## Current scope

The reference wrappers are an integrated native WinUI `TitleBar`, native upstream `Primitives::ToggleButton`, `ContentDialog`, and `PasswordBox`, plus supplemental AutoSuggestBox, CalendarDatePicker, ColorPicker, CommandBar, DatePicker, DropDownButton, Expander, FlipView, InfoBar, MenuBar, NavigationView, NumberBox, PersonPicture, RatingControl, SplitButton, TimePicker, ToggleSwitch, TeachingTip, and TreeView controls.

ToggleButton preserves its two-state interaction default; the Gallery enables three-state mode and verifies a real `None`/indeterminate round-trip. Because the 0.4.4 generated `IsChecked` getter cannot represent a null `IReference<bool>`, that nullable read stays behind one narrow raw `IToggleButton` ABI call.

`IntegratedTitleBar` takes an ordinary Winio window through the public `AsWindow + AsContainer` bridge, extends the XAML client area into the title-bar region, registers the native Windows App SDK `TitleBar`, exposes its back/pane-toggle requests, and can apply the native Mica backdrop while preserving system caption controls. AutoSuggestBox emits text, chosen-suggestion, and submitted-query events; suggestions carry stable IDs in native `Tag` values. NavigationView supports recursive main/footer menu items, stable selected IDs, pane/settings state, shell-chrome visibility, symbol icons, header text, and back requests. NumberBox exposes value/range/small-change/spin-placement plus typed old/new `ValueChanged` events. PasswordBox wraps the upstream native control with header/placeholder/password-character/length/reveal APIs and a typed change event; the Gallery reports only password length rather than echoing the secret. ColorPicker reuses Winio `Color` for RGBA state, alpha/visibility controls, and typed old/new color events. DatePicker uses the established `time::Date` calendar-day type rather than introducing another date struct or exposing WinRT `DateTime`. CalendarDatePicker reuses the same `time::Date` conversion for nullable calendar selection and flyout state. TimePicker uses `std::time::Duration` for time-of-day values instead of exposing WinRT `TimeSpan`, with minute/clock configuration and typed old/new time events. CommandBar owns stable descriptor-backed primary/overflow AppBarButton actions and emits their IDs. MenuBar likewise owns top-level menus plus stable MenuFlyoutItem actions without reparenting Winio children. PersonPicture exposes native display-name, initials, badge, group, and image-preference state without introducing parallel value types. FlipView owns descriptor-backed native pages, exposes selected index/ID, and reports typed selection changes. ContentDialog inherits the routed page XamlRoot, owns a Winio-compatible content Canvas, and emits typed open/button/closing events. TeachingTip targets an existing Winio widget through the public WinUI handle bridge and reports native close reasons from its popup lifecycle. TreeView accepts stable recursive node descriptors and emits stable IDs from native `Expanding` and `ItemInvoked` events. The Gallery drives these through their native UI Automation patterns and verifies the resulting Winio-owned window titles.

The Gallery now uses a typed sample catalog as the source for stable routes, sectioned NavigationView items/icons, and AutoSuggestBox suggestions. Its shell follows the current WinUI Gallery chrome model: a Mica-backed integrated TitleBar owns the visible pane toggle, search is laid into the title-bar band, NavigationView suppresses its duplicate back/pane buttons, and routed pages share a title/description content header that reflows between expanded and compact pane widths. One dynamically replaced boxed sample slot owns the currently selected page. See [the Gallery architecture](docs/gallery-architecture.md) for the component and layout boundaries. The AutoSuggestBox remains an independent Winio child: it is positioned visually in the title-bar band rather than reparented into `TitleBar.Content` or `NavigationView.AutoSuggestBox`, because moving a native element already owned by the Winio Canvas would make ownership ambiguous.

Supplemental bindings are no longer generated one control at a time. `winio-winui-bindings` builds one pinned broad XAML projection rooted at `Microsoft.UI.Xaml` and `Windows.UI.Xaml.Interop`, automatically references exact types already provided by `winio-winui3 0.4.4`, and follows bindgen dependency diagnostics to generate only missing supplemental dependencies. See [the supplemental bindings notes](docs/supplemental-bindings.md) for the projection boundary, provenance, and validation invariants. See [the control inventory](docs/control-inventory.md) for the distinction between upstream projection availability, supplemental projection availability, and wrapper coverage.

Routed control pages use one vertical ScrollView and can stack multiple reusable ControlExample cards with card-local native ownership, visible output, optional live options, and independently collapsed Rust source sections compiled into the app with include_str!. Wide layouts place output/options beside the primary example; narrow layouts stack them, and expanding source moves later cards while updating the page extent. Source is shown in a bounded read-only WinUI TextBox with monospaced no-wrap text, selection/copy support, and its own horizontal/vertical scrolling rather than stretching the page to the full source length. ToggleButton currently demonstrates two independent examples on one route. The UIA smoke covers page scrolling, multi-card behavior, source selection/read-only state, responsive geometry, live ToggleButton/NumberBox options, the native ToggleSwitch on/off event path, RatingControl RangeValue behavior, ColorPicker hex edit/color-change behavior, CalendarDatePicker CalendarView/date-change behavior, DatePicker native selector/date-change behavior, PasswordBox protected ValuePattern/change behavior, TimePicker native selector/time-change behavior, Expander nested-content ownership, DropDownButton expand/collapse flyout lifecycle, SplitButton primary/flyout lifecycle, CommandBar primary/overflow actions, MenuBar top-level/flyout actions, ContentDialog popup/content/Primary result lifecycle, PersonPicture native property round-trip, FlipView virtualized-item selection, route replacement, and the Search/NumberBox, TreeView, and TeachingTip regressions. This project does not claim parity with the official WinUI Gallery.
