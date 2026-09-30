use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{RatingControl, RatingControlEvent, ToggleButtonEvent};

pub struct RatingControlSample {
    page: GalleryPage,
    card: GalleryExample,
    rating: Child<RatingControl>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_value: bool,
    generation: u64,
}

impl Component for RatingControlSample {
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
            "RatingControl",
            "A native five-star rating input with typed value changes.",
            "apps/gallery/src/samples/rating_control.rs · winio_winui_ext::RatingControl",
            include_str!("rating_control.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("3")?;

        init! {
            rating: RatingControl = (&card),
        }
        rating.set_caption("Rating")?;
        rating.set_max_rating(5)?;
        rating.set_is_read_only(false)?;
        rating.set_is_clear_enabled(true)?;
        rating.set_value(3.0)?;

        Ok(Self {
            page,
            card,
            rating,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_value: true,
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
            self.rating => {
                RatingControlEvent::ValueChanged(value) =>
                    GallerySampleMessage::Emit(format_rating(value)),
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
                let initial = self.skip_initial_value && value == "3";
                self.skip_initial_value = false;
                if !initial {
                    self.card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "rating-control",
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
        self.rating.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.rating.render()
    }
}

impl RatingControlSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.rating.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 34.0,
        ))?;
        let preferred = self.rating.preferred_size()?;
        self.rating.set_size(Size::new(
            (layout.primary.size.width - 40.0)
                .min(preferred.width.max(280.0))
                .max(0.0),
            preferred.height.max(52.0),
        ))?;
        Ok(())
    }
}

fn format_rating(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i32)
    } else {
        format!("{value:.1}")
    }
}
