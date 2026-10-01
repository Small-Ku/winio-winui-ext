use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ToggleButtonEvent, Viewbox};
use winio_winui3::Microsoft::UI::Xaml::Media::Stretch;

pub struct ViewboxSample {
    page: GalleryPage,
    card: GalleryExample,
    viewbox: Child<Viewbox>,
    content: Child<Label>,
    viewport: Rect,
    source_expanded: bool,
}

impl Component for ViewboxSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, .. } = init;
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "Viewbox",
            "A native Viewbox scales its owned Canvas and the Winio content created inside it.",
            "apps/gallery/src/samples/viewbox.rs · winio_winui_ext::Viewbox",
            include_str!("viewbox.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("content 360 × 180; Viewbox 240 × 120")?;

        let mut viewbox = Child::<Viewbox>::init(card.as_container()).await?;
        viewbox.set_content_size(Size::new(360.0, 180.0))?;
        viewbox.set_stretch(Stretch::Uniform)?;
        let mut content = Child::<Label>::init(viewbox.as_container()).await?;
        content.set_text("Native Viewbox scales this Winio label")?;
        content.set_loc(Point::new(20.0, 60.0))?;
        content.set_size(Size::new(320.0, 60.0))?;

        Ok(Self {
            page,
            card,
            viewbox,
            content,
            viewport: Rect::zero(),
            source_expanded: false,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded {
                    card: card_id,
                    expanded: state == Some(true),
                },
            },
        }
    }

    async fn update(
        &mut self,
        message: Self::Message,
        _sender: &ComponentSender<Self>,
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
            GallerySampleMessage::Emit(_) => {}
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.viewbox, self.content)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.viewbox.render()?;
        self.content.render()
    }
}

impl ViewboxSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.viewbox.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 32.0,
        ))?;
        self.viewbox.set_size(Size::new(240.0, 120.0))?;
        Ok(())
    }
}
