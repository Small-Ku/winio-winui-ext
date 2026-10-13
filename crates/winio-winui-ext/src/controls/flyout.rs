use windows::Foundation::EventHandler;
use windows::core::{IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{BorrowedContainer, BorrowedWidget};
use winio::primitive::{Failable, Size};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Flyout as NativeFlyout;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::FlyoutBase;
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

/// A native WinUI Flyout with a Winio-owned content surface.
///
/// Flyout is popup-backed rather than a child in the page visual tree. Winio
/// children created through content_container live directly under the
/// flyout's final native Canvas; show_at supplies the placement target and
/// therefore the XamlRoot used by WinUI.
#[derive(Debug)]
pub struct Flyout {
    flyout_base: FlyoutBase,
    content: MUXC::Canvas,
    opened_token: i64,
    closed_token: i64,
}

impl Failable for Flyout {
    type Error = Error;
}

impl Flyout {
    pub fn content_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.content)
    }

    pub fn content_size(&self) -> Result<Size> {
        Ok(Size::new(self.content.Width()?, self.content.Height()?))
    }

    pub fn set_content_size(&mut self, size: Size) -> Result<()> {
        self.content.SetWidth(size.width)?;
        self.content.SetHeight(size.height)?;
        Ok(())
    }

    pub fn show_at(&self, target: BorrowedWidget<'_>) -> Result<()> {
        self.flyout_base.ShowAt(target.as_winui())?;
        Ok(())
    }

    pub fn hide(&self) -> Result<()> {
        self.flyout_base.Hide()?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FlyoutEvent {
    Opened,
    Closed,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FlyoutMessage {}

impl Component for Flyout {
    type Error = Error;
    type Event = FlyoutEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = FlyoutMessage;

    async fn init(_parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeFlyout::new()?;
        let content = MUXC::Canvas::new()?;
        content.SetWidth(280.0)?;
        content.SetHeight(88.0)?;
        native.SetContent(&content)?;

        let flyout_base = native.cast::<FlyoutBase>()?;

        let opened_sender = sender.clone();
        let opened_token =
            flyout_base.Opened(&EventHandler::<IInspectable>::new(move |_, _| {
                opened_sender.output(FlyoutEvent::Opened);
                Ok(())
            }))?;

        let closed_sender = sender.clone();
        let closed_token =
            flyout_base.Closed(&EventHandler::<IInspectable>::new(move |_, _| {
                closed_sender.output(FlyoutEvent::Closed);
                Ok(())
            }))?;

        Ok(Self {
            flyout_base,
            content,
            opened_token,
            closed_token,
        })
    }
}

impl Drop for Flyout {
    fn drop(&mut self) {
        let _ = self.flyout_base.RemoveOpened(self.opened_token);
        let _ = self.flyout_base.RemoveClosed(self.closed_token);
        let _ = self.flyout_base.Hide();
    }
}
