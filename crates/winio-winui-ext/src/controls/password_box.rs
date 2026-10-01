use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

/// A native WinUI PasswordBox hosted as a first-class Winio component.
#[derive(Debug)]
pub struct PasswordBox {
    native: MUXC::PasswordBox,
    element: NativeElement,
    header: MUXC::TextBlock,
    password_changed_token: i64,
}

impl Failable for PasswordBox {
    type Error = Error;
}

impl PasswordBox {
    pub fn password(&self) -> Result<String> {
        Ok(self.native.Password()?.to_string_lossy())
    }

    pub fn set_password(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetPassword(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn password_char(&self) -> Result<String> {
        Ok(self.native.PasswordChar()?.to_string_lossy())
    }

    pub fn set_password_char(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .SetPasswordChar(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn max_length(&self) -> Result<i32> {
        self.native.MaxLength()
    }

    pub fn set_max_length(&mut self, value: i32) -> Result<()> {
        self.native.SetMaxLength(value)?;
        Ok(())
    }

    pub fn header(&self) -> Result<String> {
        Ok(self.header.Text()?.to_string_lossy())
    }

    pub fn set_header(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.header.SetText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn placeholder_text(&self) -> Result<String> {
        Ok(self.native.PlaceholderText()?.to_string_lossy())
    }

    pub fn set_placeholder_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .SetPlaceholderText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn is_password_reveal_button_enabled(&self) -> Result<bool> {
        self.native.IsPasswordRevealButtonEnabled()
    }

    pub fn set_is_password_reveal_button_enabled(&mut self, enabled: bool) -> Result<()> {
        self.native.SetIsPasswordRevealButtonEnabled(enabled)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for PasswordBox {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for PasswordBox {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for PasswordBox {
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
pub enum PasswordBoxEvent {
    PasswordChanged,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PasswordBoxMessage {}

impl Component for PasswordBox {
    type Error = Error;
    type Event = PasswordBoxEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = PasswordBoxMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = MUXC::PasswordBox::new()?;
        let header = MUXC::TextBlock::new()?;
        native.SetHeader(&header)?;

        let event_sender = sender.clone();
        let password_changed_token =
            native.PasswordChanged(&RoutedEventHandler::new(move |_, _| {
                event_sender.output(PasswordBoxEvent::PasswordChanged);
                Ok(())
            }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            header,
            password_changed_token,
        })
    }
}

impl AsWidget for PasswordBox {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for PasswordBox {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemovePasswordChanged(self.password_changed_token);
    }
}
