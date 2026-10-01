use windows::Foundation::{EventHandler, TypedEventHandler};
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::FlyoutBase;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    Flyout, SplitButton as NativeSplitButton, SplitButtonClickEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// A native WinUI SplitButton with an owned flyout content surface.
#[derive(Debug)]
pub struct SplitButton {
    native: NativeSplitButton,
    element: NativeElement,
    text: MUXC::TextBlock,
    flyout_base: FlyoutBase,
    flyout_content: MUXC::Canvas,
    click_token: i64,
    opened_token: i64,
    closed_token: i64,
}

impl Failable for SplitButton {
    type Error = Error;
}

impl SplitButton {
    /// Returns the flyout content Canvas so Winio children can be constructed
    /// directly under their final native owner.
    pub fn flyout_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.flyout_content)
    }

    pub fn flyout_content_size(&self) -> Result<Size> {
        Ok(Size::new(
            self.flyout_content.Width()?,
            self.flyout_content.Height()?,
        ))
    }

    pub fn set_flyout_content_size(&mut self, size: Size) -> Result<()> {
        self.flyout_content.SetWidth(size.width)?;
        self.flyout_content.SetHeight(size.height)?;
        Ok(())
    }

    pub fn hide_flyout(&self) -> Result<()> {
        self.flyout_base.Hide()?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl TextWidget for SplitButton {
    fn text(&self) -> Result<String> {
        Ok(self.text.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.text.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for SplitButton {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for SplitButton {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for SplitButton {
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
pub enum SplitButtonEvent {
    Click,
    FlyoutOpened,
    FlyoutClosed,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SplitButtonMessage {}

impl Component for SplitButton {
    type Error = Error;
    type Event = SplitButtonEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = SplitButtonMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeSplitButton::new()?;
        let text = MUXC::TextBlock::new()?;
        native.cast::<MUXC::ContentControl>()?.SetContent(&text)?;

        let flyout = Flyout::new()?;
        let flyout_content = MUXC::Canvas::new()?;
        flyout_content.SetWidth(260.0)?;
        flyout_content.SetHeight(72.0)?;
        flyout.SetContent(&flyout_content)?;
        let flyout_base = flyout.cast::<FlyoutBase>()?;
        native.SetFlyout(&flyout_base)?;

        let click_sender = sender.clone();
        let click_token = native.Click(&TypedEventHandler::<
            NativeSplitButton,
            SplitButtonClickEventArgs,
        >::new(move |_, _| {
            click_sender.output(SplitButtonEvent::Click);
            Ok(())
        }))?;

        let opened_sender = sender.clone();
        let opened_token =
            flyout_base.Opened(&EventHandler::<IInspectable>::new(move |_, _| {
                opened_sender.output(SplitButtonEvent::FlyoutOpened);
                Ok(())
            }))?;

        let closed_sender = sender.clone();
        let closed_token =
            flyout_base.Closed(&EventHandler::<IInspectable>::new(move |_, _| {
                closed_sender.output(SplitButtonEvent::FlyoutClosed);
                Ok(())
            }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            text,
            flyout_base,
            flyout_content,
            click_token,
            opened_token,
            closed_token,
        })
    }
}

impl AsWidget for SplitButton {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for SplitButton {
    fn drop(&mut self) {
        let _ = self.native.RemoveClick(self.click_token);
        let _ = self.flyout_base.RemoveOpened(self.opened_token);
        let _ = self.flyout_base.RemoveClosed(self.closed_token);
    }
}
