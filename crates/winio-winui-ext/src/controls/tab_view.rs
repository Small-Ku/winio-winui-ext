use std::collections::HashSet;

use windows::core::{HRESULT, HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Automation::AutomationProperties;
use winio_winui3::Microsoft::UI::Xaml::Controls::{
    self as MUXC, SelectionChangedEventHandler, TabView as NativeTabView,
    TabViewItem as NativeTabViewItem,
};

use crate::native::NativeElement;

const E_INVALIDARG: HRESULT = HRESULT(0x80070057u32 as i32);

/// One tab whose stable `id` is independent of its visible `title`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabViewTabDescriptor {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl TabViewTabDescriptor {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
        }
    }
}

/// A native WinUI TabView with stable descriptor-backed tab identities.
#[derive(Debug)]
pub struct TabView {
    native: NativeTabView,
    element: NativeElement,
    tabs: Vec<TabViewTabDescriptor>,
    items: Vec<NativeTabViewItem>,
    selection_changed_token: i64,
}

impl Failable for TabView {
    type Error = Error;
}

impl TabView {
    /// Replaces tabs and preserves selection by stable ID where possible.
    /// Duplicate or empty IDs are rejected with `E_INVALIDARG`.
    pub fn set_tabs(&mut self, tabs: &[TabViewTabDescriptor]) -> Result<()> {
        let mut ids = HashSet::with_capacity(tabs.len());
        if tabs
            .iter()
            .any(|tab| tab.id.is_empty() || !ids.insert(tab.id.as_str()))
        {
            return Err(Error::new(
                E_INVALIDARG,
                "TabView tab IDs must be non-empty and unique",
            ));
        }
        let previous = self.selected_id()?.map(ToOwned::to_owned);
        let native_tabs = self.native.TabItems()?;
        native_tabs.Clear()?;
        let mut items = Vec::with_capacity(tabs.len());
        for tab in tabs {
            let item = NativeTabViewItem::new()?;
            let header = MUXC::TextBlock::new()?;
            header.SetText(&HSTRING::from(tab.title.as_str()))?;
            item.SetHeader(&header)?;
            let content = MUXC::TextBlock::new()?;
            content.SetText(&HSTRING::from(tab.content.as_str()))?;
            item.SetContent(&content)?;
            let framework = item.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
            framework.SetTag(&windows::Foundation::PropertyValue::CreateString(
                &HSTRING::from(tab.id.as_str()),
            )?)?;
            AutomationProperties::SetAutomationId(
                &item,
                &HSTRING::from(format!("tab-{}", tab.id)),
            )?;
            native_tabs.Append(&item)?;
            items.push(item);
        }
        self.tabs = tabs.to_vec();
        self.items = items;
        let selected = previous
            .and_then(|id| self.tabs.iter().position(|tab| tab.id == id))
            .or_else(|| (!self.tabs.is_empty()).then_some(0));
        self.native
            .SetSelectedIndex(selected.map_or(-1, |index| index as i32))?;
        Ok(())
    }

    pub fn selected_index(&self) -> Result<i32> {
        self.native.SelectedIndex()
    }

    /// Selects a tab by index. `-1` clears selection; any other index must exist.
    pub fn set_selected_index(&mut self, index: i32) -> Result<()> {
        if index != -1
            && usize::try_from(index)
                .ok()
                .is_none_or(|i| i >= self.tabs.len())
        {
            return Err(Error::new(
                E_INVALIDARG,
                "TabView selected index is out of range",
            ));
        }
        self.native.SetSelectedIndex(index)?;
        Ok(())
    }

    pub fn selected_id(&self) -> Result<Option<&str>> {
        Ok(usize::try_from(self.selected_index()?)
            .ok()
            .and_then(|index| self.tabs.get(index))
            .map(|tab| tab.id.as_str()))
    }
}

impl Visible for TabView {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, value: bool) -> Result<()> {
        self.element.set_visible(value)
    }
}
impl Enable for TabView {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, value: bool) -> Result<()> {
        self.element.set_enabled(value)
    }
}
impl Layoutable for TabView {
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
pub enum TabViewEvent {
    SelectionChanged {
        selected_index: i32,
        selected_id: Option<String>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TabViewMessage {}

impl Component for TabView {
    type Error = Error;
    type Event = TabViewEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = TabViewMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeTabView::new()?;
        let event_native = native.clone();
        let event_sender = sender.clone();
        let selection_changed_token =
            native.SelectionChanged(&SelectionChangedEventHandler::new(move |_, _| {
                let selected_index = event_native.SelectedIndex().unwrap_or(-1);
                let selected_id = usize::try_from(selected_index)
                    .ok()
                    .and_then(|index| event_native.TabItems().ok()?.GetAt(index as u32).ok())
                    .and_then(|item| item.cast::<NativeTabViewItem>().ok())
                    .and_then(|item| {
                        item.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()
                            .ok()
                    })
                    .and_then(|item| item.Tag().ok())
                    .and_then(|tag| tag.cast::<windows::Foundation::IPropertyValue>().ok())
                    .and_then(|tag| tag.GetString().ok())
                    .map(|id| id.to_string_lossy());
                event_sender.output(TabViewEvent::SelectionChanged {
                    selected_index,
                    selected_id,
                });
                Ok(())
            }))?;
        let element = NativeElement::new(
            parent,
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?,
        )?;
        Ok(Self {
            native,
            element,
            tabs: Vec::new(),
            items: Vec::new(),
            selection_changed_token,
        })
    }
}

impl AsWidget for TabView {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for TabView {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectionChanged(self.selection_changed_token);
    }
}
