use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::InfoBadge;

pub struct InfoBadgeSample {
    page: GalleryPage,
    card: GalleryExample,
    badge: Child<InfoBadge>,
    viewport: Rect,
    source_expanded: bool,
}
impl Component for InfoBadgeSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;
    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let page = GalleryPage::new(init.parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "InfoBadge",
            "Native numeric badge with a typed value round-trip.",
            "apps/gallery/src/samples/info_badge.rs · winio_winui_ext::InfoBadge",
            include_str!("info_badge.rs"),
            true,
            false,
        )
        .await?;
        let mut badge = Child::<InfoBadge>::init(card.as_container()).await?;
        badge.set_value(7)?;
        card.set_output(format!("value:{}", badge.value()?))?;
        Ok(Self {
            page,
            card,
            badge,
            viewport: Rect::zero(),
            source_expanded: false,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card = self.card.id();
        start! {sender,default:GallerySampleMessage::Emit(String::new()),self.card.source_toggle=>{winio_winui_ext::ToggleButtonEvent::StateChanged(state)=>GallerySampleMessage::SourceExpanded{card,expanded:state==Some(true)},},self.badge=>{}}
    }
    async fn update(&mut self, m: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(v) => {
                self.viewport = v;
                self.reflow()?
            }
            GallerySampleMessage::SourceExpanded {
                card: CardId::MAIN,
                expanded,
            } => {
                self.source_expanded = expanded;
                self.reflow()?
            }
            _ => {}
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.badge.update().await
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.badge.render()
    }
}
impl InfoBadgeSample {
    fn reflow(&mut self) -> Result<()> {
        let l = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, l)?;
        self.page.set_layout(self.viewport, l.content_height)?;
        self.badge.set_loc(Point::new(
            l.primary.origin.x + 32.0,
            l.primary.origin.y + 34.0,
        ))?;
        self.badge.set_size(Size::new(64.0, 48.0))?;
        Ok(())
    }
}
