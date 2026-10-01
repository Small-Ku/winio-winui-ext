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
    ItemsView as NativeItemsView, ItemsViewSelectionChangedEventArgs, ItemsViewSelectionMode,
};
use winio_winui3::Microsoft::UI::Xaml as MUX;

use crate::{bindable, native::NativeElement};

const E_INVALIDARG: HRESULT = HRESULT(0x80070057u32 as i32);

/// One ItemsView data item. Its stable ID is independent of the visible text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemsViewItemDescriptor {
    pub id: String,
    pub text: String,
}

impl ItemsViewItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

/// A single-selection native ItemsView using its default StackLayout and string item presentation.
#[derive(Debug)]
pub struct ItemsView {
    native: NativeItemsView,
    element: NativeElement,
    items: Vec<ItemsViewItemDescriptor>,
    event_ids: Arc<RwLock<Vec<String>>>,
    items_source: Option<IInspectable>,
    selection_changed_token: i64,
}

impl Failable for ItemsView {
    type Error = Error;
}

impl ItemsView {
    /// Replaces the source and preserves selection by stable ID when that ID still exists.
    /// IDs must be non-empty and unique. The native default item template renders `text` as a string.
    pub fn set_items(&mut self, items: &[ItemsViewItemDescriptor]) -> Result<()> {
        let mut ids = HashSet::with_capacity(items.len());
        if items
            .iter()
            .any(|item| item.id.is_empty() || !ids.insert(item.id.as_str()))
        {
            return Err(Error::new(
                E_INVALIDARG,
                "ItemsView item IDs must be non-empty and unique",
            ));
        }

        let previous_id = self.selected_id()?.map(ToOwned::to_owned);
        let values = items
            .iter()
            .map(|item| {
                PropertyValue::CreateString(&HSTRING::from(item.text.as_str()))?
                    .cast::<IInspectable>()
            })
            .collect::<windows::core::Result<Vec<_>>>()?;
        let source = bindable::iterable(values)?;

        let next_ids = items.iter().map(|item| item.id.clone()).collect::<Vec<_>>();
        let old_ids = {
            let mut event_ids = self
                .event_ids
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            std::mem::replace(&mut *event_ids, next_ids)
        };
        if let Err(error) = self.native.SetItemsSource(&source) {
            *self
                .event_ids
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner()) = old_ids;
            return Err(error);
        }

        self.items = items.to_vec();
        self.items_source = Some(source);
        if let Some(index) = previous_id
            .as_deref()
            .and_then(|id| self.items.iter().position(|item| item.id == id))
        {
            self.native.Select(index as i32)?;
        } else {
            self.native.DeselectAll()?;
        }
        Ok(())
    }

    pub fn items(&self) -> &[ItemsViewItemDescriptor] {
        &self.items
    }

    /// Returns the selected source index, or `-1` when selection is empty.
    pub fn selected_index(&self) -> Result<i32> {
        for index in 0..self.items.len() {
            let index = i32::try_from(index)
                .map_err(|_| Error::new(E_INVALIDARG, "ItemsView item index exceeds i32"))?;
            if self.native.IsSelected(index)? {
                return Ok(index);
            }
        }
        Ok(-1)
    }

    pub fn selected_id(&self) -> Result<Option<&str>> {
        Ok(usize::try_from(self.selected_index()?)
            .ok()
            .and_then(|index| self.items.get(index))
            .map(|item| item.id.as_str()))
    }

    /// Selects an item by index. `-1` clears selection; other invalid indexes return `E_INVALIDARG`.
    pub fn set_selected_index(&mut self, index: i32) -> Result<()> {
        if index == -1 {
            self.native.DeselectAll()?;
        } else if usize::try_from(index)
            .ok()
            .is_some_and(|index| index < self.items.len())
        {
            self.native.DeselectAll()?;
            self.native.Select(index)?;
        } else {
            return Err(Error::new(
                E_INVALIDARG,
                "ItemsView selected index is out of range",
            ));
        }
        Ok(())
    }
}

fn selected_info(native: &NativeItemsView, ids: &[String]) -> (i32, Option<String>) {
    ids.iter()
        .enumerate()
        .find_map(|(index, id)| {
            let index = i32::try_from(index).ok()?;
            native
                .IsSelected(index)
                .ok()
                .filter(|selected| *selected)
                .map(|_| (index, Some(id.clone())))
        })
        .unwrap_or((-1, None))
}

impl Visible for ItemsView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, value: bool) -> Result<()> {
        self.element.set_visible(value)
    }
}
impl Enable for ItemsView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, value: bool) -> Result<()> {
        self.element.set_enabled(value)
    }
}
impl Layoutable for ItemsView {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }
    fn set_loc(&mut self, value: Point) -> Result<()> {
        self.element.set_loc(value)
    }
    fn size(&self) -> Result<Size> {
        self.element.size()
    }
    fn set_size(&mut self, value: Size) -> Result<()> {
        self.element.set_size(value)
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
pub enum ItemsViewEvent {
    SelectionChanged {
        selected_index: i32,
        selected_id: Option<String>,
    },
}
#[derive(Debug)]
#[non_exhaustive]
pub enum ItemsViewMessage {}

impl Component for ItemsView {
    type Error = Error;
    type Event = ItemsViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ItemsViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeItemsView::new()?;
        native.SetSelectionMode(ItemsViewSelectionMode::Single)?;
        let event_ids = Arc::new(RwLock::new(Vec::new()));
        let event_native = native.clone();
        let event_ids_clone = Arc::clone(&event_ids);
        let event_sender = sender.clone();
        let token = native.SelectionChanged(&TypedEventHandler::<
            NativeItemsView,
            ItemsViewSelectionChangedEventArgs,
        >::new(move |_, _| {
            let ids = event_ids_clone
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let (selected_index, selected_id) = selected_info(&event_native, &ids);
            event_sender.output(ItemsViewEvent::SelectionChanged {
                selected_index,
                selected_id,
            });
            Ok(())
        }))?;
        let element = NativeElement::new(parent, native.cast::<MUX::FrameworkElement>()?)?;
        Ok(Self {
            native,
            element,
            items: Vec::new(),
            event_ids,
            items_source: None,
            selection_changed_token: token,
        })
    }
}

impl AsWidget for ItemsView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
impl Drop for ItemsView {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectionChanged(self.selection_changed_token);
    }
}
