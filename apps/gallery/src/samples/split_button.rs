use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{SplitButton, SplitButtonEvent, ToggleButtonEvent};

pub struct SplitButtonSample {
    page: GalleryPage,
    card: GalleryExample,
    split: Child<SplitButton>,
    flyout_label: Child<Label>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for SplitButtonSample {
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
            "SplitButton",
            "A primary action paired with a native flyout that owns Winio content.",
            "apps/gallery/src/samples/split_button.rs · winio_winui_ext::SplitButton",
            include_str!("split_button.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("ready")?;

        let mut split = Child::<SplitButton>::init(card.as_container()).await?;
        split.set_text("Save")?;
        split.set_flyout_content_size(Size::new(260.0, 72.0))?;

        let mut flyout_label = Child::<Label>::init(split.flyout_container()).await?;
        flyout_label.set_text("Secondary action")?;
        flyout_label.set_loc(Point::new(16.0, 16.0))?;
        flyout_label.set_size(Size::new(228.0, 32.0))?;

        Ok(Self {
            page,
            card,
            split,
            flyout_label,
            viewport: Rect::zero(),
            source_expanded: false,
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
            self.split => {
                SplitButtonEvent::Click => GallerySampleMessage::Emit("primary click".to_owned()),
                SplitButtonEvent::FlyoutOpened => GallerySampleMessage::Emit("flyout opened".to_owned()),
                SplitButtonEvent::FlyoutClosed => GallerySampleMessage::Emit("flyout closed".to_owned()),
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
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "split-button",
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
        update_children!(self.split, self.flyout_label)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.split.render()?;
        self.flyout_label.render()
    }
}

impl SplitButtonSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.split.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 42.0,
        ))?;
        self.split.set_size(Size::new(
            (layout.primary.size.width - 40.0).clamp(0.0, 280.0),
            44.0,
        ))?;
        Ok(())
    }
}
