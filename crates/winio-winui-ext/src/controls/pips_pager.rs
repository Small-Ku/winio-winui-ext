use windows::Foundation::TypedEventHandler;
use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    PipsPager as NativePipsPager, PipsPagerButtonVisibility as NativeButtonVisibility,
    PipsPagerSelectedIndexChangedEventArgs, PipsPagerWrapMode as NativeWrapMode,
};

use crate::native::NativeElement;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PipsPagerButtonVisibility {
    Visible,
    #[default]
    VisibleOnPointerOver,
    Collapsed,
}

impl PipsPagerButtonVisibility {
    fn native(self) -> NativeButtonVisibility {
        match self {
            Self::Visible => NativeButtonVisibility::Visible,
            Self::VisibleOnPointerOver => NativeButtonVisibility::VisibleOnPointerOver,
            Self::Collapsed => NativeButtonVisibility::Collapsed,
        }
    }

    fn from_native(value: NativeButtonVisibility) -> Option<Self> {
        if value == NativeButtonVisibility::Visible {
            Some(Self::Visible)
        } else if value == NativeButtonVisibility::VisibleOnPointerOver {
            Some(Self::VisibleOnPointerOver)
        } else if value == NativeButtonVisibility::Collapsed {
            Some(Self::Collapsed)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PipsPagerWrapMode {
    #[default]
    None,
    Wrap,
}

impl PipsPagerWrapMode {
    fn native(self) -> NativeWrapMode {
        match self {
            Self::None => NativeWrapMode::None,
            Self::Wrap => NativeWrapMode::Wrap,
        }
    }

    fn from_native(value: NativeWrapMode) -> Option<Self> {
        if value == NativeWrapMode::None {
            Some(Self::None)
        } else if value == NativeWrapMode::Wrap {
            Some(Self::Wrap)
        } else {
            None
        }
    }
}

/// A native WinUI PipsPager hosted as a first-class Winio component.
#[derive(Debug)]
pub struct PipsPager {
    native: NativePipsPager,
    element: NativeElement,
    selected_index_changed_token: i64,
}

impl Failable for PipsPager {
    type Error = Error;
}

impl PipsPager {
    pub fn number_of_pages(&self) -> Result<i32> {
        self.native.NumberOfPages()
    }

    pub fn set_number_of_pages(&mut self, value: i32) -> Result<()> {
        self.native.SetNumberOfPages(value)?;
        Ok(())
    }

    pub fn selected_page_index(&self) -> Result<i32> {
        self.native.SelectedPageIndex()
    }

    pub fn set_selected_page_index(&mut self, value: i32) -> Result<()> {
        self.native.SetSelectedPageIndex(value)?;
        Ok(())
    }

    pub fn max_visible_pips(&self) -> Result<i32> {
        self.native.MaxVisiblePips()
    }

    pub fn set_max_visible_pips(&mut self, value: i32) -> Result<()> {
        self.native.SetMaxVisiblePips(value)?;
        Ok(())
    }

    pub fn previous_button_visibility(&self) -> Result<Option<PipsPagerButtonVisibility>> {
        Ok(PipsPagerButtonVisibility::from_native(
            self.native.PreviousButtonVisibility()?,
        ))
    }

    pub fn set_previous_button_visibility(
        &mut self,
        value: PipsPagerButtonVisibility,
    ) -> Result<()> {
        self.native.SetPreviousButtonVisibility(value.native())?;
        Ok(())
    }

    pub fn next_button_visibility(&self) -> Result<Option<PipsPagerButtonVisibility>> {
        Ok(PipsPagerButtonVisibility::from_native(
            self.native.NextButtonVisibility()?,
        ))
    }

    pub fn set_next_button_visibility(&mut self, value: PipsPagerButtonVisibility) -> Result<()> {
        self.native.SetNextButtonVisibility(value.native())?;
        Ok(())
    }

    pub fn wrap_mode(&self) -> Result<Option<PipsPagerWrapMode>> {
        Ok(PipsPagerWrapMode::from_native(self.native.WrapMode()?))
    }

    pub fn set_wrap_mode(&mut self, value: PipsPagerWrapMode) -> Result<()> {
        self.native.SetWrapMode(value.native())?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for PipsPager {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for PipsPager {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for PipsPager {
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
pub enum PipsPagerEvent {
    SelectedIndexChanged { selected_page_index: i32 },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PipsPagerMessage {}

impl Component for PipsPager {
    type Error = Error;
    type Event = PipsPagerEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = PipsPagerMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativePipsPager::new()?;
        let event_native = native.clone();
        let event_sender = sender.clone();
        let selected_index_changed_token =
            native.SelectedIndexChanged(&TypedEventHandler::<
                NativePipsPager,
                PipsPagerSelectedIndexChangedEventArgs,
            >::new(move |_, _| {
                if let Ok(selected_page_index) = event_native.SelectedPageIndex() {
                    event_sender.output(PipsPagerEvent::SelectedIndexChanged {
                        selected_page_index,
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
            selected_index_changed_token,
        })
    }
}

impl AsWidget for PipsPager {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for PipsPager {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectedIndexChanged(self.selected_index_changed_token);
    }
}
