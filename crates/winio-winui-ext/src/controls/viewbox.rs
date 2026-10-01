use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsContainer, AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Viewbox as NativeViewbox;
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;
use winio_winui3::Microsoft::UI::Xaml::Media::Stretch;

use crate::native::NativeElement;

/// A native Viewbox that owns a Canvas for Winio children and scales it as one unit.
#[derive(Debug)]
pub struct Viewbox {
    native: NativeViewbox,
    element: NativeElement,
    content: MUXC::Canvas,
}

impl Failable for Viewbox {
    type Error = Error;
}

impl Viewbox {
    /// Returns the owned Canvas so children are inserted beneath the Viewbox at creation.
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

    pub fn stretch(&self) -> Result<Stretch> {
        self.native.Stretch()
    }

    pub fn set_stretch(&mut self, stretch: Stretch) -> Result<()> {
        self.native.SetStretch(stretch)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for Viewbox {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for Viewbox {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for Viewbox {
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

impl Component for Viewbox {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ();

    async fn init(parent: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeViewbox::new()?;
        let content = MUXC::Canvas::new()?;
        content.SetWidth(360.0)?;
        content.SetHeight(180.0)?;
        native.SetChild(&content)?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self {
            native,
            element,
            content,
        })
    }
}

impl AsWidget for Viewbox {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl AsContainer for Viewbox {
    fn as_container(&self) -> BorrowedContainer<'_> {
        self.content_container()
    }
}
