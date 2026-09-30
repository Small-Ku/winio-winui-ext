use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use windows::Foundation::{IReference, PropertyValue, TypedEventHandler};
use windows::core::{HSTRING, IInspectable, IUnknown, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::{
    Automation::AutomationProperties,
    Controls::{
        TreeView as NativeTreeView, TreeViewExpandingEventArgs, TreeViewItemInvokedEventArgs,
        TreeViewNode, TreeViewSelectionMode,
    },
};

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
    content_ids: Arc<Mutex<HashMap<usize, String>>>,
}

impl Failable for TreeView {
    type Error = Error;
}

impl TreeView {
    pub fn set_nodes(&mut self, nodes: &[TreeViewNodeDescriptor]) -> Result<()> {
        let roots = self.native.RootNodes()?;
        roots.Clear()?;
        self.content_ids
            .lock()
            .expect("tree content id map poisoned")
            .clear();
        for node in nodes {
            roots.Append(&build_node(node, &self.content_ids)?)?;
        }
        Ok(())
    }
}

fn inspectable_identity(item: &IInspectable) -> Option<usize> {
    item.cast::<IUnknown>()
        .ok()
        .map(|identity| Interface::as_raw(&identity) as usize)
}

fn build_node(
    descriptor: &TreeViewNodeDescriptor,
    content_ids: &Arc<Mutex<HashMap<usize, String>>>,
) -> Result<TreeViewNode> {
    let node = TreeViewNode::new()?;
    let visible = HSTRING::from(descriptor.text.as_str());
    let content = PropertyValue::CreateString(&visible)?.cast::<IReference<HSTRING>>()?;
    let inspectable = content.cast::<IInspectable>()?;
    if let Some(identity) = inspectable_identity(&inspectable) {
        content_ids
            .lock()
            .expect("tree content id map poisoned")
            .insert(identity, descriptor.id.clone());
    }
    node.SetContent(&content)?;
    AutomationProperties::SetAutomationId(&node, &HSTRING::from(descriptor.id.as_str()))?;
    let children = node.Children()?;
    for child in &descriptor.children {
        children.Append(&build_node(child, content_ids)?)?;
    }
    Ok(node)
}

fn tree_node_id(node: &TreeViewNode) -> Option<String> {
    AutomationProperties::GetAutomationId(node)
        .ok()
        .map(|id| id.to_string_lossy())
        .filter(|id| !id.is_empty())
}

fn invoked_item_id(
    item: &IInspectable,
    content_ids: &Arc<Mutex<HashMap<usize, String>>>,
) -> Option<String> {
    item.cast::<TreeViewNode>()
        .ok()
        .and_then(|node| tree_node_id(&node))
        .or_else(|| {
            inspectable_identity(item).and_then(|identity| {
                content_ids
                    .lock()
                    .expect("tree content id map poisoned")
                    .get(&identity)
                    .cloned()
            })
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

        let content_ids = Arc::new(Mutex::new(HashMap::new()));
        let invoked_content_ids = Arc::clone(&content_ids);
        let invoked_sender = sender.clone();
        let item_invoked_token = native.ItemInvoked(&TypedEventHandler::<
            NativeTreeView,
            TreeViewItemInvokedEventArgs,
        >::new(move |_, args| {
            let id = args
                .as_ref()
                .and_then(|args| args.InvokedItem().ok())
                .and_then(|item| invoked_item_id(&item, &invoked_content_ids));
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
            content_ids,
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
