use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui3::Microsoft::UI::Xaml::Controls::ProgressBar as NativeProgressBar;

use crate::native::NativeElement;

/// A native WinUI ProgressBar hosted as a first-class Winio component.
#[derive(Debug)]
pub struct ProgressBar {
    native: NativeProgressBar,
    element: NativeElement,
}

impl Failable for ProgressBar {
    type Error = Error;
}

impl ProgressBar {
    pub fn is_indeterminate(&self) -> Result<bool> {
        self.native.IsIndeterminate()
    }
    pub fn set_is_indeterminate(&mut self, value: bool) -> Result<()> {
        self.native.SetIsIndeterminate(value)?;
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
    pub fn value(&self) -> Result<f64> {
        self.native.Value()
    }
    pub fn set_value(&mut self, value: f64) -> Result<()> {
        self.native.SetValue(value)?;
        Ok(())
    }
    pub fn show_error(&self) -> Result<bool> {
        self.native.ShowError()
    }
    pub fn set_show_error(&mut self, value: bool) -> Result<()> {
        self.native.SetShowError(value)?;
        Ok(())
    }
    pub fn show_paused(&self) -> Result<bool> {
        self.native.ShowPaused()
    }
    pub fn set_show_paused(&mut self, value: bool) -> Result<()> {
        self.native.SetShowPaused(value)?;
        Ok(())
    }
}

impl Visible for ProgressBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, value: bool) -> Result<()> {
        self.element.set_visible(value)
    }
}
impl Enable for ProgressBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, value: bool) -> Result<()> {
        self.element.set_enabled(value)
    }
}
impl Layoutable for ProgressBar {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }
    fn set_loc(&mut self, value: Point) -> Result<()> {
        self.element.set_loc(value)
    }
    fn size(&self) -> Result<Size> {
        self.element.size()
    }
    fn set_size(&mut self, value: Size) -> Result<()> {
        self.element.set_size(value)
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
pub enum ProgressBarMessage {}

impl Component for ProgressBar {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ProgressBarMessage;
    async fn init(parent: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeProgressBar::new()?;
        let element = NativeElement::new(
            parent,
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?,
        )?;
        Ok(Self { native, element })
    }
}
impl AsWidget for ProgressBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
