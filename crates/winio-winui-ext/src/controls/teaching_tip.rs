use windows::Foundation::TypedEventHandler;
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    TeachingTip as NativeTeachingTip, TeachingTipCloseReason as NativeCloseReason,
    TeachingTipClosedEventArgs, TeachingTipPlacementMode,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TeachingTipPlacement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    Center,
}

impl TeachingTipPlacement {
    fn native(self) -> TeachingTipPlacementMode {
        match self {
            Self::Auto => TeachingTipPlacementMode::Auto,
            Self::Top => TeachingTipPlacementMode::Top,
            Self::Bottom => TeachingTipPlacementMode::Bottom,
            Self::Left => TeachingTipPlacementMode::Left,
            Self::Right => TeachingTipPlacementMode::Right,
            Self::TopRight => TeachingTipPlacementMode::TopRight,
            Self::TopLeft => TeachingTipPlacementMode::TopLeft,
            Self::BottomRight => TeachingTipPlacementMode::BottomRight,
            Self::BottomLeft => TeachingTipPlacementMode::BottomLeft,
            Self::LeftTop => TeachingTipPlacementMode::LeftTop,
            Self::LeftBottom => TeachingTipPlacementMode::LeftBottom,
            Self::RightTop => TeachingTipPlacementMode::RightTop,
            Self::RightBottom => TeachingTipPlacementMode::RightBottom,
            Self::Center => TeachingTipPlacementMode::Center,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TeachingTipCloseReason {
    CloseButton,
    LightDismiss,
    Programmatic,
    Unknown(i32),
}

impl From<NativeCloseReason> for TeachingTipCloseReason {
    fn from(reason: NativeCloseReason) -> Self {
        if reason == NativeCloseReason::CloseButton {
            Self::CloseButton
        } else if reason == NativeCloseReason::LightDismiss {
            Self::LightDismiss
        } else if reason == NativeCloseReason::Programmatic {
            Self::Programmatic
        } else {
            Self::Unknown(reason.0)
        }
    }
}

#[derive(Debug)]
pub struct TeachingTip {
    native: NativeTeachingTip,
    element: NativeElement,
    closed_token: i64,
}

impl Failable for TeachingTip {
    type Error = Error;
}

impl TeachingTip {
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

    pub fn is_open(&self) -> Result<bool> {
        self.native.IsOpen()
    }

    pub fn set_is_open(&mut self, is_open: bool) -> Result<()> {
        self.native.SetIsOpen(is_open)?;
        Ok(())
    }

    pub fn set_target(&mut self, target: BorrowedWidget<'_>) -> Result<()> {
        self.native.SetTarget(target.as_winui())?;
        Ok(())
    }

    pub fn set_preferred_placement(&mut self, placement: TeachingTipPlacement) -> Result<()> {
        self.native.SetPreferredPlacement(placement.native())?;
        Ok(())
    }

    pub fn set_close_button_content(&mut self, text: impl AsRef<str>) -> Result<()> {
        let content = MUXC::TextBlock::new()?;
        content.SetText(&HSTRING::from(text.as_ref()))?;
        self.native.SetCloseButtonContent(&content)?;
        Ok(())
    }
}

impl Visible for TeachingTip {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for TeachingTip {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for TeachingTip {
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
pub enum TeachingTipEvent {
    Closed { reason: TeachingTipCloseReason },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TeachingTipMessage {}

impl Component for TeachingTip {
    type Error = Error;
    type Event = TeachingTipEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = TeachingTipMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeTeachingTip::new()?;
        let closed_sender = sender.clone();
        let closed_token = native.Closed(&TypedEventHandler::<
            NativeTeachingTip,
            TeachingTipClosedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let Ok(reason) = args.Reason()
            {
                closed_sender.output(TeachingTipEvent::Closed {
                    reason: reason.into(),
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
            closed_token,
        })
    }
}

impl AsWidget for TeachingTip {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for TeachingTip {
    fn drop(&mut self) {
        let _ = self.native.RemoveClosed(self.closed_token);
    }
}
