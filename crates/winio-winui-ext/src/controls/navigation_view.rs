use windows::Foundation::{IPropertyValue, PropertyValue, TypedEventHandler};
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    NavigationView as NativeNavigationView, NavigationViewItem,
    NavigationViewPaneDisplayMode as NativePaneDisplayMode,
    NavigationViewSelectionChangedEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// One stable item in a [`NavigationView`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItemDescriptor {
    pub id: String,
    pub text: String,
    pub children: Vec<NavigationViewItemDescriptor>,
}

impl NavigationViewItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            children: Vec::new(),
        }
    }

    pub fn with_children(
        mut self,
        children: impl IntoIterator<Item = NavigationViewItemDescriptor>,
    ) -> Self {
        self.children.extend(children);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavigationViewPaneDisplayMode {
    Auto,
    Left,
    Top,
    LeftCompact,
    LeftMinimal,
    Unknown(i32),
}

impl From<NativePaneDisplayMode> for NavigationViewPaneDisplayMode {
    fn from(value: NativePaneDisplayMode) -> Self {
        match value.0 {
            0 => Self::Auto,
            1 => Self::Left,
            2 => Self::Top,
            3 => Self::LeftCompact,
            4 => Self::LeftMinimal,
            value => Self::Unknown(value),
        }
    }
}

impl From<NavigationViewPaneDisplayMode> for NativePaneDisplayMode {
    fn from(value: NavigationViewPaneDisplayMode) -> Self {
        match value {
            NavigationViewPaneDisplayMode::Auto => NativePaneDisplayMode::Auto,
            NavigationViewPaneDisplayMode::Left => NativePaneDisplayMode::Left,
            NavigationViewPaneDisplayMode::Top => NativePaneDisplayMode::Top,
            NavigationViewPaneDisplayMode::LeftCompact => NativePaneDisplayMode::LeftCompact,
            NavigationViewPaneDisplayMode::LeftMinimal => NativePaneDisplayMode::LeftMinimal,
            NavigationViewPaneDisplayMode::Unknown(value) => NativePaneDisplayMode(value),
        }
    }
}

/// A native WinUI `NavigationView` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct NavigationView {
    native: NativeNavigationView,
    element: NativeElement,
    selection_changed_token: i64,
    back_requested_token: i64,
}

impl Failable for NavigationView {
    type Error = Error;
}

impl NavigationView {
    /// Replaces the main menu with the supplied recursive stable identities.
    pub fn set_items(&mut self, items: &[NavigationViewItemDescriptor]) -> Result<()> {
        let menu = self.native.MenuItems()?;
        menu.Clear()?;
        for item in items {
            menu.Append(&build_item(item)?)?;
        }
        Ok(())
    }

    /// Replaces the footer menu with the supplied recursive stable identities.
    pub fn set_footer_items(&mut self, items: &[NavigationViewItemDescriptor]) -> Result<()> {
        let menu = self.native.FooterMenuItems()?;
        menu.Clear()?;
        for item in items {
            menu.Append(&build_item(item)?)?;
        }
        Ok(())
    }

    pub fn selected_id(&self) -> Result<Option<String>> {
        Ok(item_id(&self.native.SelectedItem()?))
    }

    /// Selects the first matching item in the main or footer hierarchy.
    pub fn set_selected_id(&mut self, id: &str) -> Result<bool> {
        let item = find_item(&self.native, id)?;
        if let Some(item) = item {
            self.native.SetSelectedItem(&item)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_pane_open(&self) -> Result<bool> {
        self.native.IsPaneOpen()
    }
    pub fn set_is_pane_open(&mut self, value: bool) -> Result<()> {
        self.native.SetIsPaneOpen(value)?;
        Ok(())
    }
    pub fn is_settings_visible(&self) -> Result<bool> {
        self.native.IsSettingsVisible()
    }
    pub fn set_is_settings_visible(&mut self, value: bool) -> Result<()> {
        self.native.SetIsSettingsVisible(value)?;
        Ok(())
    }
    pub fn pane_display_mode(&self) -> Result<NavigationViewPaneDisplayMode> {
        Ok(self.native.PaneDisplayMode()?.into())
    }
    pub fn set_pane_display_mode(&mut self, value: NavigationViewPaneDisplayMode) -> Result<()> {
        self.native.SetPaneDisplayMode(value.into())?;
        Ok(())
    }

    pub fn set_header_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        let header = MUXC::TextBlock::new()?;
        header.SetText(&HSTRING::from(value.as_ref()))?;
        self.native.SetHeader(&header)?;
        Ok(())
    }
}

fn build_item(descriptor: &NavigationViewItemDescriptor) -> Result<NavigationViewItem> {
    let native_item = NavigationViewItem::new()?;
    let text = MUXC::TextBlock::new()?;
    text.SetText(&HSTRING::from(descriptor.text.as_str()))?;
    native_item
        .cast::<MUXC::ContentControl>()?
        .SetContent(&text)?;
    let id = PropertyValue::CreateString(&HSTRING::from(descriptor.id.as_str()))?;
    native_item
        .cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?
        .SetTag(&id)?;
    native_item.SetSelectsOnInvoked(true)?;
    let children = native_item.MenuItems()?;
    for child in &descriptor.children {
        children.Append(&build_item(child)?)?;
    }
    Ok(native_item)
}

fn item_id(item: &IInspectable) -> Option<String> {
    item.cast::<NavigationViewItem>()
        .ok()
        .and_then(|item| {
            item.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()
                .ok()
        })
        .and_then(|item| item.Tag().ok())
        .and_then(|tag| tag.cast::<IPropertyValue>().ok())
        .and_then(|tag| tag.GetString().ok())
        .map(|id| id.to_string_lossy())
}

fn find_item(native: &NativeNavigationView, id: &str) -> Result<Option<NavigationViewItem>> {
    let main = native.MenuItems()?;
    for index in 0..main.Size()? {
        if let Ok(item) = main.GetAt(index)?.cast::<NavigationViewItem>() {
            if item_matches_id(&item, id)? {
                return Ok(Some(item));
            }
            if let Some(found) = find_in_children(&item, id)? {
                return Ok(Some(found));
            }
        }
    }
    let footer = native.FooterMenuItems()?;
    for index in 0..footer.Size()? {
        if let Ok(item) = footer.GetAt(index)?.cast::<NavigationViewItem>() {
            if item_matches_id(&item, id)? {
                return Ok(Some(item));
            }
            if let Some(found) = find_in_children(&item, id)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

fn item_matches_id(item: &NavigationViewItem, id: &str) -> Result<bool> {
    Ok(item
        .cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?
        .Tag()
        .ok()
        .and_then(|tag| tag.cast::<IPropertyValue>().ok())
        .and_then(|tag| tag.GetString().ok())
        .is_some_and(|tag| tag.to_string_lossy() == id))
}

fn find_in_children(parent: &NavigationViewItem, id: &str) -> Result<Option<NavigationViewItem>> {
    let children = parent.MenuItems()?;
    for index in 0..children.Size()? {
        if let Ok(item) = children.GetAt(index)?.cast::<NavigationViewItem>() {
            if item_matches_id(&item, id)? {
                return Ok(Some(item));
            }
            if let Some(found) = find_in_children(&item, id)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

impl Visible for NavigationView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}
impl Enable for NavigationView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}
impl Layoutable for NavigationView {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }
    fn set_loc(&mut self, point: Point) -> Result<()> {
        self.element.set_loc(point)
    }
    fn size(&self) -> Result<Size> {
        self.element.size()
    }
    fn set_size(&mut self, size: Size) -> Result<()> {
        self.element.set_size(size)
    }
    fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }
    fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum NavigationViewEvent {
    SelectionChanged {
        selected_id: Option<String>,
        is_settings_selected: bool,
    },
    BackRequested,
}
#[derive(Debug)]
#[non_exhaustive]
pub enum NavigationViewMessage {}

impl Component for NavigationView {
    type Error = Error;
    type Event = NavigationViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = NavigationViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeNavigationView::new()?;
        native.SetPaneDisplayMode(NativePaneDisplayMode::Left)?;
        let selection_sender = sender.clone();
        let selection_changed_token =
            native.SelectionChanged(&TypedEventHandler::<
                NativeNavigationView,
                NavigationViewSelectionChangedEventArgs,
            >::new(move |_, args| {
                let (selected_id, is_settings_selected) = args
                    .as_ref()
                    .map(|args| {
                        (
                            args.SelectedItem().ok().and_then(|item| item_id(&item)),
                            args.IsSettingsSelected().unwrap_or(false),
                        )
                    })
                    .unwrap_or((None, false));
                selection_sender.output(NavigationViewEvent::SelectionChanged {
                    selected_id,
                    is_settings_selected,
                });
                Ok(())
            }))?;
        let back_sender = sender.clone();
        let back_requested_token = native.BackRequested(&TypedEventHandler::<NativeNavigationView, winio_winui_bindings::Microsoft::UI::Xaml::Controls::NavigationViewBackRequestedEventArgs>::new(move |_, _| {
            back_sender.output(NavigationViewEvent::BackRequested);
            Ok(())
        }))?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self {
            native,
            element,
            selection_changed_token,
            back_requested_token,
        })
    }
}
impl AsWidget for NavigationView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
impl Drop for NavigationView {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectionChanged(self.selection_changed_token);
        let _ = self.native.RemoveBackRequested(self.back_requested_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_display_mode_maps_native_values_and_round_trips() {
        let cases = [
            (
                NativePaneDisplayMode::Auto,
                NavigationViewPaneDisplayMode::Auto,
            ),
            (
                NativePaneDisplayMode::Left,
                NavigationViewPaneDisplayMode::Left,
            ),
            (
                NativePaneDisplayMode::Top,
                NavigationViewPaneDisplayMode::Top,
            ),
            (
                NativePaneDisplayMode::LeftCompact,
                NavigationViewPaneDisplayMode::LeftCompact,
            ),
            (
                NativePaneDisplayMode::LeftMinimal,
                NavigationViewPaneDisplayMode::LeftMinimal,
            ),
        ];
        for (native, public) in cases {
            assert_eq!(NavigationViewPaneDisplayMode::from(native), public);
            assert_eq!(NativePaneDisplayMode::from(public), native);
        }
        assert_eq!(
            NavigationViewPaneDisplayMode::from(NativePaneDisplayMode(99)),
            NavigationViewPaneDisplayMode::Unknown(99)
        );
        assert_eq!(
            NativePaneDisplayMode::from(NavigationViewPaneDisplayMode::Unknown(99)),
            NativePaneDisplayMode(99)
        );
    }

    #[test]
    fn descriptor_builder_preserves_recursive_identity() {
        let item = NavigationViewItemDescriptor::new("controls", "Controls")
            .with_children([NavigationViewItemDescriptor::new("tree", "TreeView")]);
        assert_eq!(item.children.len(), 1);
        assert_eq!(item.children[0].id, "tree");
    }
}
