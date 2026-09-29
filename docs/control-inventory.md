# WinUI control inventory (Winio 0.12 / winio-winui3 0.4.4)

This inventory was checked against the public high-level widget modules in the `winio 0.12.0` crate source and the generated public `Microsoft.UI.Xaml.Controls` / `Controls.Primitives` modules in the locked `winio-winui3 0.4.4` source. It is about projection availability, not wrapper completeness or proof that every control can be hosted correctly.

## High-level Winio widgets and useful projected equivalents

| Winio 0.12 widget area | Projected native control(s) in 0.4.4 | Notes |
| --- | --- | --- |
| Button | `Button`, `Primitives::ToggleButton` | ToggleButton is the current wrapper. |
| Check box / radio button | `CheckBox`, `RadioButton` | Projected. |
| Combo box / list box | `ComboBox`, `ListBox` | Projected. |
| Edit / text box / label / link label | `TextBox`, `PasswordBox`, `TextBlock`, `HyperlinkButton` | HyperlinkButton is available for a future native link wrapper. |
| Progress / slider / scroll bar | `ProgressBar`, `Slider`, `Primitives::ScrollBar` | Projected. |
| Scroll view / canvas / tab view | `ScrollViewer`, `Canvas`, `TabView` | Projected. |
| View | `Canvas` as the backend's host surface | A hosting surface, not a stand-alone view equivalent. |
| Window / media / web view / WGPU | `Xaml::Window`, `MediaPlayerElement`, `WebView2`, `SwapChainPanel` | Projected, but these have broader lifecycle or runtime integration needs. |

The 0.4.4 projection also includes `ContentDialog` and `PasswordBox`. Each still needs an API and lifecycle review before becoming a Winio wrapper.

## Important controls absent from the 0.4.4 projection

Checked by searching the generated public controls module; these names are not present as public control structs in the compatible projection:

- `AutoSuggestBox`
- `ToggleSwitch`
- `NavigationView`
- `NumberBox`
- `TreeView`
- `TeachingTip`
- `Expander`, `FlipView`, `PersonPicture`, `CommandBar`, and `MenuBar`
- `CalendarDatePicker`, `DatePicker`, `TimePicker`, `ColorPicker`, `RatingControl`, `SplitButton`, and `DropDownButton`

`InfoBar` now has a **supplemental canary projection/wrapper** in
`winio-winui-bindings`; the current generated surface is intentionally limited to the
wrapper's canary properties rather than the full InfoBar API, and InfoBar remains absent
from the upstream 0.4.4 projection.
`NavigationView`, `TreeView`, and the other listed controls remain absent and
are not wrapped in this round. See [supplemental bindings](supplemental-bindings.md)
for its provenance and the narrow shared-type reference boundary.
