use windows::Foundation::TypedEventHandler;
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    CalendarView as NativeCalendarView, CalendarViewSelectedDatesChangedEventArgs,
    CalendarViewSelectionMode,
};

use super::date_picker::{Date, from_native_date, to_native_date};
use crate::native::NativeElement;

/// Shows an inline WinUI CalendarView with one selectable date.
///
/// The native control also supports multiple selection. This wrapper sets
/// SelectionMode to Single and uses time::Date for its selected value.
#[derive(Debug)]
pub struct CalendarView {
    native: NativeCalendarView,
    element: NativeElement,
    selected_dates_changed_token: i64,
}

impl Failable for CalendarView {
    type Error = Error;
}

impl CalendarView {
    pub fn selected_date(&self) -> Result<Option<Date>> {
        let dates = self.native.SelectedDates()?;
        if dates.Size()? == 0 {
            Ok(None)
        } else {
            Ok(Some(from_native_date(dates.GetAt(0)?)?))
        }
    }

    pub fn set_selected_date(&mut self, value: Option<Date>) -> Result<()> {
        let dates = self.native.SelectedDates()?;
        match (dates.Size()?, value) {
            (0, Some(value)) => dates.Append(to_native_date(value)?)?,
            (_, Some(value)) => dates.SetAt(0, to_native_date(value)?)?,
            (0, None) => {}
            (_, None) => dates.Clear()?,
        }
        Ok(())
    }

    pub fn set_display_date(&mut self, value: Date) -> Result<()> {
        self.native.SetDisplayDate(to_native_date(value)?)?;
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

    pub fn day_of_week_format(&self) -> Result<String> {
        Ok(self.native.DayOfWeekFormat()?.to_string_lossy())
    }

    pub fn set_day_of_week_format(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .SetDayOfWeekFormat(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn is_today_highlighted(&self) -> Result<bool> {
        self.native.IsTodayHighlighted()
    }

    pub fn set_is_today_highlighted(&mut self, value: bool) -> Result<()> {
        self.native.SetIsTodayHighlighted(value)?;
        Ok(())
    }

    pub fn is_out_of_scope_enabled(&self) -> Result<bool> {
        self.native.IsOutOfScopeEnabled()
    }

    pub fn set_is_out_of_scope_enabled(&mut self, value: bool) -> Result<()> {
        self.native.SetIsOutOfScopeEnabled(value)?;
        Ok(())
    }

    pub fn is_group_label_visible(&self) -> Result<bool> {
        self.native.IsGroupLabelVisible()
    }

    pub fn set_is_group_label_visible(&mut self, value: bool) -> Result<()> {
        self.native.SetIsGroupLabelVisible(value)?;
        Ok(())
    }

    pub fn number_of_weeks_in_view(&self) -> Result<i32> {
        self.native.NumberOfWeeksInView()
    }

    pub fn set_number_of_weeks_in_view(&mut self, value: i32) -> Result<()> {
        self.native.SetNumberOfWeeksInView(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for CalendarView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for CalendarView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for CalendarView {
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
pub enum CalendarViewEvent {
    SelectedDateChanged {
        old_date: Option<Date>,
        new_date: Option<Date>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CalendarViewMessage {}

impl Component for CalendarView {
    type Error = Error;
    type Event = CalendarViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = CalendarViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeCalendarView::new()?;
        native.SetSelectionMode(CalendarViewSelectionMode::Single)?;

        let event_sender = sender.clone();
        let selected_dates_changed_token =
            native.SelectedDatesChanged(&TypedEventHandler::<
                NativeCalendarView,
                CalendarViewSelectedDatesChangedEventArgs,
            >::new(move |_, args| {
                if let Some(args) = args.as_ref() {
                    let old_date = args
                        .RemovedDates()
                        .ok()
                        .and_then(|dates| {
                            (dates.Size().ok()? > 0)
                                .then(|| dates.GetAt(0).ok())
                                .flatten()
                        })
                        .and_then(|value| from_native_date(value).ok());
                    let new_date = args
                        .AddedDates()
                        .ok()
                        .and_then(|dates| {
                            (dates.Size().ok()? > 0)
                                .then(|| dates.GetAt(0).ok())
                                .flatten()
                        })
                        .and_then(|value| from_native_date(value).ok());
                    event_sender
                        .output(CalendarViewEvent::SelectedDateChanged { old_date, new_date });
                }
                Ok(())
            }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            selected_dates_changed_token,
        })
    }
}

impl AsWidget for CalendarView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for CalendarView {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectedDatesChanged(self.selected_dates_changed_token);
    }
}
