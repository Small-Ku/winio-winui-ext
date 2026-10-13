use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{CalendarView, CalendarViewEvent, Date, Month, ToggleButtonEvent};

pub struct CalendarViewSample {
    page: GalleryPage,
    card: GalleryExample,
    calendar: Child<CalendarView>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_date: bool,
    generation: u64,
}

impl Component for CalendarViewSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "CalendarView",
            "An inline native calendar with an intentionally single-date selection contract.",
            "apps/gallery/src/samples/calendar_view.rs · winio_winui_ext::CalendarView",
            include_str!("calendar_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("2026-10-01")?;
        card.set_primary_height(360.0);
        card.set_min_primary_width(480.0);

        let mut calendar = Child::<CalendarView>::init(card.as_container()).await?;
        calendar.set_calendar_identifier("GregorianCalendar")?;
        calendar.set_day_of_week_format("{dayofweek.abbreviated(2)}")?;
        calendar.set_min_date(sample_date(2020, Month::January, 1))?;
        calendar.set_max_date(sample_date(2030, Month::December, 31))?;
        calendar.set_is_today_highlighted(true)?;
        calendar.set_is_out_of_scope_enabled(true)?;
        calendar.set_is_group_label_visible(false)?;
        calendar.set_number_of_weeks_in_view(2)?;
        calendar.set_display_date(sample_date(2026, Month::October, 1))?;
        calendar.set_selected_date(Some(sample_date(2026, Month::October, 1)))?;

        Ok(Self {
            page,
            card,
            calendar,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_date: true,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => {
                ToggleButtonEvent::StateChanged(state) =>
                    GallerySampleMessage::SourceExpanded {
                        card: card_id,
                        expanded: state == Some(true),
                    },
            },
            self.calendar => {
                CalendarViewEvent::SelectedDateChanged { new_date, .. } =>
                    GallerySampleMessage::Emit(format_optional_date(new_date)),
            },
        }
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            GallerySampleMessage::SetBounds(viewport) => {
                self.viewport = viewport;
                self.reflow()?;
            }
            GallerySampleMessage::SourceExpanded {
                card: CardId::MAIN,
                expanded,
            } => {
                self.source_expanded = expanded;
                self.reflow()?;
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                let initial = self.skip_initial_date && value == "2026-10-01";
                self.skip_initial_date = false;
                if !initial {
                    self.card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "calendar-view",
                        value,
                    });
                }
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.calendar.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.calendar.render()
    }
}

impl CalendarViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.calendar.set_loc(Point::new(
            layout.primary.origin.x + 8.0,
            layout.primary.origin.y + 4.0,
        ))?;
        self.calendar.set_size(Size::new(
            (layout.primary.size.width - 16.0).max(0.0),
            (layout.primary.size.height - 8.0).max(0.0),
        ))?;
        Ok(())
    }
}

fn sample_date(year: i32, month: Month, day: u8) -> Date {
    Date::from_calendar_date(year, month, day).expect("sample date must be valid")
}

fn format_optional_date(value: Option<Date>) -> String {
    value.map_or_else(
        || "none".to_owned(),
        |value| {
            format!(
                "{:04}-{:02}-{:02}",
                value.year(),
                u8::from(value.month()),
                value.day()
            )
        },
    )
}
