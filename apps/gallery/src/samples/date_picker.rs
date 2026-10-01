use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{Date, DatePicker, DatePickerEvent, Month, ToggleButtonEvent};

pub struct DatePickerSample {
    page: GalleryPage,
    card: GalleryExample,
    picker: Child<DatePicker>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_date: bool,
    generation: u64,
}

impl Component for DatePickerSample {
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
            "DatePicker",
            "A native Gregorian DatePicker using time::Date rather than WinRT DateTime in the public API.",
            "apps/gallery/src/samples/date_picker.rs · winio_winui_ext::DatePicker",
            include_str!("date_picker.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("2026-10-01")?;

        let mut picker = Child::<DatePicker>::init(card.as_container()).await?;
        picker.set_header("Appointment date")?;
        picker.set_calendar_identifier("GregorianCalendar")?;
        picker.set_min_year(sample_date(2020, Month::January, 1))?;
        picker.set_max_year(sample_date(2030, Month::December, 31))?;
        picker.set_date(sample_date(2026, Month::October, 1))?;

        Ok(Self {
            page,
            card,
            picker,
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
            self.picker => {
                DatePickerEvent::DateChanged { new_date, .. } =>
                    GallerySampleMessage::Emit(format_date(new_date)),
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
                        sample_id: "date-picker",
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
        self.picker.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.picker.render()
    }
}

impl DatePickerSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.picker.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 28.0,
        ))?;
        self.picker.set_size(Size::new(
            (layout.primary.size.width - 40.0).clamp(0.0, 420.0),
            76.0,
        ))?;
        Ok(())
    }
}

fn sample_date(year: i32, month: Month, day: u8) -> Date {
    Date::from_calendar_date(year, month, day).expect("sample date must be valid")
}

fn format_date(value: Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        value.year(),
        u8::from(value.month()),
        value.day()
    )
}
