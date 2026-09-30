use windows::Foundation::TypedEventHandler;
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::RatingControl as NativeRatingControl;

use crate::native::NativeElement;

/// A native WinUI RatingControl hosted as a first-class Winio component.
#[derive(Debug)]
pub struct RatingControl {
    native: NativeRatingControl,
    element: NativeElement,
    value_changed_token: i64,
}

impl Failable for RatingControl {
    type Error = Error;
}

impl RatingControl {
    pub fn caption(&self) -> Result<String> {
        Ok(self.native.Caption()?.to_string_lossy())
    }

    pub fn set_caption(&mut self, caption: impl AsRef<str>) -> Result<()> {
        self.native.SetCaption(&HSTRING::from(caption.as_ref()))?;
        Ok(())
    }

    pub fn value(&self) -> Result<f64> {
        self.native.Value()
    }

    pub fn set_value(&mut self, value: f64) -> Result<()> {
        self.native.SetValue2(value)?;
        Ok(())
    }

    pub fn max_rating(&self) -> Result<i32> {
        self.native.MaxRating()
    }

    pub fn set_max_rating(&mut self, value: i32) -> Result<()> {
        self.native.SetMaxRating(value)?;
        Ok(())
    }

    pub fn is_read_only(&self) -> Result<bool> {
        self.native.IsReadOnly()
    }

    pub fn set_is_read_only(&mut self, value: bool) -> Result<()> {
        self.native.SetIsReadOnly(value)?;
        Ok(())
    }

    pub fn is_clear_enabled(&self) -> Result<bool> {
        self.native.IsClearEnabled()
    }

    pub fn set_is_clear_enabled(&mut self, value: bool) -> Result<()> {
        self.native.SetIsClearEnabled(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for RatingControl {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for RatingControl {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for RatingControl {
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum RatingControlEvent {
    ValueChanged(f64),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RatingControlMessage {}

impl Component for RatingControl {
    type Error = Error;
    type Event = RatingControlEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = RatingControlMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeRatingControl::new()?;

        let event_sender = sender.clone();
        let value_changed_token = native.ValueChanged(&TypedEventHandler::<
            NativeRatingControl,
            IInspectable,
        >::new(move |native, _| {
            if let Some(native) = native.as_ref()
                && let Ok(value) = native.Value()
            {
                event_sender.output(RatingControlEvent::ValueChanged(value));
            }
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            value_changed_token,
        })
    }
}

impl AsWidget for RatingControl {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for RatingControl {
    fn drop(&mut self) {
        let _ = self.native.RemoveValueChanged(self.value_changed_token);
    }
}
