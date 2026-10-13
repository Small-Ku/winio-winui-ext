use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::RepeatButton as NativeRepeatButton;
use winio_winui3::Microsoft::UI::Xaml::Controls::Primitives as MUXCP;
use winio_winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

/// A native WinUI RepeatButton with explicit repeat timing and typed clicks.
#[derive(Debug)]
pub struct RepeatButton {
    native: NativeRepeatButton,
    element: NativeElement,
    text: MUXC::TextBlock,
    button_base: MUXCP::ButtonBase,
    click_token: i64,
}

impl Failable for RepeatButton {
    type Error = Error;
}

impl RepeatButton {
    pub fn delay(&self) -> Result<i32> {
        self.native.Delay()
    }

    pub fn set_delay(&mut self, value: i32) -> Result<()> {
        self.native.SetDelay(value)?;
        Ok(())
    }

    pub fn interval(&self) -> Result<i32> {
        self.native.Interval()
    }

    pub fn set_interval(&mut self, value: i32) -> Result<()> {
        self.native.SetInterval(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl TextWidget for RepeatButton {
    fn text(&self) -> Result<String> {
        Ok(self.text.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.text.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for RepeatButton {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for RepeatButton {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for RepeatButton {
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
pub enum RepeatButtonEvent {
    Click,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RepeatButtonMessage {}

impl Component for RepeatButton {
    type Error = Error;
    type Event = RepeatButtonEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = RepeatButtonMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeRepeatButton::new()?;
        let text = MUXC::TextBlock::new()?;
        native.cast::<MUXC::ContentControl>()?.SetContent(&text)?;
        let button_base = native.cast::<MUXCP::ButtonBase>()?;

        let click_sender = sender.clone();
        let click_token = button_base.Click(&RoutedEventHandler::new(move |_, _| {
            click_sender.output(RepeatButtonEvent::Click);
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            text,
            button_base,
            click_token,
        })
    }
}

impl AsWidget for RepeatButton {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for RepeatButton {
    fn drop(&mut self) {
        let _ = self.button_base.RemoveClick(self.click_token);
    }
}
