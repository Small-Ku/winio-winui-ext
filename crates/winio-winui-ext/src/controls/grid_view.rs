use std::collections::HashSet;

use windows::Foundation::PropertyValue;
use windows::core::{HRESULT, HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::{
    Automation::AutomationProperties,
    Controls::{GridView as NativeGridView, GridViewItem, ListViewSelectionMode},
};
use winio_winui3::Microsoft::UI::Xaml::{
    self as MUX,
    Controls::{self as MUXC, Primitives::Selector, SelectionChangedEventHandler},
};

use crate::{bindable, native::NativeElement};

const E_INVALIDARG: HRESULT = HRESULT(0x80070057u32 as i32);

/// One stable tile in a [`GridView`]. The ID is independent of its visible label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridViewItemDescriptor {
    pub id: String,
    pub text: String,
}

impl GridViewItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

/// A native WinUI GridView backed by an owned, UIElement-valued IBindableIterable.
#[derive(Debug)]
pub struct GridView {
    native: NativeGridView,
    selector: Selector,
    element: NativeElement,
    items: Vec<GridViewItemDescriptor>,
    native_items: Vec<GridViewItem>,
    items_source: Option<IInspectable>,
    selection_changed_token: i64,
}

impl Failable for GridView {
    type Error = Error;
}

impl GridView {
    /// Replaces tiles and preserves selection only when its stable ID remains present.
    /// A cleared selection or a removed selected ID stays unselected. Empty or duplicate IDs
    /// are rejected; `set_selected_index` accepts only `-1` to clear.
    pub fn set_items(&mut self, items: &[GridViewItemDescriptor]) -> Result<()> {
        let mut ids = HashSet::with_capacity(items.len());
        if items
            .iter()
            .any(|item| item.id.is_empty() || !ids.insert(item.id.as_str()))
        {
            return Err(Error::new(
                E_INVALIDARG,
                "GridView item IDs must be non-empty and unique",
            ));
        }
        let previous_id = self.selected_id()?.map(ToOwned::to_owned);
        let mut native_items = Vec::with_capacity(items.len());
        let mut inspectables = Vec::with_capacity(items.len());
        for descriptor in items {
            let item = GridViewItem::new()?;
            let text = MUXC::TextBlock::new()?;
            text.SetText(&HSTRING::from(descriptor.text.as_str()))?;
            item.cast::<MUXC::ContentControl>()?.SetContent(&text)?;
            let framework = item.cast::<MUX::FrameworkElement>()?;
            framework.SetTag(&PropertyValue::CreateString(&HSTRING::from(
                descriptor.id.as_str(),
            ))?)?;
            AutomationProperties::SetAutomationId(
                &item,
                &HSTRING::from(format!("grid-view-{}", descriptor.id)),
            )?;
            inspectables.push(item.cast::<IInspectable>()?);
            native_items.push(item);
        }
        let source = bindable::iterable(inspectables)?;
        self.native
            .cast::<MUXC::ItemsControl>()?
            .SetItemsSource(&source)?;
        self.items = items.to_vec();
        self.native_items = native_items;
        self.items_source = Some(source);
        let selected_index = selected_index_for_id(previous_id.as_deref(), &self.items);
        self.selector.SetSelectedIndex(selected_index)?;
        Ok(())
    }

    pub fn items(&self) -> &[GridViewItemDescriptor] {
        &self.items
    }
    pub fn selected_index(&self) -> Result<i32> {
        self.selector.SelectedIndex()
    }
    pub fn selected_id(&self) -> Result<Option<&str>> {
        Ok(usize::try_from(self.selected_index()?)
            .ok()
            .and_then(|i| self.items.get(i))
            .map(|item| item.id.as_str()))
    }
    pub fn set_selected_index(&mut self, index: i32) -> Result<()> {
        if index != -1
            && usize::try_from(index)
                .ok()
                .is_none_or(|i| i >= self.items.len())
        {
            return Err(Error::new(
                E_INVALIDARG,
                "GridView selected index is out of range",
            ));
        }
        self.selector.SetSelectedIndex(index)?;
        Ok(())
    }
}

impl Visible for GridView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, value: bool) -> Result<()> {
        self.element.set_visible(value)
    }
}
impl Enable for GridView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, value: bool) -> Result<()> {
        self.element.set_enabled(value)
    }
}
impl Layoutable for GridView {
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
pub enum GridViewEvent {
    SelectionChanged {
        selected_index: i32,
        selected_id: Option<String>,
    },
}
#[derive(Debug)]
#[non_exhaustive]
pub enum GridViewMessage {}

impl Component for GridView {
    type Error = Error;
    type Event = GridViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = GridViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeGridView::new()?;
        let selector = native.cast::<Selector>()?;
        native.SetSelectionMode(ListViewSelectionMode::Single)?;
        let event_native = native.clone();
        let event_sender = sender.clone();
        let token =
            selector.SelectionChanged(&SelectionChangedEventHandler::new(move |_, _| {
                let index = event_native
                    .cast::<Selector>()
                    .and_then(|s| s.SelectedIndex())
                    .unwrap_or(-1);
                let id = event_native
                    .cast::<Selector>()
                    .ok()
                    .and_then(|s| s.SelectedItem().ok())
                    .and_then(|item| item.cast::<MUX::FrameworkElement>().ok())
                    .and_then(|item| item.Tag().ok())
                    .and_then(|tag| tag.cast::<windows::Foundation::IPropertyValue>().ok())
                    .and_then(|tag| tag.GetString().ok())
                    .map(|id| id.to_string_lossy());
                event_sender.output(GridViewEvent::SelectionChanged {
                    selected_index: index,
                    selected_id: id,
                });
                Ok(())
            }))?;
        let element = NativeElement::new(parent, native.cast::<MUX::FrameworkElement>()?)?;
        Ok(Self {
            native,
            selector,
            element,
            items: Vec::new(),
            native_items: Vec::new(),
            items_source: None,
            selection_changed_token: token,
        })
    }
}
impl AsWidget for GridView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
impl Drop for GridView {
    fn drop(&mut self) {
        let _ = self
            .selector
            .RemoveSelectionChanged(self.selection_changed_token);
    }
}

fn selected_index_for_id(previous_id: Option<&str>, items: &[GridViewItemDescriptor]) -> i32 {
    previous_id
        .and_then(|id| items.iter().position(|item| item.id == id))
        .and_then(|index| i32::try_from(index).ok())
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items(ids: &[&str]) -> Vec<GridViewItemDescriptor> {
        ids.iter()
            .map(|id| GridViewItemDescriptor::new(*id, format!("label {id}")))
            .collect()
    }

    #[test]
    fn replacing_items_preserves_only_an_existing_selected_id() {
        let reordered = items(&["third", "first", "second"]);
        assert_eq!(selected_index_for_id(Some("second"), &reordered), 2);
        assert_eq!(selected_index_for_id(None, &reordered), -1);
        assert_eq!(selected_index_for_id(Some("removed"), &reordered), -1);
        assert_eq!(selected_index_for_id(Some("second"), &[]), -1);
    }
}
