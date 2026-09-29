use windows::core::{HSTRING, Interface, Type};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winui3::Microsoft::UI::Xaml::Controls::Primitives as MUXCP;
use winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

/// A native WinUI `ToggleButton` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct ToggleButton {
    native: MUXCP::ToggleButton,
    element: NativeElement,
    text: MUXC::TextBlock,
    checked_token: i64,
    unchecked_token: i64,
    indeterminate_token: i64,
}

impl Failable for ToggleButton {
    type Error = Error;
}

impl TextWidget for ToggleButton {
    fn text(&self) -> Result<String> {
        Ok(self.text.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.text.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for ToggleButton {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for ToggleButton {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for ToggleButton {
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

impl ToggleButton {
    fn checked_reference(&self) -> Result<Option<windows::Foundation::IReference<bool>>> {
        // `winio-winui3 0.4.4` projects `IsChecked` as a non-optional
        // `IReference<bool>`, so its generated getter turns a legitimate null
        // WinRT reference into `Error::empty()`. Read the ABI directly to
        // preserve the nullable value without changing the projection crate.
        let this = self.native.cast::<MUXCP::IToggleButton>()?;
        unsafe {
            let mut raw = std::ptr::null_mut();
            (Interface::vtable(&this).IsChecked)(Interface::as_raw(&this), &mut raw).ok()?;
            if raw.is_null() {
                Ok(None)
            } else {
                Ok(Some(Type::from_abi(raw)?))
            }
        }
    }

    /// Returns the native nullable checked state. `None` means indeterminate.
    pub fn is_checked(&self) -> Result<Option<bool>> {
        self.checked_reference()?
            .map(|checked| checked.GetBoolean())
            .transpose()
    }

    /// Sets the native nullable checked state. `None` means indeterminate.
    ///
    /// WinUI defaults `IsThreeState` to `false`; call [`Self::set_three_state`]
    /// when user interaction should cycle through the indeterminate state.
    pub fn set_checked(&mut self, checked: Option<bool>) -> Result<()> {
        match checked {
            Some(value) => {
                let reference: windows::Foundation::IReference<bool> =
                    windows::Foundation::PropertyValue::CreateBoolean(value)?.cast()?;
                self.native.SetIsChecked(&reference)?;
            }
            None => self
                .native
                .SetIsChecked(None::<&windows::Foundation::IReference<bool>>)?,
        }
        Ok(())
    }

    /// Returns whether pointer/keyboard interaction cycles through three states.
    pub fn is_three_state(&self) -> Result<bool> {
        self.native.IsThreeState()
    }

    /// Enables or disables WinUI's three-state interaction mode.
    pub fn set_three_state(&mut self, three_state: bool) -> Result<()> {
        self.native.SetIsThreeState(three_state)?;
        Ok(())
    }

    /// Measures the native control with unbounded available space.
    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    /// Returns the native control's minimum width and height.
    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

/// Events emitted by [`ToggleButton`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ToggleButtonEvent {
    /// The nullable checked state changed: `Some(true)` is checked,
    /// `Some(false)` is unchecked, and `None` is indeterminate.
    StateChanged(Option<bool>),
}

/// Input messages for [`ToggleButton`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ToggleButtonMessage {}

impl Component for ToggleButton {
    type Error = Error;
    type Event = ToggleButtonEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ToggleButtonMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = MUXCP::ToggleButton::new()?;
        let text = MUXC::TextBlock::new()?;
        native.SetContent(&text)?;

        let checked_sender = sender.clone();
        let checked_token = native.Checked(&RoutedEventHandler::new(move |_, _| {
            checked_sender.output(ToggleButtonEvent::StateChanged(Some(true)));
            Ok(())
        }))?;
        let unchecked_sender = sender.clone();
        let unchecked_token = native.Unchecked(&RoutedEventHandler::new(move |_, _| {
            unchecked_sender.output(ToggleButtonEvent::StateChanged(Some(false)));
            Ok(())
        }))?;
        let indeterminate_sender = sender.clone();
        let indeterminate_token = native.Indeterminate(&RoutedEventHandler::new(move |_, _| {
            indeterminate_sender.output(ToggleButtonEvent::StateChanged(None));
            Ok(())
        }))?;

        let element = NativeElement::new(parent, native.cast()?)?;

        Ok(Self {
            native,
            element,
            text,
            checked_token,
            unchecked_token,
            indeterminate_token,
        })
    }
}

impl AsWidget for ToggleButton {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for ToggleButton {
    fn drop(&mut self) {
        let _ = self.native.RemoveChecked(self.checked_token);
        let _ = self.native.RemoveUnchecked(self.unchecked_token);
        let _ = self.native.RemoveIndeterminate(self.indeterminate_token);
    }
}
