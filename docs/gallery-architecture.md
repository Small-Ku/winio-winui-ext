# Gallery architecture

## M2 responsibilities

`MainModel` owns the permanent shell: the Window, integrated TitleBar, NavigationView, and independent AutoSuggestBox. The typed catalog in `apps/gallery/src/catalog.rs` defines stable sample IDs, titles, sections, keywords, and descriptions. Navigation hierarchy and search suggestions are both derived from that catalog. Each selected route is constructed by one async factory and stored in the shell's single boxed sample slot.

Each routed sample owns a `Child<View>` root and its sample-specific controls. The shell sends a common `SetBounds(Rect)` message to that component; the sample sets its root rectangle and lays its controls out in root-local coordinates. Native sample events pass through the sample's `ComponentSender` and become a common output event handled by the shell.

## Why the boxed slot works

Winio's `BoxComponent` erases the concrete `Component` type while preserving the shared Message, Event, and Error types. It does not erase `AsWidget` or `Layoutable`, so the shell never lays out the boxed component directly. The concrete sample's `View` root provides the common widget/container boundary, and the concrete sample keeps child layout next to its controls.

Replacing the boxed child during a shell update drops the old sample and its View/native controls. Winio's run loop starts a fresh `model.start()` future after each message or event; the next iteration therefore polls the newly installed child's `start()` path. The UIA smoke verifies that the ToggleButton peer disappears from the main window subtree after routing to NumberBox.

Every routed sample instance receives a monotonically increasing generation. Output events carry both that generation and the stable sample ID, and the shell accepts an event only when both match the current routed instance. This prevents delayed callbacks from a dropped sample from mutating the current page, including the case where the user later returns to the same sample kind. Re-selecting the already active sample is a no-op instead of rebuilding it.


## Shell chrome

The shell mirrors the current WinUI Gallery hierarchy without transferring native ownership between Winio children. The integrated Windows App SDK `TitleBar` provides the single visible pane-toggle button and Mica-backed window chrome. The NavigationView disables its own back and pane-toggle buttons so the window does not expose a second navigation chrome. The standalone AutoSuggestBox remains a normal Winio child, but the shell lays it into the TitleBar visual band with a 580-DIP maximum width and hides it when a narrow window cannot reserve enough space around the app title and system caption buttons.

The NavigationView is 300 DIPs wide while expanded and 48 DIPs while compact. The shared page title, description, and routed sample bounds use the same pane-width state, so collapsing the pane reclaims the horizontal space instead of leaving a 300-DIP blank gutter. Top-level catalog entries carry native `SymbolIcon` values; leaf route identity remains the stable catalog ID. UI Automation verifies that the standalone search stays inside the TitleBar band, the NavigationView does not expose its legacy toggle button, the TitleBar toggle compacts and reopens the pane, and page content shifts with that state.

## Search ownership

The Gallery AutoSuggestBox remains a separate shell child instead of being assigned to `NavigationView.AutoSuggestBox` or `TitleBar.Content`. A Winio child already owns its native element through the Winio Canvas; transferring that same element into another native parent would make ownership and reparenting ambiguous. The shell therefore positions the standalone control in the TitleBar visual band while keeping one clear Winio owner. It uses the same catalog as NavigationView and routes only stable sample IDs or exact ID/title matches.

## M3 remains future work

M3 is not included in this architecture milestone. Reusable sample cards, source-code display, live options and output chrome, and a broader sample inventory remain future work. The Gallery does not claim parity with the official WinUI Gallery.
