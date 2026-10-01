use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    TwoPaneView as NativeTwoPaneView, TwoPaneViewMode as NativeMode,
    TwoPaneViewPriority as NativePriority, TwoPaneViewTallModeConfiguration as NativeTall,
    TwoPaneViewWideModeConfiguration as NativeWide,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// The adaptive layout mode selected by the native TwoPaneView.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TwoPaneViewMode {
    SinglePane,
    Wide,
    Tall,
    Unknown(i32),
}

impl From<NativeMode> for TwoPaneViewMode {
    fn from(value: NativeMode) -> Self {
        match value.0 {
            0 => Self::SinglePane,
            1 => Self::Wide,
            2 => Self::Tall,
            value => Self::Unknown(value),
        }
    }
}

/// Chooses which pane remains visible when the control enters single-pane mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TwoPaneViewPriority {
    Pane1,
    Pane2,
    Unknown(i32),
}
impl From<NativePriority> for TwoPaneViewPriority {
    fn from(value: NativePriority) -> Self {
        match value.0 {
            0 => Self::Pane1,
            1 => Self::Pane2,
            value => Self::Unknown(value),
        }
    }
}
impl From<TwoPaneViewPriority> for NativePriority {
    fn from(value: TwoPaneViewPriority) -> Self {
        Self(match value {
            TwoPaneViewPriority::Pane1 => 0,
            TwoPaneViewPriority::Pane2 => 1,
            TwoPaneViewPriority::Unknown(value) => value,
        })
    }
}

/// Arrangement of the panes while the view is in wide mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TwoPaneViewWideModeConfiguration {
    SinglePane,
    LeftRight,
    RightLeft,
    Unknown(i32),
}
impl From<NativeWide> for TwoPaneViewWideModeConfiguration {
    fn from(value: NativeWide) -> Self {
        match value.0 {
            0 => Self::SinglePane,
            1 => Self::LeftRight,
            2 => Self::RightLeft,
            value => Self::Unknown(value),
        }
    }
}
impl From<TwoPaneViewWideModeConfiguration> for NativeWide {
    fn from(value: TwoPaneViewWideModeConfiguration) -> Self {
        Self(match value {
            TwoPaneViewWideModeConfiguration::SinglePane => 0,
            TwoPaneViewWideModeConfiguration::LeftRight => 1,
            TwoPaneViewWideModeConfiguration::RightLeft => 2,
            TwoPaneViewWideModeConfiguration::Unknown(value) => value,
        })
    }
}

/// Arrangement of the panes while the view is in tall mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TwoPaneViewTallModeConfiguration {
    SinglePane,
    TopBottom,
    BottomTop,
    Unknown(i32),
}
impl From<NativeTall> for TwoPaneViewTallModeConfiguration {
    fn from(value: NativeTall) -> Self {
        match value.0 {
            0 => Self::SinglePane,
            1 => Self::TopBottom,
            2 => Self::BottomTop,
            value => Self::Unknown(value),
        }
    }
}
impl From<TwoPaneViewTallModeConfiguration> for NativeTall {
    fn from(value: TwoPaneViewTallModeConfiguration) -> Self {
        Self(match value {
            TwoPaneViewTallModeConfiguration::SinglePane => 0,
            TwoPaneViewTallModeConfiguration::TopBottom => 1,
            TwoPaneViewTallModeConfiguration::BottomTop => 2,
            TwoPaneViewTallModeConfiguration::Unknown(value) => value,
        })
    }
}

/// A native TwoPaneView that owns one Winio-compatible Canvas per pane.
#[derive(Debug)]
pub struct TwoPaneView {
    native: NativeTwoPaneView,
    element: NativeElement,
    pane1: MUXC::Canvas,
    pane2: MUXC::Canvas,
}

impl Failable for TwoPaneView {
    type Error = Error;
}

impl TwoPaneView {
    /// Container for Winio children that belong to pane 1.
    pub fn pane1_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.pane1)
    }

    /// Container for Winio children that belong to pane 2.
    pub fn pane2_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.pane2)
    }

    pub fn mode(&self) -> Result<TwoPaneViewMode> {
        Ok(self.native.Mode()?.into())
    }
    pub fn priority(&self) -> Result<TwoPaneViewPriority> {
        Ok(self.native.PanePriority()?.into())
    }
    pub fn set_priority(&mut self, value: TwoPaneViewPriority) -> Result<()> {
        self.native.SetPanePriority(value.into())?;
        Ok(())
    }
    pub fn wide_mode_configuration(&self) -> Result<TwoPaneViewWideModeConfiguration> {
        Ok(self.native.WideModeConfiguration()?.into())
    }
    pub fn set_wide_mode_configuration(
        &mut self,
        value: TwoPaneViewWideModeConfiguration,
    ) -> Result<()> {
        self.native.SetWideModeConfiguration(value.into())?;
        Ok(())
    }
    pub fn tall_mode_configuration(&self) -> Result<TwoPaneViewTallModeConfiguration> {
        Ok(self.native.TallModeConfiguration()?.into())
    }
    pub fn set_tall_mode_configuration(
        &mut self,
        value: TwoPaneViewTallModeConfiguration,
    ) -> Result<()> {
        self.native.SetTallModeConfiguration(value.into())?;
        Ok(())
    }
    pub fn min_wide_mode_width(&self) -> Result<f64> {
        self.native.MinWideModeWidth()
    }
    pub fn set_min_wide_mode_width(&mut self, value: f64) -> Result<()> {
        self.native.SetMinWideModeWidth(value)?;
        Ok(())
    }
    pub fn min_tall_mode_height(&self) -> Result<f64> {
        self.native.MinTallModeHeight()
    }
    pub fn set_min_tall_mode_height(&mut self, value: f64) -> Result<()> {
        self.native.SetMinTallModeHeight(value)?;
        Ok(())
    }
}

impl Visible for TwoPaneView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, value: bool) -> Result<()> {
        self.element.set_visible(value)
    }
}
impl Enable for TwoPaneView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, value: bool) -> Result<()> {
        self.element.set_enabled(value)
    }
}
impl Layoutable for TwoPaneView {
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

#[derive(Debug)]
#[non_exhaustive]
pub enum TwoPaneViewMessage {}

impl Component for TwoPaneView {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = TwoPaneViewMessage;

    async fn init(parent: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeTwoPaneView::new()?;
        let pane1 = MUXC::Canvas::new()?;
        let pane2 = MUXC::Canvas::new()?;
        native.SetPane1(&pane1)?;
        native.SetPane2(&pane2)?;
        let element = NativeElement::new(
            parent,
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?,
        )?;
        Ok(Self {
            native,
            element,
            pane1,
            pane2,
        })
    }
}

impl AsWidget for TwoPaneView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
