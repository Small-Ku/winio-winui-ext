use windows::Foundation::TypedEventHandler;
use windows::core::Interface;
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    NumberBox as NativeNumberBox, NumberBoxSpinButtonPlacementMode, NumberBoxValueChangedEventArgs,
};

use crate::native::NativeElement;

/// Placement of the native NumberBox spin buttons.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NumberBoxSpinPlacement {
    #[default]
    Hidden,
    Compact,
    Inline,
}

impl NumberBoxSpinPlacement {
    fn native(self) -> NumberBoxSpinButtonPlacementMode {
        match self {
            Self::Hidden => NumberBoxSpinButtonPlacementMode::Hidden,
            Self::Compact => NumberBoxSpinButtonPlacementMode::Compact,
            Self::Inline => NumberBoxSpinButtonPlacementMode::Inline,
        }
    }
}

/// A native WinUI `NumberBox` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct NumberBox {
    native: NativeNumberBox,
    element: NativeElement,
    value_changed_token: i64,
}

impl Failable for NumberBox {
    type Error = Error;
}

impl NumberBox {
    pub fn value(&self) -> Result<f64> {
        self.native.Value()
    }

    pub fn set_value(&mut self, value: f64) -> Result<()> {
        self.native.SetValue(value)?;
        Ok(())
    }

    pub fn minimum(&self) -> Result<f64> {
        self.native.Minimum()
    }

    pub fn set_minimum(&mut self, minimum: f64) -> Result<()> {
        self.native.SetMinimum(minimum)?;
        Ok(())
    }

    pub fn maximum(&self) -> Result<f64> {
        self.native.Maximum()
    }

    pub fn set_maximum(&mut self, maximum: f64) -> Result<()> {
        self.native.SetMaximum(maximum)?;
        Ok(())
    }

    pub fn small_change(&self) -> Result<f64> {
        self.native.SmallChange()
    }

    pub fn set_small_change(&mut self, small_change: f64) -> Result<()> {
        self.native.SetSmallChange(small_change)?;
        Ok(())
    }

    pub fn set_spin_button_placement(&mut self, placement: NumberBoxSpinPlacement) -> Result<()> {
        self.native.SetSpinButtonPlacementMode(placement.native())?;
        Ok(())
    }
}

impl Visible for NumberBox {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for NumberBox {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for NumberBox {
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

/// Native value changes emitted by [`NumberBox`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum NumberBoxEvent {
    ValueChanged { old_value: f64, new_value: f64 },
}

/// Input messages for [`NumberBox`].
#[derive(Debug)]
#[non_exhaustive]
pub enum NumberBoxMessage {}

impl Component for NumberBox {
    type Error = Error;
    type Event = NumberBoxEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = NumberBoxMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeNumberBox::new()?;
        let value_sender = sender.clone();
        let value_changed_token = native.ValueChanged(&TypedEventHandler::<
            NativeNumberBox,
            NumberBoxValueChangedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let (Ok(old_value), Ok(new_value)) = (args.OldValue(), args.NewValue())
            {
                value_sender.output(NumberBoxEvent::ValueChanged {
                    old_value,
                    new_value,
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
            value_changed_token,
        })
    }
}

impl AsWidget for NumberBox {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for NumberBox {
    fn drop(&mut self) {
        let _ = self.native.RemoveValueChanged(self.value_changed_token);
    }
}
