use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ProgressRing, ToggleButton, ToggleButtonEvent};

pub struct ProgressRingSample {
    page: GalleryPage,
    card: GalleryExample,
    ring: Child<ProgressRing>,
    indeterminate: Child<ToggleButton>,
    viewport: Rect,
    source_expanded: bool,
}

impl Component for ProgressRingSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit {
            parent,
            generation: _,
        } = init;
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "ProgressRing",
            "A native determinate ProgressRing with a live indeterminate option.",
            "apps/gallery/src/samples/progress_ring.rs · winio_winui_ext::ProgressRing",
            include_str!("progress_ring.rs"),
            true,
            true,
        )
        .await?;
        card.set_output("value:65 · determinate")?;

        let mut ring = Child::<ProgressRing>::init(card.as_container()).await?;
        ring.set_minimum(0.0)?;
        ring.set_maximum(100.0)?;
        ring.set_value(65.0)?;
        ring.set_is_indeterminate(false)?;
        ring.set_is_active(true)?;

        let mut indeterminate = Child::<ToggleButton>::init(card.as_container()).await?;
        indeterminate.set_text("Indeterminate")?;
        indeterminate.set_three_state(false)?;

        Ok(Self {
            page,
            card,
            ring,
            indeterminate,
            viewport: Rect::zero(),
            source_expanded: false,
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
            self.indeterminate => {
                ToggleButtonEvent::StateChanged(state) =>
                    GallerySampleMessage::Emit(if state == Some(true) {
                        "progress-ring:indeterminate".to_owned()
                    } else {
                        "progress-ring:determinate".to_owned()
                    }),
            },
            self.ring => {},
        }
    }

    async fn update(&mut self, message: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
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
            GallerySampleMessage::Emit(value) if value == "progress-ring:indeterminate" => {
                self.ring.set_is_indeterminate(true)?;
                self.card.set_output("indeterminate")?;
            }
            GallerySampleMessage::Emit(value) if value == "progress-ring:determinate" => {
                self.ring.set_is_indeterminate(false)?;
                self.card.set_output("value:65 · determinate")?;
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.ring, self.indeterminate)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.ring.render()?;
        self.indeterminate.render()
    }
}

impl ProgressRingSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.ring.set_loc(Point::new(
            layout.primary.origin.x + 56.0,
            layout.primary.origin.y + 36.0,
        ))?;
        self.ring.set_size(Size::new(96.0, 96.0))?;

        if let Some(options) = layout.options {
            self.indeterminate.set_visible(true)?;
            self.indeterminate
                .set_loc(Point::new(options.origin.x + 12.0, options.origin.y + 38.0))?;
            self.indeterminate
                .set_size(Size::new((options.size.width - 24.0).max(0.0), 36.0))?;
        } else {
            self.indeterminate.set_visible(false)?;
        }
        Ok(())
    }
}
