# Gallery architecture

## M2 shell and routing responsibilities

`MainModel` owns the permanent shell: the Window, integrated TitleBar, NavigationView, and independent AutoSuggestBox. The typed catalog in `apps/gallery/src/catalog.rs` defines stable sample IDs, titles, sections, keywords, and descriptions. Navigation hierarchy and search suggestions are both derived from that catalog. Each selected route is constructed by one async factory and stored in the shell's single boxed sample slot.

Each routed sample owns its sample-specific controls behind the shared boxed component contract. The shell sends a common `SetBounds(Rect)` viewport message; M3A control pages pass that viewport to a reusable `GalleryExample`, while Home owns a simpler `ScrollView` landing page. Native sample events still pass through the sample's `ComponentSender` and become common output events handled by the shell.

## Why the boxed slot works

Winio's `BoxComponent` erases the concrete `Component` type while preserving the shared Message, Event, and Error types. It does not erase `AsWidget` or `Layoutable`, so the shell never lays out the boxed component directly. Each concrete sample keeps its own container/layout boundary: M3A control pages use `GalleryExample` (which owns a Winio `ScrollView`) and Home uses a `ScrollView` directly.

Replacing the boxed child during a shell update drops the old sample and its View/native controls. Winio's run loop starts a fresh `model.start()` future after each message or event; the next iteration therefore polls the newly installed child's `start()` path. The UIA smoke verifies that the ToggleButton peer disappears from the main window subtree after routing to NumberBox.

Every routed sample instance receives a monotonically increasing generation. Output events carry both that generation and the stable sample ID, and the shell accepts an event only when both match the current routed instance. This prevents delayed callbacks from a dropped sample from mutating the current page, including the case where the user later returns to the same sample kind. Re-selecting the already active sample is a no-op instead of rebuilding it.


## Shell chrome

The shell mirrors the current WinUI Gallery hierarchy without transferring native ownership between Winio children. The integrated Windows App SDK `TitleBar` provides the single visible pane-toggle button and Mica-backed window chrome. The NavigationView disables its own back and pane-toggle buttons so the window does not expose a second navigation chrome. The standalone AutoSuggestBox remains a normal Winio child, but the shell lays it into the TitleBar visual band with a 580-DIP maximum width and hides it when a narrow window cannot reserve enough space around the app title and system caption buttons.

The NavigationView is 300 DIPs wide while expanded and 48 DIPs while compact. The shared page title, description, and routed sample bounds use the same pane-width state, so collapsing the pane reclaims the horizontal space instead of leaving a 300-DIP blank gutter. Top-level catalog entries carry native `SymbolIcon` values; leaf route identity remains the stable catalog ID. UI Automation verifies that the standalone search stays inside the TitleBar band, the NavigationView does not expose its legacy toggle button, the TitleBar toggle compacts and reopens the pane, and page content shifts with that state.

## Search ownership

The Gallery AutoSuggestBox remains a separate shell child instead of being assigned to `NavigationView.AutoSuggestBox` or `TitleBar.Content`. A Winio child already owns its native element through the Winio Canvas; transferring that same element into another native parent would make ownership and reparenting ambiguous. The shell therefore positions the standalone control in the TitleBar visual band while keeping one clear Winio owner. It uses the same catalog as NavigationView and routes only stable sample IDs or exact ID/title matches.

## M3A ControlExample presentation

`apps/gallery/src/chrome.rs` provides the app-local presentation layer shared by routed control pages. `GalleryExample` owns a vertical Winio `ScrollView` and exposes that ScrollView's Canvas as the stable `AsContainer` boundary for the actual sample controls. Decorative native Borders and the monospaced source TextBlock are created as sibling children of the same Canvas; already-owned Winio/native controls are never reparented into another native content host.

`CardLayout` is a deterministic layout calculation with a readable 1064-DIP maximum width. At wide widths the primary example remains on the left while Output and Options occupy an auxiliary column. Below the compact breakpoint those auxiliary regions stack vertically. The Source code toggle remains collapsed initially; expanding it reveals compile-time Rust source from `include_str!`, increases the ScrollView content extent, and leaves the shell title/navigation outside the scrollable viewport.

Visible Output is driven from the same sample update path used for deterministic window-title smoke evidence, so there is no second state model. ToggleButton proves a live three-state option, and NumberBox proves a live small-change option. TreeView keeps width-derived layout so its node text does not regress to the earlier fixed-width truncation.

## Remaining Gallery work

The next milestone is breadth rather than another shell rewrite: expand the typed catalog and wrapper/sample inventory for useful projected controls, add multiple examples per control where the API warrants them, and improve source presentation (for example wrapper/sample tabs or syntax-aware rendering) only when it adds clear value. Packaging/release work and exact parity with the official WinUI Gallery remain out of scope unless separately scheduled.
