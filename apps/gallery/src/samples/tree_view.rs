use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};
use winio::prelude::*;
use winio_winui_ext::{TreeView, TreeViewEvent, TreeViewNodeDescriptor};
pub struct TreeViewSample {
    root: Child<View>,
    tree: Child<TreeView>,
    generation: u64,
}
impl Component for TreeViewSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;
    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        init! {root:View=(parent),tree:TreeView=(&root)=>{size:Size::new(520.0,230.0)}}
        tree.set_nodes(&[TreeViewNodeDescriptor::new("root", "Root node")
            .with_children([TreeViewNodeDescriptor::new("leaf", "Leaf node")])])?;
        Ok(Self {
            root,
            tree,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {sender,default:GallerySampleMessage::Emit(String::new()),self.root=>{},self.tree=>{TreeViewEvent::Expanded{id}=>GallerySampleMessage::Emit(format!("expanded: {}",id.unwrap_or_default())),TreeViewEvent::ItemInvoked{id}=>GallerySampleMessage::Emit(format!("invoked: {}",id.unwrap_or_default())),}}
    }
    async fn update(&mut self, m: Self::Message, s: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(r) => {
                self.root.set_rect(r)?;
                self.tree.set_loc(Point::new(24.0, 24.0))?;
                self.tree
                    .set_size(Size::new((r.size.width - 48.0).max(0.0), 230.0))?;
            }
            GallerySampleMessage::Emit(v) => s.output(GallerySampleEvent::Output {
                generation: self.generation,
                sample_id: "tree-view",
                value: v,
            }),
            GallerySampleMessage::OpenTeachingTip => {}
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.root, self.tree)
    }
    fn render_children(&mut self) -> Result<()> {
        self.root.render()?;
        self.tree.render()
    }
}
