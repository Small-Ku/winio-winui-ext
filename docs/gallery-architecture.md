# Gallery architecture

## M2 shell and routing responsibilities

`MainModel` owns the permanent shell: the Window, integrated TitleBar, NavigationView, and independent AutoSuggestBox. The typed catalog in `apps/gallery/src/catalog.rs` defines stable sample IDs, titles, sections, keywords, and descriptions. Navigation hierarchy and search suggestions are both derived from that catalog. Each selected route is constructed by one async factory and stored in the shell's single boxed sample slot.

Each routed sample owns its controls behind the shared boxed component contract. The shell sends a common `SetBounds(Rect)` viewport message. Routed control pages own one `GalleryPage` and one vertical Winio `ScrollView`, while Home owns a simpler `ScrollView` landing page. A routed page can contain N `GalleryExample` cards. Native sample events pass through the sample's `ComponentSender` and become common output events handled by the shell.

## Why the boxed slot works

Winio's `BoxComponent` erases the concrete `Component` type while preserving the shared Message, Event, and Error types. It does not erase `AsWidget` or `Layoutable`, so the shell never lays out the boxed component directly. Each concrete sample keeps its own container/layout boundary: control pages use `GalleryPage` as the single ScrollView owner and `GalleryExample` as card-local roots; Home uses a `ScrollView` directly. Card roots are created under page scroll content, and all card chrome and sample controls are constructed beneath their owning card from the start. No Winio-owned native element is transferred to another native content host.

Replacing the boxed child during a shell update drops the old sample and its View/native controls. Winio's run loop starts a fresh `model.start()` future after each message or event; the next iteration therefore polls the newly installed child's `start()` path. The UIA smoke verifies both ToggleButton example controls disappear from the main window subtree after routing to NumberBox.

Every routed sample instance receives a monotonically increasing generation. Output events carry both that generation and the stable sample ID, and the shell accepts an event only when both match the current routed instance. This prevents delayed callbacks from a dropped sample from mutating the current page, including the case where the user later returns to the same sample kind. Re-selecting the already active sample is a no-op instead of rebuilding it.

## Shell chrome

The shell mirrors the current WinUI Gallery hierarchy without transferring native ownership between Winio children. The integrated Windows App SDK `TitleBar` provides the single visible pane-toggle button and Mica-backed window chrome. The NavigationView disables its own back and pane-toggle buttons so the window does not expose a second navigation chrome. The standalone AutoSuggestBox remains a normal Winio child, but the shell lays it into the TitleBar visual band with a 580-DIP maximum width and hides it when a narrow window cannot reserve enough space around the app title and system caption buttons.

The NavigationView is 300 DIPs wide while expanded and 48 DIPs while compact. The shared page title, description, and routed sample bounds use the same pane-width state, so collapsing the pane reclaims the horizontal space instead of leaving a 300-DIP blank gutter. Top-level catalog entries carry native `SymbolIcon` values; leaf route identity remains the stable catalog ID. UI Automation verifies that the standalone search stays inside the TitleBar band, the NavigationView does not expose its legacy toggle button, the TitleBar toggle compacts and reopens the pane, and page content shifts with that state.

## Search ownership

The Gallery AutoSuggestBox remains a separate shell child instead of being assigned to `NavigationView.AutoSuggestBox` or `TitleBar.Content`. A Winio child already owns its native element through the Winio Canvas; transferring that same element into another native parent would make ownership and reparenting ambiguous. The shell positions the standalone control in the TitleBar visual band while keeping one clear Winio owner. It uses the same catalog as NavigationView and routes only stable sample IDs or exact ID/title matches.

## Gallery page and ControlExample card presentation

`apps/gallery/src/chrome.rs` provides the app-local page and card primitives. `GalleryPage` owns exactly one vertical Winio `ScrollView`, disables horizontal scrolling, exposes its content container, and maintains a measured extent sentinel at the current page content height. `GalleryExample` is a card-local Canvas root under that page. Its theme-aware Borders, labels, source disclosure and source TextBox live inside the root; the sample creates controls with the card as their `AsContainer` from construction time. Cards do not own page scrolling or sample-specific option state.

`CardLayout` is deterministic and caps readable width at 1064 DIP. At wide widths the primary example remains on the left while Output and Options occupy an auxiliary column. Below the compact breakpoint those auxiliary regions stack vertically. Each Source code toggle starts collapsed and reveals compile-time Rust source from `include_str!` in a 360-DIP read-only WinUI TextBox. The source viewer uses monospaced no-wrap text, remains selectable/copyable, and owns its horizontal/vertical text scrolling while `GalleryPage` remains the single page-level ScrollView. `stack_cards` places cards with a 16-DIP gap and computes page extent from the current card heights, so expanding a card's bounded source viewer moves later cards by a predictable amount. Resize and compact NavigationView changes recompute card widths and stack positions.

ToggleButton proves the multi-example route with two independent native examples: a nullable three-state control with the live Three-state option, and a separate basic two-state control. Each card has its own output, source-expanded state, and distinct accessible source toggle name. The UIA smoke checks one routed ScrollViewer, independent interactions, card displacement and scroll extent, whole-page scrolling with fixed shell chrome, route cleanup, and narrow card layout. NumberBox proves a live small-change option; TreeView retains width-derived sizing for Root and Leaf text; the smoke also keeps the Search/NumberBox geometry and TeachingTip popup lifecycle checks.

## Further work

The main gaps are control breadth and sample depth: more useful projected controls still need wrapper APIs and Gallery routes, and existing controls can gain additional examples/options where they exercise distinct behavior. Source syntax highlighting and a dedicated copy button are optional polish beyond the native TextBox selection/copy behavior. Keyboard/focus, high-contrast/theme, DPI, and packaging/release coverage can be filled in as the project approaches a first release.
