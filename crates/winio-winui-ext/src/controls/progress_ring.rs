use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::ProgressRing as NativeProgressRing;

use crate::native::NativeElement;

/// A native WinUI ProgressRing hosted as a first-class Winio component.
#[derive(Debug)]
pub struct ProgressRing {
    native: NativeProgressRing,
    element: NativeElement,
}

impl Failable for ProgressRing {
    type Error = Error;
}

impl ProgressRing {
    pub fn is_active(&self) -> Result<bool> {
        self.native.IsActive()
    }

    pub fn set_is_active(&mut self, active: bool) -> Result<()> {
        self.native.SetIsActive(active)?;
        Ok(())
    }

    pub fn is_indeterminate(&self) -> Result<bool> {
        self.native.IsIndeterminate()
    }

    pub fn set_is_indeterminate(&mut self, indeterminate: bool) -> Result<()> {
        self.native.SetIsIndeterminate(indeterminate)?;
        Ok(())
    }

    pub fn value(&self) -> Result<f64> {
        self.native.Value()
    }

    pub fn set_value(&mut self, value: f64) -> Result<()> {
        self.native.SetValue2(value)?;
        Ok(())
    }

    pub fn minimum(&self) -> Result<f64> {
        self.native.Minimum()
    }

    pub fn set_minimum(&mut self, value: f64) -> Result<()> {
        self.native.SetMinimum(value)?;
        Ok(())
    }

    pub fn maximum(&self) -> Result<f64> {
        self.native.Maximum()
    }

    pub fn set_maximum(&mut self, value: f64) -> Result<()> {
        self.native.SetMaximum(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for ProgressRing {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for ProgressRing {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for ProgressRing {
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
pub enum ProgressRingMessage {}

impl Component for ProgressRing {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ProgressRingMessage;

    async fn init(parent: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeProgressRing::new()?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self { native, element })
    }
}

impl AsWidget for ProgressRing {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
