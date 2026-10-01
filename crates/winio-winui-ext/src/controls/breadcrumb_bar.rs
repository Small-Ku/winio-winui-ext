use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use windows::Foundation::{PropertyValue, TypedEventHandler};
use windows::core::{HRESULT, HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    BreadcrumbBar as NativeBreadcrumbBar, BreadcrumbBarItemClickedEventArgs,
};

use crate::{bindable, native::NativeElement};

const E_INVALIDARG: HRESULT = HRESULT(0x80070057u32 as i32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbBarItemDescriptor {
    pub id: String,
    pub text: String,
}

impl BreadcrumbBarItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug)]
pub struct BreadcrumbBar {
    native: NativeBreadcrumbBar,
    element: NativeElement,
    descriptors: Vec<BreadcrumbBarItemDescriptor>,
    items_source: Option<IInspectable>,
    event_ids: Arc<RwLock<Vec<String>>>,
    item_clicked_token: i64,
}

impl Failable for BreadcrumbBar {
    type Error = Error;
}

impl BreadcrumbBar {
    /// Replaces path items. IDs must be non-empty and unique because click events use them as stable identity.
    pub fn set_items(&mut self, descriptors: &[BreadcrumbBarItemDescriptor]) -> Result<()> {
        let mut ids = HashSet::with_capacity(descriptors.len());
        if descriptors
            .iter()
            .any(|item| item.id.is_empty() || !ids.insert(item.id.as_str()))
        {
            return Err(Error::new(
                E_INVALIDARG,
                "BreadcrumbBar item IDs must be non-empty and unique",
            ));
        }
        let values = descriptors
            .iter()
            .map(|descriptor| PropertyValue::CreateString(&HSTRING::from(descriptor.text.as_str())))
            .collect::<windows::core::Result<Vec<_>>>()?;
        let source = bindable::iterable(values)?;
        self.native.SetItemsSource(&source)?;

        self.descriptors = descriptors.to_vec();
        *self
            .event_ids
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) =
            descriptors.iter().map(|item| item.id.clone()).collect();
        self.items_source = Some(source);
        Ok(())
    }

    pub fn items(&self) -> &[BreadcrumbBarItemDescriptor] {
        &self.descriptors
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for BreadcrumbBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for BreadcrumbBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for BreadcrumbBar {
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum BreadcrumbBarEvent {
    ItemClicked { index: usize, id: String },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum BreadcrumbBarMessage {}

impl Component for BreadcrumbBar {
    type Error = Error;
    type Event = BreadcrumbBarEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = BreadcrumbBarMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeBreadcrumbBar::new()?;
        let event_ids = Arc::new(RwLock::new(Vec::<String>::new()));
        let click_ids = event_ids.clone();
        let event_sender = sender.clone();
        let item_clicked_token = native.ItemClicked(&TypedEventHandler::<
            NativeBreadcrumbBar,
            BreadcrumbBarItemClickedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let Ok(index) = usize::try_from(args.Index()?)
                && let Some(id) = click_ids
                    .read()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .get(index)
                    .cloned()
            {
                event_sender.output(BreadcrumbBarEvent::ItemClicked { index, id });
            }
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            descriptors: Vec::new(),
            items_source: None,
            event_ids,
            item_clicked_token,
        })
    }
}

impl AsWidget for BreadcrumbBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for BreadcrumbBar {
    fn drop(&mut self) {
        let _ = self.native.RemoveItemClicked(self.item_clicked_token);
    }
}
