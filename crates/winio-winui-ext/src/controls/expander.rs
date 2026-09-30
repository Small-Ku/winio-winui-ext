use windows::Foundation::TypedEventHandler;
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsContainer, AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    Expander as NativeExpander, ExpanderCollapsedEventArgs, ExpanderExpandingEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// A native WinUI Expander that can own Winio children from construction time.
#[derive(Debug)]
pub struct Expander {
    native: NativeExpander,
    element: NativeElement,
    header: MUXC::TextBlock,
    content: MUXC::Canvas,
    expanding_token: i64,
    collapsed_token: i64,
}

impl Failable for Expander {
    type Error = Error;
}

impl Expander {
    pub fn is_expanded(&self) -> Result<bool> {
        self.native.IsExpanded()
    }

    pub fn set_is_expanded(&mut self, expanded: bool) -> Result<()> {
        self.native.SetIsExpanded(expanded)?;
        Ok(())
    }

    pub fn content_size(&self) -> Result<Size> {
        Ok(Size::new(self.content.Width()?, self.content.Height()?))
    }

    pub fn set_content_size(&mut self, size: Size) -> Result<()> {
        self.content.SetWidth(size.width)?;
        self.content.SetHeight(size.height)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl TextWidget for Expander {
    fn text(&self) -> Result<String> {
        Ok(self.header.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.header.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for Expander {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for Expander {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for Expander {
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
pub enum ExpanderEvent {
    Expanding,
    Collapsed,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ExpanderMessage {}

impl Component for Expander {
    type Error = Error;
    type Event = ExpanderEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ExpanderMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeExpander::new()?;
        let header = MUXC::TextBlock::new()?;
        native.SetHeader(&header)?;

        let content = MUXC::Canvas::new()?;
        content.SetWidth(320.0)?;
        content.SetHeight(80.0)?;
        native
            .cast::<MUXC::ContentControl>()?
            .SetContent(&content)?;

        let expanding_sender = sender.clone();
        let expanding_token = native.Expanding(&TypedEventHandler::<
            NativeExpander,
            ExpanderExpandingEventArgs,
        >::new(move |_, _| {
            expanding_sender.output(ExpanderEvent::Expanding);
            Ok(())
        }))?;

        let collapsed_sender = sender.clone();
        let collapsed_token = native.Collapsed(&TypedEventHandler::<
            NativeExpander,
            ExpanderCollapsedEventArgs,
        >::new(move |_, _| {
            collapsed_sender.output(ExpanderEvent::Collapsed);
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            header,
            content,
            expanding_token,
            collapsed_token,
        })
    }
}

impl AsWidget for Expander {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl AsContainer for Expander {
    fn as_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.content)
    }
}

impl Drop for Expander {
    fn drop(&mut self) {
        let _ = self.native.RemoveExpanding(self.expanding_token);
        let _ = self.native.RemoveCollapsed(self.collapsed_token);
    }
}
