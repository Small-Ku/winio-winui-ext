# Gallery architecture

The Gallery demonstrates WinUI controls hosted by Winio. Its shell owns navigation and window chrome; each route owns the controls it displays. Native controls keep the same parent for their lifetime.

## Shell and routes

`MainModel` in `apps/gallery/src/main.rs` owns the window, `IntegratedTitleBar`, `NavigationView`, and an independent `AutoSuggestBox`. The typed catalog in `apps/gallery/src/catalog.rs` defines sample IDs, titles, sections, search terms, and descriptions. Both navigation and search read the same catalog.

When a route changes, the shell:

1. Builds the new sample through an async factory.
2. Replaces its single boxed sample component.
3. Drops the old sample and its native controls.
4. Accepts output events only when both the route ID and sample generation match the active sample.

A monotonically increasing generation prevents late callbacks from an old route from updating a new instance of the same sample. Selecting the already active route does not rebuild it.

### Why `BoxComponent` is used

`BoxComponent` allows different samples to share the same `Message`, `Event`, and `Error` types. It does not supply `AsWidget` or `Layoutable` for layout, so each sample arranges its own controls.

The shell sends `SetBounds(Rect)` to the active sample. Home owns a Winio `ScrollView` directly. Control routes use a `GalleryPage` with one vertical `ScrollView`; each page can contain several `GalleryExample` cards.

The Winio run loop starts the current component's `model.start()` future after each message or event. Once a route replaces the boxed component, the next iteration polls the new sample. The UIA smoke checks that the previous ToggleButton controls leave the window's automation tree after navigating to NumberBox.

## Window chrome and navigation

`IntegratedTitleBar` provides the native title bar, pane-toggle button, and Mica-backed window chrome. The system caption buttons remain available. `NavigationView` hides its own back and pane-toggle buttons to avoid duplicate controls.

The navigation pane is 300 DIPs wide when expanded and 48 DIPs when compact. The shell uses its current width to place the page title, description, search, and routed sample. Collapsing the pane gives that space back to the content.

The shell positions the search box in the title-bar band, with a maximum width of 580 DIPs. When the window becomes too narrow for the title and system caption buttons, it hides the search box.

### Why search stays a separate Winio child

Winio already attaches the native `AutoSuggestBox` to its own Canvas. Moving the same element into `NavigationView.AutoSuggestBox` or `TitleBar.Content` would assign it a different native parent while Winio still owns the original parent relationship.

The shell instead positions the existing child in the title-bar band. Search routes use stable catalog IDs or exact ID/title matches; the search box does not need to change native parent.

UI Automation checks title-bar search placement, the single pane-toggle control, pane collapse/reopen, and the resulting content movement.

## Pages and example cards

`apps/gallery/src/chrome.rs` implements the shared page and card layout.

- `GalleryPage` owns one vertical `ScrollView`, disables horizontal page scrolling, and measures the current content height.
- `GalleryExample` owns a card-local Canvas containing borders, labels, the example area, output, options, and source disclosure.
- Each sample constructs its native controls under the card's `AsContainer`. Controls with their own content Canvas, such as Expander, can likewise own Winio children from creation.
- Cards do not own page scrolling or the sample-specific state of their options.

`CardLayout` caps readable width at 1064 DIPs. Wide cards place the example on the left and Output/Options beside it; narrow cards stack those regions. `stack_cards` inserts a 16-DIP gap and updates card positions whenever content height or available width changes.

Each card initially hides its Rust source. The Source toggle reveals text embedded at compile time with `include_str!` inside a read-only WinUI TextBox up to 360 DIPs high. The viewer supports selection, copy, and independent horizontal/vertical text scrolling. Expanding it pushes following cards downward without changing the page's scroll owner.

## Runtime checks

`mise run smoke` launches the Gallery and runs Windows UI Automation checks. Examples include:

- ToggleButton: two independent cards, three-state versus two-state selection, separate output, source expansion, and route cleanup.
- NumberBox: native range input and a live small-change option.
- TreeView: native expansion and leaf invocation while card width changes.
- TeachingTip: native popup open/close behavior.
- Search and window chrome: correct title-bar geometry and navigation reflow.

The [control inventory](control-inventory.md) records additional controls and the interactions currently tested.

## Remaining coverage

The Gallery does not yet cover every WinUI feature or runtime environment. Keyboard and focus behavior, high-contrast and theme changes, DPI changes, and packaging/release paths need further testing. Source syntax highlighting and a separate copy button are optional improvements; source selection and copying already work through the native TextBox.
