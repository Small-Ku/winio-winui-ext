use crate::{
    chrome::{CardLayout, GalleryExample, GalleryExampleEvent, GalleryExampleInit},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{TreeView, TreeViewEvent, TreeViewNodeDescriptor};

pub struct TreeViewSample {
    example: Child<GalleryExample>,
    tree: Child<TreeView>,
    viewport: Rect,
    layout: CardLayout,
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
        let mut example = Child::<GalleryExample>::init(GalleryExampleInit {
            parent,
            title: "TreeView",
            description: "A hierarchical native view with stable node IDs.",
            source_path: "apps/gallery/src/samples/tree_view.rs · winio_winui_ext::TreeView",
            source: include_str!("tree_view.rs"),
            has_output: true,
            has_options: false,
        })
        .await?;
        example.set_output("No tree event yet")?;
        init! { tree: TreeView = (&example) => { size: Size::new(520.0, 124.0) } }
        tree.set_nodes(&[TreeViewNodeDescriptor::new("root", "Root node")
            .with_children([TreeViewNodeDescriptor::new("leaf", "Leaf node")])])?;
        let layout = CardLayout::calculate(
            800.0,
            true,
            false,
            false,
            include_str!("tree_view.rs").len(),
        );
        Ok(Self {
            example,
            tree,
            viewport: Rect::zero(),
            layout,
            source_expanded: false,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.example => { GalleryExampleEvent::SourceExpanded(expanded) => GallerySampleMessage::SourceExpanded(expanded), },
            self.tree => {
                TreeViewEvent::Expanded { id } => GallerySampleMessage::Emit(format!("expanded: {}", id.unwrap_or_default())),
                TreeViewEvent::ItemInvoked { id } => GallerySampleMessage::Emit(format!("invoked: {}", id.unwrap_or_default())),
            },
        }
    }

    async fn update(&mut self, m: Self::Message, s: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(viewport) => {
                self.viewport = viewport;
                self.layout = CardLayout::calculate(
                    viewport.size.width,
                    true,
                    false,
                    self.source_expanded,
                    include_str!("tree_view.rs").len(),
                );
                self.example.set_view(viewport, self.layout)?;
                self.place_tree()?;
            }
            GallerySampleMessage::SourceExpanded(expanded) => {
                self.source_expanded = expanded;
                self.layout = CardLayout::calculate(
                    self.viewport.size.width,
                    true,
                    false,
                    expanded,
                    include_str!("tree_view.rs").len(),
                );
                self.example.set_view(self.viewport, self.layout)?;
                self.place_tree()?;
            }
            GallerySampleMessage::Emit(value) => {
                if !value.is_empty() {
                    self.example.set_output(&value)?;
                    s.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "tree-view",
                        value,
                    });
                }
            }
            GallerySampleMessage::OpenTeachingTip
            | GallerySampleMessage::ToggleThreeState(_)
            | GallerySampleMessage::NumberStepOption(_) => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.example, self.tree)
    }
    fn render_children(&mut self) -> Result<()> {
        self.example.render()?;
        self.tree.render()
    }
}

impl TreeViewSample {
    fn place_tree(&mut self) -> Result<()> {
        let primary = self.layout.primary;
        self.tree
            .set_loc(Point::new(primary.origin.x + 12.0, primary.origin.y + 28.0))?;
        self.tree.set_size(Size::new(
            (primary.size.width - 24.0).max(0.0),
            (primary.size.height - 40.0).max(80.0),
        ))?;
        Ok(())
    }
}
