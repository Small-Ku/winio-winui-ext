use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    BreadcrumbBar, BreadcrumbBarEvent, BreadcrumbBarItemDescriptor, ToggleButtonEvent,
};

pub struct BreadcrumbBarSample {
    page: GalleryPage,
    card: GalleryExample,
    breadcrumb: Child<BreadcrumbBar>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for BreadcrumbBarSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let page = GalleryPage::new(init.parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "BreadcrumbBar",
            "A native breadcrumb path backed by a WinUI IBindableIterable and stable descriptor IDs.",
            "apps/gallery/src/samples/breadcrumb_bar.rs · winio_winui_ext::BreadcrumbBar",
            include_str!("breadcrumb_bar.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("path:root")?;

        let mut breadcrumb = Child::<BreadcrumbBar>::init(card.as_container()).await?;
        breadcrumb.set_items(&[
            BreadcrumbBarItemDescriptor::new("root", "Root"),
            BreadcrumbBarItemDescriptor::new("projects", "Projects"),
            BreadcrumbBarItemDescriptor::new("details", "Details"),
        ])?;

        Ok(Self {
            page,
            card,
            breadcrumb,
            viewport: Rect::zero(),
            source_expanded: false,
            generation: init.generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card = self.card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => {
                ToggleButtonEvent::StateChanged(value) =>
                    GallerySampleMessage::SourceExpanded {
                        card,
                        expanded: value == Some(true),
                    },
            },
            self.breadcrumb => {
                BreadcrumbBarEvent::ItemClicked { id, .. } =>
                    GallerySampleMessage::Emit(format!("path:{id}")),
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
            GallerySampleMessage::Emit(value) if value.starts_with("path:") => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "breadcrumb-bar",
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
        self.breadcrumb.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.breadcrumb.render()
    }
}

impl BreadcrumbBarSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.breadcrumb.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 36.0,
        ))?;
        self.breadcrumb
            .set_size(Size::new((layout.primary.size.width - 40.0).max(0.0), 52.0))?;
        Ok(())
    }
}
