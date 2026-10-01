use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ItemsView, ItemsViewEvent, ItemsViewItemDescriptor, ToggleButtonEvent};

pub struct ItemsViewSample {
    page: GalleryPage,
    card: GalleryExample,
    items_view: Child<ItemsView>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_selection: bool,
    generation: u64,
}

impl Component for ItemsViewSample {
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
            "ItemsView",
            "A native ItemsView using its default StackLayout and item string presentation.",
            "apps/gallery/src/samples/items_view.rs · winio_winui_ext::ItemsView",
            include_str!("items_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("selected:first")?;
        let mut items_view = Child::<ItemsView>::init(card.as_container()).await?;
        items_view.set_items(&[
            ItemsViewItemDescriptor::new("first", "First entry"),
            ItemsViewItemDescriptor::new("second", "Second entry"),
            ItemsViewItemDescriptor::new("third", "Third entry"),
        ])?;
        items_view.set_selected_index(0)?;
        Ok(Self {
            page,
            card,
            items_view,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_selection: true,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded {
                    card: card_id, expanded: state == Some(true),
                },
            },
            self.items_view => {
                ItemsViewEvent::SelectionChanged { selected_id, .. } =>
                    GallerySampleMessage::Emit(format!("selected:{}", selected_id.unwrap_or_else(|| "none".to_owned()))),
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
            GallerySampleMessage::Emit(value) if value.starts_with("selected:") => {
                if self.skip_initial_selection {
                    if value == "selected:none" {
                        return Ok(false);
                    }
                    if value == "selected:first" {
                        self.skip_initial_selection = false;
                        return Ok(false);
                    }
                }
                self.skip_initial_selection = false;
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "items-view",
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
        self.items_view.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.items_view.render()
    }
}

impl ItemsViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.items_view.set_loc(Point::new(
            layout.primary.origin.x + 12.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.items_view.set_size(Size::new(
            (layout.primary.size.width - 24.0).max(0.0),
            200.0,
        ))?;
        Ok(())
    }
}
