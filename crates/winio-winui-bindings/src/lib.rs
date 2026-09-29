#![cfg(windows)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

#[rustfmt::skip]
mod generated;

pub use generated::Microsoft;

#[cfg(test)]
mod tests {
    #[test]
    fn shared_xaml_base_types_are_referenced() {
        let generated = include_str!("generated.rs");
        for name in [
            "DependencyObject",
            "IDependencyObject",
            "UIElement",
            "IUIElement",
            "IUIElementOverrides",
            "IUIElementProtected",
            "FrameworkElement",
            "IFrameworkElement",
            "IFrameworkElementOverrides",
            "IFrameworkElementProtected",
            "Control",
            "ContentControl",
            "IContentControl",
            "IContentControlOverrides",
            "IControl",
            "IControlOverrides",
            "IControlProtected",
            "IAnimationObject",
            "IVisualElement",
            "IVisualElement2",
            "RoutedEventArgs",
            "RoutedEventHandler",
        ] {
            assert!(
                !generated.contains(&format!("pub struct {name}("))
                    && !generated.contains(&format!("pub struct {name} {{"))
                    && !declares_interface(generated, name),
                "shared type {name} must come from winio-winui3"
            );
        }
        assert!(generated.contains("pub struct InfoBar("));
        assert!(generated.contains("pub struct InfoBarSeverity(pub i32)"));
        assert!(generated.contains("pub struct NavigationView("));
        assert!(generated.contains("pub struct NavigationViewItem("));
        assert!(generated.contains("pub struct NavigationViewSelectionChangedEventArgs("));
        assert!(generated.contains("pub struct NavigationViewPaneDisplayMode(pub i32)"));
        assert!(generated.contains("windows::Foundation::TypedEventHandler"));
        for shared in [
            "winio_winui3::Microsoft::UI::Xaml::DependencyObject",
            "winio_winui3::Microsoft::UI::Xaml::UIElement",
            "winio_winui3::Microsoft::UI::Xaml::FrameworkElement",
            "winio_winui3::Microsoft::UI::Xaml::Controls::Control",
            "winio_winui3::Microsoft::UI::Xaml::Controls::ContentControl",
            "winio_winui3::Microsoft::UI::Xaml::Controls::IContentControl",
            "winio_winui3::Microsoft::UI::Xaml::Controls::IContentControlOverrides",
            "winio_winui3::Microsoft::UI::Xaml::IUIElement",
            "winio_winui3::Microsoft::UI::Xaml::Controls::IControl",
            "winio_winui3::Microsoft::UI::Composition::IAnimationObject",
            "winio_winui3::Microsoft::UI::Composition::IVisualElement",
            "winio_winui3::Microsoft::UI::Composition::IVisualElement2",
        ] {
            assert!(
                generated.contains(shared),
                "missing shared reference to {shared}"
            );
        }
    }

    fn declares_interface(source: &str, type_name: &str) -> bool {
        let lines: Vec<_> = source.lines().map(str::trim).collect();
        lines.windows(2).any(|pair| {
            pair[0].ends_with("define_interface!(") && pair[1].starts_with(&format!("{type_name},"))
        })
    }
}
