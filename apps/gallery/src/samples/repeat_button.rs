use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{RepeatButton, RepeatButtonEvent, ToggleButtonEvent};

pub struct RepeatButtonSample {
    page: GalleryPage,
    card: GalleryExample,
    repeat: Child<RepeatButton>,
    viewport: Rect,
    source_expanded: bool,
    count: u32,
    generation: u64,
}

impl Component for RepeatButtonSample {
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
            "RepeatButton",
            "A native button that repeats Click while the pointer remains pressed.",
            "apps/gallery/src/samples/repeat_button.rs · winio_winui_ext::RepeatButton",
            include_str!("repeat_button.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("count:0")?;

        let mut repeat = Child::<RepeatButton>::init(card.as_container()).await?;
        repeat.set_text("Press and hold")?;
        repeat.set_delay(150)?;
        repeat.set_interval(80)?;

        Ok(Self {
            page,
            card,
            repeat,
            viewport: Rect::zero(),
            source_expanded: false,
            count: 0,
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
            self.repeat => {
                RepeatButtonEvent::Click => GallerySampleMessage::Emit("click".to_owned()),
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
            GallerySampleMessage::Emit(value) if value == "click" => {
                self.count = self.count.saturating_add(1);
                let value = format!("count:{}", self.count);
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "repeat-button",
                    value,
                });
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.repeat.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.repeat.render()
    }
}

impl RepeatButtonSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.repeat.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 34.0,
        ))?;
        let preferred = self.repeat.preferred_size()?;
        self.repeat.set_size(Size::new(
            (layout.primary.size.width - 40.0)
                .min(preferred.width.max(180.0))
                .max(0.0),
            preferred.height.max(48.0),
        ))?;
        Ok(())
    }
}
