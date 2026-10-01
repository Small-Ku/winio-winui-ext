use std::time::Duration;

use windows::Foundation::{EventHandler, TimeSpan};
use windows::core::{HRESULT, HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    TimePicker as NativeTimePicker, TimePickerValueChangedEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls::TextBlock;

use crate::native::NativeElement;

const NANOS_PER_TICK: u128 = 100;
const E_INVALIDARG: HRESULT = HRESULT(0x80070057u32 as i32);

fn from_native_time(value: TimeSpan) -> Duration {
    if value.Duration <= 0 {
        Duration::ZERO
    } else {
        Duration::from_nanos(value.Duration as u64).saturating_mul(NANOS_PER_TICK as u32)
    }
}

fn to_native_time(value: Duration) -> Result<TimeSpan> {
    let ticks = value.as_nanos() / NANOS_PER_TICK;
    let ticks = i64::try_from(ticks)
        .map_err(|_| Error::new(E_INVALIDARG, "time exceeds the WinRT TimeSpan range"))?;
    Ok(TimeSpan { Duration: ticks })
}

/// A native WinUI TimePicker using std::time::Duration for time-of-day values.
#[derive(Debug)]
pub struct TimePicker {
    native: NativeTimePicker,
    element: NativeElement,
    header: TextBlock,
    time_changed_token: i64,
}

impl Failable for TimePicker {
    type Error = Error;
}

impl TimePicker {
    pub fn header(&self) -> Result<String> {
        Ok(self.header.Text()?.to_string_lossy())
    }

    pub fn set_header(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.header.SetText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }

    pub fn time(&self) -> Result<Duration> {
        Ok(from_native_time(self.native.Time()?))
    }

    pub fn set_time(&mut self, time: Duration) -> Result<()> {
        self.native.SetTime(to_native_time(time)?)?;
        Ok(())
    }

    pub fn minute_increment(&self) -> Result<i32> {
        self.native.MinuteIncrement()
    }

    pub fn set_minute_increment(&mut self, increment: i32) -> Result<()> {
        self.native.SetMinuteIncrement(increment)?;
        Ok(())
    }

    pub fn clock_identifier(&self) -> Result<String> {
        Ok(self.native.ClockIdentifier()?.to_string_lossy())
    }

    pub fn set_clock_identifier(&mut self, identifier: impl AsRef<str>) -> Result<()> {
        self.native
            .SetClockIdentifier(&HSTRING::from(identifier.as_ref()))?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for TimePicker {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for TimePicker {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for TimePicker {
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
pub enum TimePickerEvent {
    TimeChanged {
        old_time: Duration,
        new_time: Duration,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TimePickerMessage {}

impl Component for TimePicker {
    type Error = Error;
    type Event = TimePickerEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = TimePickerMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeTimePicker::new()?;
        let header = TextBlock::new()?;
        native.SetHeader(&header)?;

        let event_sender = sender.clone();
        let time_changed_token = native.TimeChanged(&EventHandler::<
            TimePickerValueChangedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let (Ok(old_time), Ok(new_time)) = (args.OldTime(), args.NewTime())
            {
                event_sender.output(TimePickerEvent::TimeChanged {
                    old_time: from_native_time(old_time),
                    new_time: from_native_time(new_time),
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
            header,
            time_changed_token,
        })
    }
}

impl AsWidget for TimePicker {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for TimePicker {
    fn drop(&mut self) {
        let _ = self.native.RemoveTimeChanged(self.time_changed_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timespan_round_trip_uses_std_duration() {
        let value = Duration::from_secs(10 * 60 * 60 + 30 * 60) + Duration::from_millis(123);
        let native = to_native_time(value).unwrap();
        assert_eq!(from_native_time(native), value);
    }
}
