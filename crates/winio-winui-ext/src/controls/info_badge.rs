use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::InfoBadge as NativeInfoBadge;

use crate::native::NativeElement;

/// A native WinUI InfoBadge hosted as a first-class Winio component.
#[derive(Debug)]
pub struct InfoBadge {
    native: NativeInfoBadge,
    element: NativeElement,
}

impl Failable for InfoBadge {
    type Error = Error;
}

impl InfoBadge {
    /// The displayed numeric value, or a non-positive value for the dot state.
    pub fn value(&self) -> Result<i32> {
        self.native.Value()
    }

    pub fn set_value(&mut self, value: i32) -> Result<()> {
        self.native.SetValue2(value)?;
        Ok(())
    }
}

impl Visible for InfoBadge {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}
impl Enable for InfoBadge {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}
impl Layoutable for InfoBadge {
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
pub enum InfoBadgeMessage {}

impl Component for InfoBadge {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = InfoBadgeMessage;

    async fn init(parent: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeInfoBadge::new()?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self { native, element })
    }
}

impl AsWidget for InfoBadge {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
