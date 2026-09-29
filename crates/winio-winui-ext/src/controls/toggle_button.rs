use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winui3::Microsoft::UI::Xaml::Controls::Primitives as MUXCP;
use winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

/// A native WinUI `ToggleButton` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct ToggleButton {
    native: MUXCP::ToggleButton,
    element: NativeElement,
    text: MUXC::TextBlock,
    click_token: i64,
}

impl Failable for ToggleButton {
    type Error = Error;
}

impl TextWidget for ToggleButton {
    fn text(&self) -> Result<String> {
        Ok(self.text.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.text.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for ToggleButton {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for ToggleButton {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for ToggleButton {
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
}

/// Events emitted by [`ToggleButton`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ToggleButtonEvent {
    /// The native WinUI toggle button was clicked.
    Click,
}

/// Input messages for [`ToggleButton`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ToggleButtonMessage {}

impl Component for ToggleButton {
    type Error = Error;
    type Event = ToggleButtonEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ToggleButtonMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = MUXCP::ToggleButton::new()?;
        let text = MUXC::TextBlock::new()?;
        native.SetContent(&text)?;

        let event_sender = sender.clone();
        let click_token = native.Click(&RoutedEventHandler::new(move |_, _| {
            event_sender.output(ToggleButtonEvent::Click);
            Ok(())
        }))?;

        let element = NativeElement::new(parent, native.cast()?)?;

        Ok(Self {
            native,
            element,
            text,
            click_token,
        })
    }
}

impl AsWidget for ToggleButton {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for ToggleButton {
    fn drop(&mut self) {
        let _ = self.native.RemoveClick(self.click_token);
    }
}
