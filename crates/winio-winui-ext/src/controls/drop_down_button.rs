use windows::Foundation::EventHandler;
use windows::core::{HRESULT, HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, TextWidget, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::FlyoutBase;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    DropDownButton as NativeDropDownButton, Flyout,
};
use winio_winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

type SetButtonFlyoutFn =
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> HRESULT;

/// winio-winui3 0.4.4 projects Microsoft.UI.Xaml.Controls.IButton but cannot
/// expose its Flyout accessors because its projection omits FlyoutBase.
/// The WinRT ABI is still present: after the six IInspectable slots,
/// IButton::Flyout and IButton::SetFlyout are slots 6 and 7.
///
/// Keep this bridge private and narrowly scoped until the upstream projection
/// can express the property directly.
fn set_button_flyout(button: &MUXC::Button, flyout: &FlyoutBase) -> Result<()> {
    const SET_FLYOUT_SLOT: usize = 7;

    let this = button.as_raw();
    let vtable = unsafe {
        // SAFETY: Button is a WinRT class whose default interface is IButton;
        // COM interface pointers begin with a vtable pointer.
        *(this as *const *const *const core::ffi::c_void)
    };
    let set_flyout: SetButtonFlyoutFn = unsafe {
        // SAFETY: WinRT interface slot order is ABI-stable. Slot 7 is
        // IButton::SetFlyout for the metadata IID projected by winio-winui3.
        core::mem::transmute(*vtable.add(SET_FLYOUT_SLOT))
    };
    unsafe {
        // SAFETY: both interface pointers remain alive for the duration of the
        // call; WinRT property assignment retains the flyout as needed.
        set_flyout(this, flyout.as_raw()).ok()?;
    }
    Ok(())
}

/// A native WinUI DropDownButton with an owned flyout content surface.
#[derive(Debug)]
pub struct DropDownButton {
    button: MUXC::Button,
    element: NativeElement,
    text: MUXC::TextBlock,
    flyout_base: FlyoutBase,
    flyout_content: MUXC::Canvas,
    click_token: i64,
    opened_token: i64,
    closed_token: i64,
}

impl Failable for DropDownButton {
    type Error = Error;
}

impl DropDownButton {
    /// Returns the flyout content Canvas so Winio children can be created under
    /// their final native owner from construction time.
    pub fn flyout_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.flyout_content)
    }

    pub fn flyout_content_size(&self) -> Result<Size> {
        Ok(Size::new(
            self.flyout_content.Width()?,
            self.flyout_content.Height()?,
        ))
    }

    pub fn set_flyout_content_size(&mut self, size: Size) -> Result<()> {
        self.flyout_content.SetWidth(size.width)?;
        self.flyout_content.SetHeight(size.height)?;
        Ok(())
    }

    pub fn hide_flyout(&self) -> Result<()> {
        self.flyout_base.Hide()?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl TextWidget for DropDownButton {
    fn text(&self) -> Result<String> {
        Ok(self.text.Text()?.to_string_lossy())
    }

    fn set_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.text.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }
}

impl Visible for DropDownButton {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for DropDownButton {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for DropDownButton {
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
pub enum DropDownButtonEvent {
    Click,
    FlyoutOpened,
    FlyoutClosed,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DropDownButtonMessage {}

impl Component for DropDownButton {
    type Error = Error;
    type Event = DropDownButtonEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = DropDownButtonMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeDropDownButton::new()?;
        let button = native.cast::<MUXC::Button>()?;
        let text = MUXC::TextBlock::new()?;
        button.cast::<MUXC::ContentControl>()?.SetContent(&text)?;

        let flyout = Flyout::new()?;
        let flyout_content = MUXC::Canvas::new()?;
        flyout_content.SetWidth(260.0)?;
        flyout_content.SetHeight(72.0)?;
        flyout.SetContent(&flyout_content)?;
        let flyout_base = flyout.cast::<FlyoutBase>()?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        set_button_flyout(&button, &flyout_base)?;

        let click_sender = sender.clone();
        let click_token = button.Click(&RoutedEventHandler::new(move |_, _| {
            click_sender.output(DropDownButtonEvent::Click);
            Ok(())
        }))?;

        let opened_sender = sender.clone();
        let opened_token =
            flyout_base.Opened(&EventHandler::<IInspectable>::new(move |_, _| {
                opened_sender.output(DropDownButtonEvent::FlyoutOpened);
                Ok(())
            }))?;

        let closed_sender = sender.clone();
        let closed_token =
            flyout_base.Closed(&EventHandler::<IInspectable>::new(move |_, _| {
                closed_sender.output(DropDownButtonEvent::FlyoutClosed);
                Ok(())
            }))?;

        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            button,
            element,
            text,
            flyout_base,
            flyout_content,
            click_token,
            opened_token,
            closed_token,
        })
    }
}

impl AsWidget for DropDownButton {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for DropDownButton {
    fn drop(&mut self) {
        let _ = self.button.RemoveClick(self.click_token);
        let _ = self.flyout_base.RemoveOpened(self.opened_token);
        let _ = self.flyout_base.RemoveClosed(self.closed_token);
    }
}
