use windows::Foundation::TypedEventHandler;
use windows::core::{IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    SplitView as NativeSplitView, SplitViewDisplayMode as NativeDisplayMode,
    SplitViewPaneClosingEventArgs, SplitViewPanePlacement as NativePanePlacement,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// Native SplitView display behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SplitViewDisplayMode {
    Overlay,
    Inline,
    CompactOverlay,
    CompactInline,
    Unknown(i32),
}

impl From<NativeDisplayMode> for SplitViewDisplayMode {
    fn from(value: NativeDisplayMode) -> Self {
        match value.0 {
            0 => Self::Overlay,
            1 => Self::Inline,
            2 => Self::CompactOverlay,
            3 => Self::CompactInline,
            value => Self::Unknown(value),
        }
    }
}

impl From<SplitViewDisplayMode> for NativeDisplayMode {
    fn from(value: SplitViewDisplayMode) -> Self {
        Self(match value {
            SplitViewDisplayMode::Overlay => 0,
            SplitViewDisplayMode::Inline => 1,
            SplitViewDisplayMode::CompactOverlay => 2,
            SplitViewDisplayMode::CompactInline => 3,
            SplitViewDisplayMode::Unknown(value) => value,
        })
    }
}

/// Side on which the native pane is placed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SplitViewPanePlacement {
    Left,
    Right,
    Unknown(i32),
}

impl From<NativePanePlacement> for SplitViewPanePlacement {
    fn from(value: NativePanePlacement) -> Self {
        match value.0 {
            0 => Self::Left,
            1 => Self::Right,
            value => Self::Unknown(value),
        }
    }
}

impl From<SplitViewPanePlacement> for NativePanePlacement {
    fn from(value: SplitViewPanePlacement) -> Self {
        Self(match value {
            SplitViewPanePlacement::Left => 0,
            SplitViewPanePlacement::Right => 1,
            SplitViewPanePlacement::Unknown(value) => value,
        })
    }
}

/// A native two-region pane/content control that owns both Winio child hosts.
#[derive(Debug)]
pub struct SplitView {
    native: NativeSplitView,
    element: NativeElement,
    pane: MUXC::Canvas,
    content: MUXC::Canvas,
    opening_token: i64,
    opened_token: i64,
    closing_token: i64,
    closed_token: i64,
}

impl Failable for SplitView {
    type Error = Error;
}

impl SplitView {
    /// Container for Winio widgets hosted inside the native pane.
    pub fn pane_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.pane)
    }

    /// Container for Winio widgets hosted inside the native main content.
    pub fn content_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.content)
    }

    pub fn is_pane_open(&self) -> Result<bool> {
        self.native.IsPaneOpen()
    }

    pub fn set_is_pane_open(&mut self, open: bool) -> Result<()> {
        self.native.SetIsPaneOpen(open)?;
        Ok(())
    }

    pub fn display_mode(&self) -> Result<SplitViewDisplayMode> {
        Ok(self.native.DisplayMode()?.into())
    }

    pub fn set_display_mode(&mut self, mode: SplitViewDisplayMode) -> Result<()> {
        self.native.SetDisplayMode(mode.into())?;
        Ok(())
    }

    pub fn pane_placement(&self) -> Result<SplitViewPanePlacement> {
        Ok(self.native.PanePlacement()?.into())
    }

    pub fn set_pane_placement(&mut self, placement: SplitViewPanePlacement) -> Result<()> {
        self.native.SetPanePlacement(placement.into())?;
        Ok(())
    }

    pub fn open_pane_length(&self) -> Result<f64> {
        self.native.OpenPaneLength()
    }

    pub fn set_open_pane_length(&mut self, length: f64) -> Result<()> {
        self.native.SetOpenPaneLength(length)?;
        Ok(())
    }

    pub fn compact_pane_length(&self) -> Result<f64> {
        self.native.CompactPaneLength()
    }

    pub fn set_compact_pane_length(&mut self, length: f64) -> Result<()> {
        self.native.SetCompactPaneLength(length)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for SplitView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, value: bool) -> Result<()> {
        self.element.set_visible(value)
    }
}

impl Enable for SplitView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, value: bool) -> Result<()> {
        self.element.set_enabled(value)
    }
}

impl Layoutable for SplitView {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }
    fn set_loc(&mut self, value: Point) -> Result<()> {
        self.element.set_loc(value)
    }
    fn size(&self) -> Result<Size> {
        self.element.size()
    }
    fn set_size(&mut self, value: Size) -> Result<()> {
        self.element.set_size(value)
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
pub enum SplitViewEvent {
    PaneOpening,
    PaneOpened,
    PaneClosing { cancelled: bool },
    PaneClosed,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SplitViewMessage {}

impl Component for SplitView {
    type Error = Error;
    type Event = SplitViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = SplitViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeSplitView::new()?;
        let pane = MUXC::Canvas::new()?;
        let content = MUXC::Canvas::new()?;
        native.SetPane(&pane)?;
        native.SetContent(&content)?;

        let opening_sender = sender.clone();
        let opening_token = native.PaneOpening(&TypedEventHandler::<
            NativeSplitView,
            IInspectable,
        >::new(move |_, _| {
            opening_sender.output(SplitViewEvent::PaneOpening);
            Ok(())
        }))?;
        let opened_sender = sender.clone();
        let opened_token = native.PaneOpened(
            &TypedEventHandler::<NativeSplitView, IInspectable>::new(move |_, _| {
                opened_sender.output(SplitViewEvent::PaneOpened);
                Ok(())
            }),
        )?;
        let closing_sender = sender.clone();
        let closing_token = native.PaneClosing(&TypedEventHandler::<
            NativeSplitView,
            SplitViewPaneClosingEventArgs,
        >::new(move |_, args| {
            let cancelled = args
                .as_ref()
                .map(|args| args.Cancel())
                .transpose()?
                .unwrap_or(false);
            closing_sender.output(SplitViewEvent::PaneClosing { cancelled });
            Ok(())
        }))?;
        let closed_sender = sender.clone();
        let closed_token = native.PaneClosed(
            &TypedEventHandler::<NativeSplitView, IInspectable>::new(move |_, _| {
                closed_sender.output(SplitViewEvent::PaneClosed);
                Ok(())
            }),
        )?;

        let element = NativeElement::new(
            parent,
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?,
        )?;
        Ok(Self {
            native,
            element,
            pane,
            content,
            opening_token,
            opened_token,
            closing_token,
            closed_token,
        })
    }
}

impl AsWidget for SplitView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for SplitView {
    fn drop(&mut self) {
        let _ = self.native.RemovePaneOpening(self.opening_token);
        let _ = self.native.RemovePaneOpened(self.opened_token);
        let _ = self.native.RemovePaneClosing(self.closing_token);
        let _ = self.native.RemovePaneClosed(self.closed_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_mode_mapping_preserves_known_and_unknown_values() {
        for (native, public) in [
            (NativeDisplayMode::Overlay, SplitViewDisplayMode::Overlay),
            (NativeDisplayMode::Inline, SplitViewDisplayMode::Inline),
            (
                NativeDisplayMode::CompactOverlay,
                SplitViewDisplayMode::CompactOverlay,
            ),
            (
                NativeDisplayMode::CompactInline,
                SplitViewDisplayMode::CompactInline,
            ),
        ] {
            assert_eq!(SplitViewDisplayMode::from(native), public);
            assert_eq!(NativeDisplayMode::from(public), native);
        }
        assert_eq!(
            SplitViewDisplayMode::from(NativeDisplayMode(41)),
            SplitViewDisplayMode::Unknown(41)
        );
        assert_eq!(
            NativeDisplayMode::from(SplitViewDisplayMode::Unknown(41)),
            NativeDisplayMode(41)
        );
    }

    #[test]
    fn pane_placement_mapping_preserves_known_and_unknown_values() {
        for (native, public) in [
            (NativePanePlacement::Left, SplitViewPanePlacement::Left),
            (NativePanePlacement::Right, SplitViewPanePlacement::Right),
        ] {
            assert_eq!(SplitViewPanePlacement::from(native), public);
            assert_eq!(NativePanePlacement::from(public), native);
        }
        assert_eq!(
            SplitViewPanePlacement::from(NativePanePlacement(41)),
            SplitViewPanePlacement::Unknown(41)
        );
        assert_eq!(
            NativePanePlacement::from(SplitViewPanePlacement::Unknown(41)),
            NativePanePlacement(41)
        );
    }
}
