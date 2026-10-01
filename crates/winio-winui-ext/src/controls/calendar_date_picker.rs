use windows::Foundation::{
    DateTime as NativeDateTime, IReference, PropertyValue, TypedEventHandler,
};
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    CalendarDatePicker as NativeCalendarDatePicker, CalendarDatePickerDateChangedEventArgs,
};
use winio_winui3::Microsoft::UI::Xaml::Controls::TextBlock;

use super::date_picker::{Date, from_native_date, to_native_date};
use crate::native::NativeElement;

fn date_reference(value: Date) -> Result<IReference<NativeDateTime>> {
    PropertyValue::CreateDateTime(to_native_date(value)?)?.cast::<IReference<NativeDateTime>>()
}

fn optional_date(reference: IReference<NativeDateTime>) -> Result<Option<Date>> {
    if reference.as_raw().is_null() {
        Ok(None)
    } else {
        Ok(Some(from_native_date(reference.Value()?)?))
    }
}

/// A native WinUI CalendarDatePicker reusing the shared time::Date API.
#[derive(Debug)]
pub struct CalendarDatePicker {
    native: NativeCalendarDatePicker,
    element: NativeElement,
    header: TextBlock,
    date_changed_token: i64,
}

impl Failable for CalendarDatePicker {
    type Error = Error;
}

impl CalendarDatePicker {
    pub fn header(&self) -> Result<String> {
        Ok(self.header.Text()?.to_string_lossy())
    }

    pub fn set_header(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.header.SetText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn date(&self) -> Result<Option<Date>> {
        optional_date(self.native.Date()?)
    }

    pub fn set_date(&mut self, value: Option<Date>) -> Result<()> {
        match value {
            Some(value) => {
                let reference = date_reference(value)?;
                self.native.SetDate(&reference)?;
            }
            None => {
                self.native.SetDate(None::<&IReference<NativeDateTime>>)?;
            }
        }
        Ok(())
    }

    pub fn is_calendar_open(&self) -> Result<bool> {
        self.native.IsCalendarOpen()
    }

    pub fn set_is_calendar_open(&mut self, open: bool) -> Result<()> {
        self.native.SetIsCalendarOpen(open)?;
        Ok(())
    }

    pub fn date_format(&self) -> Result<String> {
        Ok(self.native.DateFormat()?.to_string_lossy())
    }

    pub fn set_date_format(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetDateFormat(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn placeholder_text(&self) -> Result<String> {
        Ok(self.native.PlaceholderText()?.to_string_lossy())
    }

    pub fn set_placeholder_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .SetPlaceholderText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn min_date(&self) -> Result<Date> {
        from_native_date(self.native.MinDate()?)
    }

    pub fn set_min_date(&mut self, value: Date) -> Result<()> {
        self.native.SetMinDate(to_native_date(value)?)?;
        Ok(())
    }

    pub fn max_date(&self) -> Result<Date> {
        from_native_date(self.native.MaxDate()?)
    }

    pub fn set_max_date(&mut self, value: Date) -> Result<()> {
        self.native.SetMaxDate(to_native_date(value)?)?;
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

    pub fn is_today_highlighted(&self) -> Result<bool> {
        self.native.IsTodayHighlighted()
    }

    pub fn set_is_today_highlighted(&mut self, highlighted: bool) -> Result<()> {
        self.native.SetIsTodayHighlighted(highlighted)?;
        Ok(())
    }

    pub fn is_group_label_visible(&self) -> Result<bool> {
        self.native.IsGroupLabelVisible()
    }

    pub fn set_is_group_label_visible(&mut self, visible: bool) -> Result<()> {
        self.native.SetIsGroupLabelVisible(visible)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for CalendarDatePicker {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for CalendarDatePicker {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for CalendarDatePicker {
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
pub enum CalendarDatePickerEvent {
    DateChanged {
        old_date: Option<Date>,
        new_date: Option<Date>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CalendarDatePickerMessage {}

impl Component for CalendarDatePicker {
    type Error = Error;
    type Event = CalendarDatePickerEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = CalendarDatePickerMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeCalendarDatePicker::new()?;
        let header = TextBlock::new()?;
        native.SetHeader(&header)?;

        let event_sender = sender.clone();
        let date_changed_token = native.DateChanged(&TypedEventHandler::<
            NativeCalendarDatePicker,
            CalendarDatePickerDateChangedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref() {
                let old_date = args
                    .OldDate()
                    .ok()
                    .and_then(|reference| optional_date(reference).ok())
                    .flatten();
                let new_date = args
                    .NewDate()
                    .ok()
                    .and_then(|reference| optional_date(reference).ok())
                    .flatten();
                event_sender.output(CalendarDatePickerEvent::DateChanged { old_date, new_date });
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

impl AsWidget for CalendarDatePicker {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for CalendarDatePicker {
    fn drop(&mut self) {
        let _ = self.native.RemoveDateChanged(self.date_changed_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn date_reference_round_trip_preserves_calendar_day() {
        let date = Date::from_calendar_date(2026, Month::October, 1).unwrap();
        let reference = date_reference(date).unwrap();
        assert_eq!(optional_date(reference).unwrap(), Some(date));
    }
}
