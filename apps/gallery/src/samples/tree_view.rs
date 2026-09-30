use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ToggleButtonEvent, TreeView, TreeViewEvent, TreeViewNodeDescriptor};

pub struct TreeViewSample {
    page: GalleryPage,
    card: GalleryExample,
    tree: Child<TreeView>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}
impl Component for TreeViewSample {
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
            "TreeView",
            "A hierarchical native view with stable node IDs.",
            "apps/gallery/src/samples/tree_view.rs · winio_winui_ext::TreeView",
            include_str!("tree_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("No tree event yet")?;
        init! { tree: TreeView = (&card) => { size: Size::new(520.0, 124.0) } }
        tree.set_nodes(&[TreeViewNodeDescriptor::new("root", "Root node")
            .with_children([TreeViewNodeDescriptor::new("leaf", "Leaf node")])])?;
        Ok(Self {
            page,
            card,
            tree,
            viewport: Rect::zero(),
            source_expanded: false,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! { sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded { card: card_id, expanded: state == Some(true) }, },
            self.tree => { TreeViewEvent::Expanded { id } => GallerySampleMessage::Emit(format!("expanded: {}", id.unwrap_or_default())), TreeViewEvent::ItemInvoked { id } => GallerySampleMessage::Emit(format!("invoked: {}", id.unwrap_or_default())), },
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
                    sample_id: "tree-view",
                    value,
                });
            }
            _ => {}
        }
        Ok(false)
    }
    #[allow(clippy::needless_question_mark)]
    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.tree)
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.tree.render()
    }
}
impl TreeViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.tree.set_loc(Point::new(
            layout.primary.origin.x + 12.0,
            layout.primary.origin.y + 28.0,
        ))?;
        self.tree.set_size(Size::new(
            (layout.primary.size.width - 24.0).max(0.0),
            (layout.primary.size.height - 40.0).max(80.0),
        ))?;
        Ok(())
    }
}
