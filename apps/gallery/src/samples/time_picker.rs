use std::time::Duration;

use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{TimePicker, TimePickerEvent, ToggleButtonEvent};

pub struct TimePickerSample {
    page: GalleryPage,
    card: GalleryExample,
    picker: Child<TimePicker>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_time: bool,
    generation: u64,
}

impl Component for TimePickerSample {
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
            "TimePicker",
            "A native 24-hour TimePicker using std::time::Duration for time-of-day values.",
            "apps/gallery/src/samples/time_picker.rs · winio_winui_ext::TimePicker",
            include_str!("time_picker.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("10:30")?;

        let mut picker = Child::<TimePicker>::init(card.as_container()).await?;
        picker.set_header("Appointment time")?;
        picker.set_clock_identifier("24HourClock")?;
        picker.set_minute_increment(15)?;
        picker.set_time(Duration::from_secs(10 * 60 * 60 + 30 * 60))?;

        Ok(Self {
            page,
            card,
            picker,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_time: true,
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
                TimePickerEvent::TimeChanged { new_time, .. } =>
                    GallerySampleMessage::Emit(format_time(new_time)),
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
                let initial = self.skip_initial_time && value == "10:30";
                self.skip_initial_time = false;
                if !initial {
                    self.card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "time-picker",
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

impl TimePickerSample {
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
            (layout.primary.size.width - 40.0).clamp(0.0, 380.0),
            76.0,
        ))?;
        Ok(())
    }
}

fn format_time(value: Duration) -> String {
    let seconds = value.as_secs();
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;
    format!("{hours:02}:{minutes:02}")
}
