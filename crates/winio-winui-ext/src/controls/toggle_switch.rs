use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::ToggleSwitch as NativeToggleSwitch;
use winio_winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

/// A native WinUI ToggleSwitch hosted as a first-class Winio component.
#[derive(Debug)]
pub struct ToggleSwitch {
    native: NativeToggleSwitch,
    element: NativeElement,
    header: MUXC::TextBlock,
    toggled_token: i64,
}

impl Failable for ToggleSwitch {
    type Error = Error;
}

impl ToggleSwitch {
    pub fn is_on(&self) -> Result<bool> {
        self.native.IsOn()
    }

    pub fn set_is_on(&mut self, is_on: bool) -> Result<()> {
        self.native.SetIsOn(is_on)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl TextWidget for ToggleSwitch {
    fn text(&self) -> Result<String> {
        Ok(self.header.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.header.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for ToggleSwitch {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for ToggleSwitch {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for ToggleSwitch {
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
pub enum ToggleSwitchEvent {
    Toggled(bool),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToggleSwitchMessage {}

impl Component for ToggleSwitch {
    type Error = Error;
    type Event = ToggleSwitchEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ToggleSwitchMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeToggleSwitch::new()?;
        let header = MUXC::TextBlock::new()?;
        native.SetHeader(&header)?;

        let event_sender = sender.clone();
        let toggled_token = native.Toggled(&RoutedEventHandler::new(move |native, _| {
            if let Some(native) = native.as_ref()
                && let Ok(native) = native.cast::<NativeToggleSwitch>()
                && let Ok(is_on) = native.IsOn()
            {
                event_sender.output(ToggleSwitchEvent::Toggled(is_on));
            }
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            header,
            toggled_token,
        })
    }
}

impl AsWidget for ToggleSwitch {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for ToggleSwitch {
    fn drop(&mut self) {
        let _ = self.native.RemoveToggled(self.toggled_token);
    }
}
