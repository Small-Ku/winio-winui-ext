use windows::Foundation::{EventHandler, TypedEventHandler};
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::FlyoutBase;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    Flyout, SplitButton as NativeSplitButton, SplitButtonClickEventArgs,
    ToggleSplitButton as NativeToggleSplitButton,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// A native ToggleSplitButton with an owned flyout Canvas and typed check state.
#[derive(Debug)]
pub struct ToggleSplitButton {
    native: NativeToggleSplitButton,
    element: NativeElement,
    text: MUXC::TextBlock,
    flyout_base: FlyoutBase,
    flyout_content: MUXC::Canvas,
    click_token: i64,
    checked_token: i64,
    opened_token: i64,
    closed_token: i64,
}

impl Failable for ToggleSplitButton {
    type Error = Error;
}

impl ToggleSplitButton {
    pub fn flyout_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.flyout_content)
    }
    pub fn is_checked(&self) -> Result<bool> {
        self.native.IsChecked()
    }
    pub fn set_is_checked(&mut self, checked: bool) -> Result<()> {
        self.native.SetIsChecked(checked)?;
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

impl TextWidget for ToggleSplitButton {
    fn text(&self) -> Result<String> {
        Ok(self.text.Text()?.to_string_lossy())
    }
    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.text.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}
impl Visible for ToggleSplitButton {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}
impl Enable for ToggleSplitButton {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}
impl Layoutable for ToggleSplitButton {
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
pub enum ToggleSplitButtonEvent {
    Click,
    CheckedChanged(bool),
    FlyoutOpened,
    FlyoutClosed,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToggleSplitButtonMessage {}

impl Component for ToggleSplitButton {
    type Error = Error;
    type Event = ToggleSplitButtonEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ToggleSplitButtonMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeToggleSplitButton::new()?;
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
            click_sender.output(ToggleSplitButtonEvent::Click);
            Ok(())
        }))?;
        let checked_sender = sender.clone();
        let checked_native = native.clone();
        let checked_token = native.IsCheckedChanged(&TypedEventHandler::new(move |_, _| {
            checked_sender.output(ToggleSplitButtonEvent::CheckedChanged(
                checked_native.IsChecked()?,
            ));
            Ok(())
        }))?;
        let opened_sender = sender.clone();
        let opened_token =
            flyout_base.Opened(&EventHandler::<IInspectable>::new(move |_, _| {
                opened_sender.output(ToggleSplitButtonEvent::FlyoutOpened);
                Ok(())
            }))?;
        let closed_sender = sender.clone();
        let closed_token =
            flyout_base.Closed(&EventHandler::<IInspectable>::new(move |_, _| {
                closed_sender.output(ToggleSplitButtonEvent::FlyoutClosed);
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
            checked_token,
            opened_token,
            closed_token,
        })
    }
}

impl AsWidget for ToggleSplitButton {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for ToggleSplitButton {
    fn drop(&mut self) {
        let _ = self.native.RemoveClick(self.click_token);
        let _ = self.native.RemoveIsCheckedChanged(self.checked_token);
        let _ = self.flyout_base.RemoveOpened(self.opened_token);
        let _ = self.flyout_base.RemoveClosed(self.closed_token);
    }
}
