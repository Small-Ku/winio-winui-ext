use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    FlipView as NativeFlipView, FlipViewItem as NativeFlipViewItem,
};
use winio_winui3::Microsoft::UI::Xaml::Controls::{
    self as MUXC, Primitives::Selector, SelectionChangedEventHandler,
};

use crate::native::NativeElement;

/// One stable page hosted by a FlipView.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlipViewPage {
    pub id: String,
    pub text: String,
}

impl FlipViewPage {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

/// A native WinUI FlipView with descriptor-backed pages.
#[derive(Debug)]
pub struct FlipView {
    native: NativeFlipView,
    selector: Selector,
    element: NativeElement,
    pages: Vec<FlipViewPage>,
    selection_changed_token: i64,
}

impl Failable for FlipView {
    type Error = Error;
}

impl FlipView {
    pub fn set_pages(&mut self, pages: &[FlipViewPage]) -> Result<()> {
        let items_control = self.native.cast::<MUXC::ItemsControl>()?;
        let items = items_control.Items()?;
        items.Clear()?;

        for page in pages {
            let item = NativeFlipViewItem::new()?;
            let text = MUXC::TextBlock::new()?;
            text.SetText(&HSTRING::from(page.text.as_str()))?;
            text.SetTextWrapping(winio_winui3::Microsoft::UI::Xaml::TextWrapping::Wrap)?;
            item.cast::<MUXC::ContentControl>()?.SetContent(&text)?;
            items.Append(&item)?;
        }

        self.pages = pages.to_vec();
        if self.pages.is_empty() {
            self.selector.SetSelectedIndex(-1)?;
        } else if self.selector.SelectedIndex()? < 0 {
            self.selector.SetSelectedIndex(0)?;
        }
        Ok(())
    }

    pub fn selected_index(&self) -> Result<i32> {
        self.selector.SelectedIndex()
    }

    pub fn set_selected_index(&mut self, index: i32) -> Result<()> {
        self.selector.SetSelectedIndex(index)?;
        Ok(())
    }

    pub fn selected_id(&self) -> Result<Option<&str>> {
        let index = self.selected_index()?;
        Ok(usize::try_from(index)
            .ok()
            .and_then(|index| self.pages.get(index))
            .map(|page| page.id.as_str()))
    }

    pub fn use_touch_animations_for_all_navigation(&self) -> Result<bool> {
        self.native.UseTouchAnimationsForAllNavigation()
    }

    pub fn set_use_touch_animations_for_all_navigation(&mut self, enabled: bool) -> Result<()> {
        self.native.SetUseTouchAnimationsForAllNavigation(enabled)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for FlipView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for FlipView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for FlipView {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FlipViewEvent {
    SelectionChanged { selected_index: i32 },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FlipViewMessage {}

impl Component for FlipView {
    type Error = Error;
    type Event = FlipViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = FlipViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeFlipView::new()?;
        let selector = native.cast::<Selector>()?;

        let event_sender = sender.clone();
        let event_selector = selector.clone();
        let selection_changed_token =
            selector.SelectionChanged(&SelectionChangedEventHandler::new(move |_, _| {
                if let Ok(selected_index) = event_selector.SelectedIndex() {
                    event_sender.output(FlipViewEvent::SelectionChanged { selected_index });
                }
                Ok(())
            }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            selector,
            element,
            pages: Vec::new(),
            selection_changed_token,
        })
    }
}

impl AsWidget for FlipView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for FlipView {
    fn drop(&mut self) {
        let _ = self
            .selector
            .RemoveSelectionChanged(self.selection_changed_token);
    }
}
