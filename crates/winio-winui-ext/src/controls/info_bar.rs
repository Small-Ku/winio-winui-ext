use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::InfoBar as NativeInfoBar;

use crate::native::NativeElement;

/// A native WinUI `InfoBar` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct InfoBar {
    native: NativeInfoBar,
    element: NativeElement,
}

impl Failable for InfoBar {
    type Error = Error;
}

impl InfoBar {
    /// Returns the InfoBar message.
    pub fn message(&self) -> Result<String> {
        Ok(self.native.Message()?.to_string_lossy())
    }

    /// Sets the InfoBar message.
    pub fn set_message(&mut self, message: impl AsRef<str>) -> Result<()> {
        self.native.SetMessage(&HSTRING::from(message.as_ref()))?;
        Ok(())
    }

    /// Returns whether the InfoBar is open.
    pub fn is_open(&self) -> Result<bool> {
        self.native.IsOpen()
    }

    /// Opens or closes the InfoBar.
    pub fn set_is_open(&mut self, is_open: bool) -> Result<()> {
        self.native.SetIsOpen(is_open)?;
        Ok(())
    }

    /// Returns whether WinUI shows the native close affordance.
    pub fn is_closable(&self) -> Result<bool> {
        self.native.IsClosable()
    }

    /// Shows or hides the native close affordance.
    pub fn set_is_closable(&mut self, is_closable: bool) -> Result<()> {
        self.native.SetIsClosable(is_closable)?;
        Ok(())
    }
}

impl Visible for InfoBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for InfoBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for InfoBar {
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

/// Input messages for [`InfoBar`].
#[derive(Debug)]
#[non_exhaustive]
pub enum InfoBarMessage {}

impl Component for InfoBar {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = InfoBarMessage;

    async fn init(parent: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeInfoBar::new()?;
        // Cast to Winio's FrameworkElement type before attaching the control.
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self { native, element })
    }
}

impl AsWidget for InfoBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
