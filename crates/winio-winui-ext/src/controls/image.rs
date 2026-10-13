use windows::Foundation::Uri;
use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::{
    Controls::Image as NativeImage, ExceptionRoutedEventHandler,
};
use winio_winui3::Microsoft::UI::Xaml::{Media::Stretch, RoutedEventHandler, Thickness};

pub use winio_winui_bindings::Microsoft::UI::Xaml::Media::{ImageSource, Imaging::BitmapImage};

use crate::native::NativeElement;

/// A native WinUI Image with explicit native image-source ownership.
///
/// Callers can provide an ImageSource directly. BitmapImage and URI setters
/// provide simpler ways to set a native image source.
#[derive(Debug)]
pub struct Image {
    native: NativeImage,
    element: NativeElement,
    opened_token: i64,
    failed_token: i64,
}

impl Failable for Image {
    type Error = Error;
}

impl Image {
    pub fn source(&self) -> Result<ImageSource> {
        self.native.Source()
    }

    pub fn set_source(&mut self, source: &ImageSource) -> Result<()> {
        self.native.SetSource(source)?;
        Ok(())
    }

    pub fn set_bitmap_image(&mut self, source: &BitmapImage) -> Result<()> {
        let source = source.cast::<ImageSource>()?;
        self.set_source(&source)
    }

    pub fn set_uri_source(&mut self, uri: &Uri) -> Result<()> {
        let source = BitmapImage::new()?;
        source.SetUriSource(uri)?;
        self.set_bitmap_image(&source)
    }

    pub fn stretch(&self) -> Result<Stretch> {
        self.native.Stretch()
    }

    pub fn set_stretch(&mut self, value: Stretch) -> Result<()> {
        self.native.SetStretch(value)?;
        Ok(())
    }

    pub fn nine_grid(&self) -> Result<Thickness> {
        self.native.NineGrid()
    }

    pub fn set_nine_grid(&mut self, value: Thickness) -> Result<()> {
        self.native.SetNineGrid(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for Image {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for Image {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for Image {
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImageEvent {
    Opened,
    Failed { message: String },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ImageMessage {}

impl Component for Image {
    type Error = Error;
    type Event = ImageEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ImageMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeImage::new()?;

        let opened_sender = sender.clone();
        let opened_token = native.ImageOpened(&RoutedEventHandler::new(move |_, _| {
            opened_sender.output(ImageEvent::Opened);
            Ok(())
        }))?;

        let failed_sender = sender.clone();
        let failed_token =
            native.ImageFailed(&ExceptionRoutedEventHandler::new(move |_, args| {
                let message = args
                    .as_ref()
                    .and_then(|args| args.ErrorMessage().ok())
                    .map(|message| message.to_string_lossy())
                    .unwrap_or_default();
                failed_sender.output(ImageEvent::Failed { message });
                Ok(())
            }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            opened_token,
            failed_token,
        })
    }
}

impl AsWidget for Image {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        let _ = self.native.RemoveImageOpened(self.opened_token);
        let _ = self.native.RemoveImageFailed(self.failed_token);
    }
}
