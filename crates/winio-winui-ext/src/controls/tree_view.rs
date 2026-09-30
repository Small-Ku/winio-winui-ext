use windows::Foundation::{IPropertyValue, PropertyValue, TypedEventHandler};
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    TreeView as NativeTreeView, TreeViewExpandingEventArgs, TreeViewItemInvokedEventArgs,
    TreeViewNode, TreeViewSelectionMode,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

/// One node in a [`TreeView`], with stable identity and visible text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeViewNodeDescriptor {
    pub id: String,
    pub text: String,
    pub children: Vec<Self>,
}

impl TreeViewNodeDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = Self>) -> Self {
        self.children.extend(children);
        self
    }
}

/// A native WinUI `TreeView` hosted as a first-class Winio component.
#[derive(Debug)]
pub struct TreeView {
    native: NativeTreeView,
    element: NativeElement,
    item_invoked_token: i64,
    expanding_token: i64,
}

impl Failable for TreeView {
    type Error = Error;
}

impl TreeView {
    pub fn set_nodes(&mut self, nodes: &[TreeViewNodeDescriptor]) -> Result<()> {
        let roots = self.native.RootNodes()?;
        roots.Clear()?;
        for node in nodes {
            roots.Append(&build_node(node)?)?;
        }
        Ok(())
    }
}

fn build_node(descriptor: &TreeViewNodeDescriptor) -> Result<TreeViewNode> {
    let node = TreeViewNode::new()?;
    let text = MUXC::TextBlock::new()?;
    text.SetText(&HSTRING::from(descriptor.text.as_str()))?;
    let id = PropertyValue::CreateString(&HSTRING::from(descriptor.id.as_str()))?;
    text.SetTag(&id)?;
    node.SetContent(&text)?;
    let children = node.Children()?;
    for child in &descriptor.children {
        children.Append(&build_node(child)?)?;
    }
    Ok(node)
}

fn tagged_text_id(item: &IInspectable) -> Option<String> {
    item.cast::<MUXC::TextBlock>()
        .ok()
        .and_then(|text| text.Tag().ok())
        .and_then(|tag| tag.cast::<IPropertyValue>().ok())
        .and_then(|tag| tag.GetString().ok())
        .map(|id| id.to_string_lossy())
}

fn tree_node_id(node: &TreeViewNode) -> Option<String> {
    node.Content()
        .ok()
        .and_then(|content| tagged_text_id(&content))
}

fn invoked_item_id(item: &IInspectable) -> Option<String> {
    tagged_text_id(item).or_else(|| {
        item.cast::<TreeViewNode>()
            .ok()
            .and_then(|node| tree_node_id(&node))
    })
}

impl Visible for TreeView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for TreeView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for TreeView {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }

    fn set_loc(&mut self, point: Point) -> Result<()> {
        self.element.set_loc(point)
    }

    fn size(&self) -> Result<Size> {
        self.element.size()
    }

    fn set_size(&mut self, size: Size) -> Result<()> {
        self.element.set_size(size)
    }

    fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TreeViewEvent {
    ItemInvoked { id: Option<String> },
    Expanded { id: Option<String> },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TreeViewMessage {}

impl Component for TreeView {
    type Error = Error;
    type Event = TreeViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = TreeViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeTreeView::new()?;
        native.SetSelectionMode(TreeViewSelectionMode::Single)?;

        let invoked_sender = sender.clone();
        let item_invoked_token = native.ItemInvoked(&TypedEventHandler::<
            NativeTreeView,
            TreeViewItemInvokedEventArgs,
        >::new(move |_, args| {
            let id = args
                .as_ref()
                .and_then(|args| args.InvokedItem().ok())
                .and_then(|item| invoked_item_id(&item));
            invoked_sender.output(TreeViewEvent::ItemInvoked { id });
            Ok(())
        }))?;

        let expanding_sender = sender.clone();
        let expanding_token = native.Expanding(&TypedEventHandler::<
            NativeTreeView,
            TreeViewExpandingEventArgs,
        >::new(move |_, args| {
            let id = args
                .as_ref()
                .and_then(|args| args.Node().ok())
                .and_then(|node| tree_node_id(&node));
            expanding_sender.output(TreeViewEvent::Expanded { id });
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self {
            native,
            element,
            item_invoked_token,
            expanding_token,
        })
    }
}

impl AsWidget for TreeView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for TreeView {
    fn drop(&mut self) {
        let _ = self.native.RemoveItemInvoked(self.item_invoked_token);
        let _ = self.native.RemoveExpanding(self.expanding_token);
    }
}
