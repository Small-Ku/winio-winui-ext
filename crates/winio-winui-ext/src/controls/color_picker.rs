use windows::Foundation::TypedEventHandler;
use windows::UI::Color as NativeColor;
use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    ColorChangedEventArgs, ColorPicker as NativeColorPicker,
};

use crate::native::NativeElement;

/// An RGBA color value used by the ColorPicker wrapper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorValue {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorValue {
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(255, r, g, b)
    }
}

impl From<NativeColor> for ColorValue {
    fn from(value: NativeColor) -> Self {
        Self::from_argb(value.A, value.R, value.G, value.B)
    }
}

impl From<ColorValue> for NativeColor {
    fn from(value: ColorValue) -> Self {
        Self {
            A: value.a,
            R: value.r,
            G: value.g,
            B: value.b,
        }
    }
}

/// A native WinUI ColorPicker hosted as a first-class Winio component.
#[derive(Debug)]
pub struct ColorPicker {
    native: NativeColorPicker,
    element: NativeElement,
    color_changed_token: i64,
}

impl Failable for ColorPicker {
    type Error = Error;
}

impl ColorPicker {
    pub fn color(&self) -> Result<ColorValue> {
        Ok(self.native.Color()?.into())
    }

    pub fn set_color(&mut self, color: ColorValue) -> Result<()> {
        self.native.SetColor(color.into())?;
        Ok(())
    }

    pub fn is_alpha_enabled(&self) -> Result<bool> {
        self.native.IsAlphaEnabled()
    }

    pub fn set_is_alpha_enabled(&mut self, enabled: bool) -> Result<()> {
        self.native.SetIsAlphaEnabled(enabled)?;
        Ok(())
    }

    pub fn is_color_spectrum_visible(&self) -> Result<bool> {
        self.native.IsColorSpectrumVisible()
    }

    pub fn set_is_color_spectrum_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetIsColorSpectrumVisible(visible)?;
        Ok(())
    }

    pub fn is_color_slider_visible(&self) -> Result<bool> {
        self.native.IsColorSliderVisible()
    }

    pub fn set_is_color_slider_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetIsColorSliderVisible(visible)?;
        Ok(())
    }

    pub fn is_color_channel_text_input_visible(&self) -> Result<bool> {
        self.native.IsColorChannelTextInputVisible()
    }

    pub fn set_is_color_channel_text_input_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetIsColorChannelTextInputVisible(visible)?;
        Ok(())
    }

    pub fn is_hex_input_visible(&self) -> Result<bool> {
        self.native.IsHexInputVisible()
    }

    pub fn set_is_hex_input_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetIsHexInputVisible(visible)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for ColorPicker {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for ColorPicker {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for ColorPicker {
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
pub enum ColorPickerEvent {
    ColorChanged { old: ColorValue, new: ColorValue },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ColorPickerMessage {}

impl Component for ColorPicker {
    type Error = Error;
    type Event = ColorPickerEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ColorPickerMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeColorPicker::new()?;

        let event_sender = sender.clone();
        let color_changed_token = native.ColorChanged(&TypedEventHandler::<
            NativeColorPicker,
            ColorChangedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let (Ok(old), Ok(new)) = (args.OldColor(), args.NewColor())
            {
                event_sender.output(ColorPickerEvent::ColorChanged {
                    old: old.into(),
                    new: new.into(),
                });
            }
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            color_changed_token,
        })
    }
}

impl AsWidget for ColorPicker {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for ColorPicker {
    fn drop(&mut self) {
        let _ = self.native.RemoveColorChanged(self.color_changed_token);
    }
}
