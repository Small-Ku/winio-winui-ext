use windows::Foundation::{DateTime as NativeDateTime, EventHandler};
use windows::core::{HRESULT, HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    DatePicker as NativeDatePicker, DatePickerValueChangedEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls::TextBlock;

use crate::native::NativeElement;

pub use time::{Date, Month};

const WINDOWS_EPOCH_JULIAN_DAY: i32 = 2_305_814;
const TICKS_PER_DAY: i64 = 864_000_000_000;
const E_INVALIDARG: HRESULT = HRESULT(0x80070057u32 as i32);

pub(super) fn to_native_date(value: Date) -> Result<NativeDateTime> {
    let days = i64::from(value.to_julian_day() - WINDOWS_EPOCH_JULIAN_DAY);
    let ticks = days
        .checked_mul(TICKS_PER_DAY)
        .ok_or_else(|| Error::new(E_INVALIDARG, "date exceeds the WinRT DateTime range"))?;
    Ok(NativeDateTime {
        UniversalTime: ticks,
    })
}

pub(super) fn from_native_date(value: NativeDateTime) -> Result<Date> {
    let days = value.UniversalTime.div_euclid(TICKS_PER_DAY);
    let julian_day = i64::from(WINDOWS_EPOCH_JULIAN_DAY)
        .checked_add(days)
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| {
            Error::new(
                E_INVALIDARG,
                "WinRT DateTime exceeds the supported date range",
            )
        })?;
    Date::from_julian_day(julian_day).map_err(|_| {
        Error::new(
            E_INVALIDARG,
            "WinRT DateTime is not a supported calendar date",
        )
    })
}

/// A native WinUI DatePicker using time::Date for calendar-day values.
#[derive(Debug)]
pub struct DatePicker {
    native: NativeDatePicker,
    element: NativeElement,
    header: TextBlock,
    date_changed_token: i64,
}

impl Failable for DatePicker {
    type Error = Error;
}

impl DatePicker {
    pub fn header(&self) -> Result<String> {
        Ok(self.header.Text()?.to_string_lossy())
    }

    pub fn set_header(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.header.SetText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn date(&self) -> Result<Date> {
        from_native_date(self.native.Date()?)
    }

    pub fn set_date(&mut self, value: Date) -> Result<()> {
        self.native.SetDate(to_native_date(value)?)?;
        Ok(())
    }

    pub fn min_year(&self) -> Result<Date> {
        from_native_date(self.native.MinYear()?)
    }

    pub fn set_min_year(&mut self, value: Date) -> Result<()> {
        self.native.SetMinYear(to_native_date(value)?)?;
        Ok(())
    }

    pub fn max_year(&self) -> Result<Date> {
        from_native_date(self.native.MaxYear()?)
    }

    pub fn set_max_year(&mut self, value: Date) -> Result<()> {
        self.native.SetMaxYear(to_native_date(value)?)?;
        Ok(())
    }

    pub fn calendar_identifier(&self) -> Result<String> {
        Ok(self.native.CalendarIdentifier()?.to_string_lossy())
    }

    pub fn set_calendar_identifier(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .SetCalendarIdentifier(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn is_day_visible(&self) -> Result<bool> {
        self.native.DayVisible()
    }

    pub fn set_day_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetDayVisible(visible)?;
        Ok(())
    }

    pub fn is_month_visible(&self) -> Result<bool> {
        self.native.MonthVisible()
    }

    pub fn set_month_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetMonthVisible(visible)?;
        Ok(())
    }

    pub fn is_year_visible(&self) -> Result<bool> {
        self.native.YearVisible()
    }

    pub fn set_year_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetYearVisible(visible)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for DatePicker {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for DatePicker {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for DatePicker {
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
pub enum DatePickerEvent {
    DateChanged { old_date: Date, new_date: Date },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DatePickerMessage {}

impl Component for DatePicker {
    type Error = Error;
    type Event = DatePickerEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = DatePickerMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeDatePicker::new()?;
        let header = TextBlock::new()?;
        native.SetHeader(&header)?;

        let event_sender = sender.clone();
        let date_changed_token = native.DateChanged(&EventHandler::<
            DatePickerValueChangedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let (Ok(old_native), Ok(new_native)) = (args.OldDate(), args.NewDate())
                && let (Ok(old_date), Ok(new_date)) =
                    (from_native_date(old_native), from_native_date(new_native))
            {
                event_sender.output(DatePickerEvent::DateChanged { old_date, new_date });
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
            date_changed_token,
        })
    }
}

impl AsWidget for DatePicker {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for DatePicker {
    fn drop(&mut self) {
        let _ = self.native.RemoveDateChanged(self.date_changed_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winrt_datetime_round_trip_uses_time_date() {
        for value in [
            Date::from_calendar_date(1601, Month::January, 1).unwrap(),
            Date::from_calendar_date(1970, Month::January, 1).unwrap(),
            Date::from_calendar_date(2026, Month::October, 1).unwrap(),
            Date::from_calendar_date(9999, Month::December, 31).unwrap(),
        ] {
            assert_eq!(
                from_native_date(to_native_date(value).unwrap()).unwrap(),
                value
            );
        }
    }

    #[test]
    fn native_datetime_with_time_of_day_maps_to_calendar_date() {
        let date = Date::from_calendar_date(2026, Month::October, 1).unwrap();
        let native = to_native_date(date).unwrap();
        let afternoon = NativeDateTime {
            UniversalTime: native.UniversalTime + 15 * 60 * 60 * 10_000_000,
        };
        assert_eq!(from_native_date(afternoon).unwrap(), date);
    }
}
