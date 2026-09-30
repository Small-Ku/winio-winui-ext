use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{Expander, ExpanderEvent, ToggleButtonEvent};

pub struct ExpanderSample {
    page: GalleryPage,
    card: GalleryExample,
    expander: Child<Expander>,
    nested: Child<Label>,
    viewport: Rect,
    source_expanded: bool,
    expanded: bool,
    generation: u64,
}

impl Component for ExpanderSample {
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
            "Expander",
            "A native Expander that owns Winio children through its content Canvas.",
            "apps/gallery/src/samples/expander.rs · winio_winui_ext::Expander",
            include_str!("expander.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("collapsed")?;

        init! {
            expander: Expander = (&card) => { text: "Advanced settings" },
            nested: Label = (&expander) => { text: "Nested Winio content" },
        }
        expander.set_content_size(Size::new(360.0, 72.0))?;
        expander.set_is_expanded(false)?;

        Ok(Self {
            page,
            card,
            expander,
            nested,
            viewport: Rect::zero(),
            source_expanded: false,
            expanded: false,
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
            self.expander => {
                ExpanderEvent::Expanding => GallerySampleMessage::Emit("expanded".to_owned()),
                ExpanderEvent::Collapsed => GallerySampleMessage::Emit("collapsed".to_owned()),
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
                self.expanded = value == "expanded";
                self.card.set_output(&value)?;
                self.reflow()?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "expander",
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
        update_children!(self.expander, self.nested)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.expander.render()?;
        self.nested.render()
    }
}

impl ExpanderSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        let width = (layout.primary.size.width - 40.0).clamp(0.0, 460.0);
        let content_width = (width - 32.0).max(0.0);
        self.expander
            .set_content_size(Size::new(content_width, 72.0))?;
        self.expander.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 24.0,
        ))?;
        self.expander
            .set_size(Size::new(width, if self.expanded { 132.0 } else { 52.0 }))?;

        self.nested.set_loc(Point::new(12.0, 16.0))?;
        self.nested
            .set_size(Size::new((content_width - 24.0).max(0.0), 32.0))?;
        Ok(())
    }
}
