use std::fmt::Debug;

use windows::core::{Interface, Weak};
use winio::handle::{AsContainer, AsWidget, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winui3::Microsoft::UI::Xaml::{self as MUX, Controls as MUXC};

/// Owns a native WinUI `FrameworkElement` hosted inside a Winio `Canvas`.
///
/// This is the interoperability boundary for extension controls. It mirrors
/// Winio's WinUI backend ownership rules without depending on its private
/// `Widget` implementation.
pub(crate) struct NativeElement {
    handle: MUX::FrameworkElement,
    parent: Weak<MUXC::Canvas>,
}

impl NativeElement {
    pub(crate) fn new(parent: impl AsContainer, handle: MUX::FrameworkElement) -> Result<Self> {
        handle.SetHorizontalAlignment(MUX::HorizontalAlignment::Center)?;
        handle.SetVerticalAlignment(MUX::VerticalAlignment::Center)?;

        let parent = parent.as_container();
        let canvas = parent.as_winui();
        canvas.Children()?.Append(&handle)?;

        Ok(Self {
            handle,
            parent: canvas.downgrade()?,
        })
    }
}

impl Failable for NativeElement {
    type Error = Error;
}

impl Visible for NativeElement {
    fn is_visible(&self) -> Result<bool> {
        Ok(self.handle.Visibility()? == MUX::Visibility::Visible)
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.handle.SetVisibility(if visible {
            MUX::Visibility::Visible
        } else {
            MUX::Visibility::Collapsed
        })?;
        Ok(())
    }
}

impl Enable for NativeElement {
    fn is_enabled(&self) -> Result<bool> {
        match self.handle.cast::<MUXC::Control>() {
            Ok(control) => control.IsEnabled(),
            Err(_) => Ok(true),
        }
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        if let Ok(control) = self.handle.cast::<MUXC::Control>() {
            control.SetIsEnabled(enabled)?;
        }
        Ok(())
    }
}

impl Layoutable for NativeElement {
    fn loc(&self) -> Result<Point> {
        Ok(Point::new(
            MUXC::Canvas::GetLeft(&self.handle)?,
            MUXC::Canvas::GetTop(&self.handle)?,
        ))
    }

    fn set_loc(&mut self, point: Point) -> Result<()> {
        MUXC::Canvas::SetLeft(&self.handle, point.x)?;
        MUXC::Canvas::SetTop(&self.handle, point.y)?;
        Ok(())
    }

    fn size(&self) -> Result<Size> {
        Ok(Size::new(self.handle.Width()?, self.handle.Height()?))
    }

    fn set_size(&mut self, size: Size) -> Result<()> {
        self.handle.SetWidth(size.width)?;
        self.handle.SetHeight(size.height)?;
        Ok(())
    }
}

impl AsWidget for NativeElement {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        BorrowedWidget::winui(&self.handle)
    }
}

impl Drop for NativeElement {
    fn drop(&mut self) {
        let Some(parent) = self.parent.upgrade() else {
            return;
        };
        let Ok(children) = parent.Children() else {
            return;
        };

        let mut index = 0;
        if children.IndexOf(&self.handle, &mut index).is_ok() {
            let _ = children.RemoveAt(index);
        }
    }
}

impl Debug for NativeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeElement")
            .field("handle", &self.handle)
            .finish()
    }
}
