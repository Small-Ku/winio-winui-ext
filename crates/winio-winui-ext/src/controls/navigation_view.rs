use windows::Foundation::{IPropertyValue, PropertyValue, TypedEventHandler};
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    NavigationView as NativeNavigationView, NavigationViewItem, NavigationViewPaneDisplayMode,
    NavigationViewSelectionChangedEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// One stable item in a [`NavigationView`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationViewItemDescriptor {
    pub id: String,
    pub text: String,
}

impl NavigationViewItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

/// A native WinUI `NavigationView` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct NavigationView {
    native: NativeNavigationView,
    element: NativeElement,
    selection_changed_token: i64,
}

impl Failable for NavigationView {
    type Error = Error;
}

impl NavigationView {
    /// Replaces the menu with the supplied stable identities and visible text.
    pub fn set_items(&mut self, items: &[NavigationViewItemDescriptor]) -> Result<()> {
        let menu = self.native.MenuItems()?;
        menu.Clear()?;
        for item in items {
            let native_item = NavigationViewItem::new()?;
            let text = MUXC::TextBlock::new()?;
            text.SetText(&HSTRING::from(item.text.as_str()))?;
            native_item.SetContent(&text)?;
            let id = PropertyValue::CreateString(&HSTRING::from(item.id.as_str()))?;
            native_item.SetTag(&id)?;
            native_item.SetSelectsOnInvoked(true)?;
            menu.Append(&native_item)?;
        }
        Ok(())
    }
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

/// Output events emitted by [`NavigationView`].
#[derive(Debug)]
#[non_exhaustive]
pub enum NavigationViewEvent {
    /// A native selection changed. Settings or an untagged item has no id.
    SelectionChanged { selected_id: Option<String> },
}

/// Input messages for [`NavigationView`].
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
        native.SetPaneDisplayMode(NavigationViewPaneDisplayMode::Left)?;

        let selection_sender = sender.clone();
        let selection_changed_token =
            native.SelectionChanged(&TypedEventHandler::<
                NativeNavigationView,
                NavigationViewSelectionChangedEventArgs,
            >::new(move |_, args| {
                let selected_id = args
                    .as_ref()
                    .and_then(|args| args.SelectedItem().ok())
                    .and_then(|item| item.cast::<NavigationViewItem>().ok())
                    .and_then(|item| item.Tag().ok())
                    .and_then(|tag| tag.cast::<IPropertyValue>().ok())
                    .and_then(|tag| tag.GetString().ok())
                    .map(|id| id.to_string_lossy());
                selection_sender.output(NavigationViewEvent::SelectionChanged { selected_id });
                Ok(())
            }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self {
            native,
            element,
            selection_changed_token,
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
    }
}
