use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{
    AsContainer, AsWidget, AsWindow, BorrowedContainer, BorrowedWidget, BorrowedWindow,
};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::TitleBar as NativeTitleBar;
use winio_winui3::Microsoft::UI::Xaml as MUX;

use crate::native::NativeElement;

pub struct IntegratedTitleBarInit<'a> {
    window: BorrowedWindow<'a>,
    container: BorrowedContainer<'a>,
}

impl<'a, T> From<&'a T> for IntegratedTitleBarInit<'a>
where
    T: AsWindow + AsContainer + ?Sized,
{
    fn from(value: &'a T) -> Self {
        Self {
            window: value.as_window(),
            container: value.as_container(),
        }
    }
}

#[derive(Debug)]
pub struct IntegratedTitleBar {
    native: NativeTitleBar,
    element: NativeElement,
    window: MUX::Window,
}

impl Failable for IntegratedTitleBar {
    type Error = Error;
}

impl IntegratedTitleBar {
    pub fn title(&self) -> Result<String> {
        Ok(self.native.Title()?.to_string_lossy())
    }

    pub fn set_title(&mut self, title: impl AsRef<str>) -> Result<()> {
        self.native.SetTitle(&HSTRING::from(title.as_ref()))?;
        Ok(())
    }

    pub fn subtitle(&self) -> Result<String> {
        Ok(self.native.Subtitle()?.to_string_lossy())
    }

    pub fn set_subtitle(&mut self, subtitle: impl AsRef<str>) -> Result<()> {
        self.native.SetSubtitle(&HSTRING::from(subtitle.as_ref()))?;
        Ok(())
    }

    pub fn is_integrated(&self) -> Result<bool> {
        self.window.ExtendsContentIntoTitleBar()
    }
}

impl Visible for IntegratedTitleBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for IntegratedTitleBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for IntegratedTitleBar {
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
pub enum IntegratedTitleBarMessage {}

impl Component for IntegratedTitleBar {
    type Error = Error;
    type Event = ();
    type Init<'a> = IntegratedTitleBarInit<'a>;
    type Message = IntegratedTitleBarMessage;

    async fn init(init: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let window = init.window.as_winui().clone();
        let native = NativeTitleBar::new()?;
        let framework_element = native.cast::<MUX::FrameworkElement>()?;
        let ui_element = native.cast::<MUX::UIElement>()?;
        let element = NativeElement::new(init.container, framework_element)?;

        window.SetExtendsContentIntoTitleBar(true)?;
        window.SetTitleBar(&ui_element)?;

        Ok(Self {
            native,
            element,
            window,
        })
    }
}

impl AsWidget for IntegratedTitleBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for IntegratedTitleBar {
    fn drop(&mut self) {
        let _ = self.window.SetTitleBar(None::<&MUX::UIElement>);
        let _ = self.window.SetExtendsContentIntoTitleBar(false);
    }
}
