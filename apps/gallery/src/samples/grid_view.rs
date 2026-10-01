use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{GridView, GridViewEvent, GridViewItemDescriptor, ToggleButtonEvent};

pub struct GridViewSample {
    page: GalleryPage,
    card: GalleryExample,
    grid: Child<GridView>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_selection: bool,
    generation: u64,
}

impl Component for GridViewSample {
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
            "GridView",
            "A native tile selector with owned UIElement items and stable descriptor IDs.",
            "apps/gallery/src/samples/grid_view.rs · winio_winui_ext::GridView",
            include_str!("grid_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("tile:first")?;
        let mut grid = Child::<GridView>::init(card.as_container()).await?;
        grid.set_items(&[
            GridViewItemDescriptor::new("first", "First tile"),
            GridViewItemDescriptor::new("second", "Second tile"),
            GridViewItemDescriptor::new("third", "Third tile"),
        ])?;
        grid.set_selected_index(0)?;
        Ok(Self {
            page,
            card,
            grid,
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
            self.grid => {
                GridViewEvent::SelectionChanged { selected_id, .. } =>
                    GallerySampleMessage::Emit(format!("tile:{}", selected_id.unwrap_or_default())),
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
            GallerySampleMessage::Emit(value) if value.starts_with("tile:") => {
                if self.skip_initial_selection && value == "tile:first" {
                    self.skip_initial_selection = false;
                    return Ok(false);
                }
                self.skip_initial_selection = false;
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "grid-view",
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
        self.grid.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.grid.render()
    }
}

impl GridViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.grid.set_loc(Point::new(
            layout.primary.origin.x + 12.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.grid.set_size(Size::new(
            (layout.primary.size.width - 24.0).max(0.0),
            176.0,
        ))?;
        Ok(())
    }
}
