use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::InfoBar;

pub struct InfoBarSample {
    page: GalleryPage,
    card: GalleryExample,
    info: Child<InfoBar>,
    viewport: Rect,
    source_expanded: bool,
}
impl Component for InfoBarSample {
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
            "InfoBar",
            "A persistent informational status message.",
            "apps/gallery/src/samples/info_bar.rs · winio_winui_ext::InfoBar",
            include_str!("info_bar.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("Open · informational status")?;
        init! { info: InfoBar = (&card) => { size: Size::new(480.0, 82.0) } }
        info.set_message("Supplemental InfoBar canary")?;
        info.set_is_closable(false)?;
        info.set_is_open(true)?;
        Ok(Self {
            page,
            card,
            info,
            viewport: Rect::zero(),
            source_expanded: false,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! { sender, default: GallerySampleMessage::Emit(String::new()), self.card.source_toggle => { winio_winui_ext::ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded { card: card_id, expanded: state == Some(true) }, }, self.info => {}, }
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
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(value)?
            }
            _ => {}
        }
        Ok(false)
    }
    #[allow(clippy::needless_question_mark)]
    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.info)
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.info.render()
    }
}
impl InfoBarSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.info.set_loc(Point::new(
            layout.primary.origin.x + 18.0,
            layout.primary.origin.y + 32.0,
        ))?;
        self.info
            .set_size(Size::new((layout.primary.size.width - 36.0).max(0.0), 82.0))?;
        Ok(())
    }
}
