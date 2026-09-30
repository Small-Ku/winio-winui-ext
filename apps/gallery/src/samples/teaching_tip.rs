use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    TeachingTip, TeachingTipEvent, TeachingTipPlacement, ToggleButton, ToggleButtonEvent,
};

pub struct TeachingTipSample {
    page: GalleryPage,
    card: GalleryExample,
    target: Child<ToggleButton>,
    tip: Child<TeachingTip>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}
impl Component for TeachingTipSample {
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
            "TeachingTip",
            "A native top-level popup anchored to a Winio target.",
            "apps/gallery/src/samples/teaching_tip.rs · winio_winui_ext::TeachingTip",
            include_str!("teaching_tip.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("Closed")?;
        init! { target: ToggleButton = (&card) => { text: "Open TeachingTip" }, tip: TeachingTip = (&card) => { size: Size::new(1.0, 1.0) }, }
        tip.set_title("Native TeachingTip canary")?;
        tip.set_subtitle("Popup lifecycle through Winio")?;
        tip.set_close_button_content("Close TeachingTip")?;
        tip.set_preferred_placement(TeachingTipPlacement::Bottom)?;
        tip.set_target(target.as_widget())?;
        Ok(Self {
            page,
            card,
            target,
            tip,
            viewport: Rect::zero(),
            source_expanded: false,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! { sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded { card: card_id, expanded: state == Some(true) }, },
            self.target => { ToggleButtonEvent::StateChanged(_) => GallerySampleMessage::OpenTeachingTip, },
            self.tip => { TeachingTipEvent::Closed { reason } => GallerySampleMessage::Emit(format!("closed:{reason:?}")), },
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
            GallerySampleMessage::OpenTeachingTip => {
                self.tip.set_is_open(true)?;
                let value = "opened".to_owned();
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "teaching-tip",
                    value,
                });
            }
            GallerySampleMessage::Emit(value) => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "teaching-tip",
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
        update_children!(self.target, self.tip)
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.target.render()?;
        self.tip.render()
    }
}
impl TeachingTipSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.target.set_loc(Point::new(
            layout.primary.origin.x + 22.0,
            layout.primary.origin.y + 44.0,
        ))?;
        self.target.set_size(Size::new(
            (layout.primary.size.width - 44.0).clamp(0.0, 300.0),
            44.0,
        ))?;
        Ok(())
    }
}
