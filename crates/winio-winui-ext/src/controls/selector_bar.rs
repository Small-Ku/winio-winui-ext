use windows::Foundation::TypedEventHandler;
use windows::Foundation::{IPropertyValue, PropertyValue};
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Automation::AutomationProperties;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    SelectorBar as NativeSelectorBar, SelectorBarItem, SelectorBarSelectionChangedEventArgs,
};

use crate::native::NativeElement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorBarItemDescriptor {
    pub id: String,
    pub text: String,
}

impl SelectorBarItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug)]
pub struct SelectorBar {
    native: NativeSelectorBar,
    element: NativeElement,
    descriptors: Vec<SelectorBarItemDescriptor>,
    items: Vec<SelectorBarItem>,
    selection_changed_token: i64,
}

impl Failable for SelectorBar {
    type Error = Error;
}

impl SelectorBar {
    pub fn set_items(&mut self, descriptors: &[SelectorBarItemDescriptor]) -> Result<()> {
        let items = self.native.Items()?;
        items.Clear()?;
        let mut owned = Vec::with_capacity(descriptors.len());
        for descriptor in descriptors {
            let item = SelectorBarItem::new()?;
            item.SetText(&HSTRING::from(descriptor.text.as_str()))?;
            let id = PropertyValue::CreateString(&HSTRING::from(descriptor.id.as_str()))?;
            item.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?
                .SetTag(&id)?;
            AutomationProperties::SetAutomationId(
                &item,
                &HSTRING::from(format!("selector-{}", descriptor.id)),
            )?;
            items.Append(&item)?;
            owned.push(item);
        }
        self.descriptors = descriptors.to_vec();
        self.items = owned;
        if self.native.SelectedItem().is_err() && !self.items.is_empty() {
            self.native.SetSelectedItem(&self.items[0])?;
        }
        Ok(())
    }

    pub fn selected_index(&self) -> Result<i32> {
        let Ok(selected) = self.native.SelectedItem() else {
            return Ok(-1);
        };
        Ok(self
            .items
            .iter()
            .position(|item| item == &selected)
            .map_or(-1, |i| i as i32))
    }

    pub fn set_selected_index(&mut self, index: i32) -> Result<()> {
        if let Some(item) = usize::try_from(index).ok().and_then(|i| self.items.get(i)) {
            self.native.SetSelectedItem(item)?;
        }
        Ok(())
    }

    pub fn selected_id(&self) -> Result<Option<&str>> {
        Ok(usize::try_from(self.selected_index()?)
            .ok()
            .and_then(|index| self.descriptors.get(index))
            .map(|item| item.id.as_str()))
    }
}

impl Visible for SelectorBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, v: bool) -> Result<()> {
        self.element.set_visible(v)
    }
}
impl Enable for SelectorBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, v: bool) -> Result<()> {
        self.element.set_enabled(v)
    }
}
impl Layoutable for SelectorBar {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }
    fn set_loc(&mut self, v: Point) -> Result<()> {
        self.element.set_loc(v)
    }
    fn size(&self) -> Result<Size> {
        self.element.size()
    }
    fn set_size(&mut self, v: Size) -> Result<()> {
        self.element.set_size(v)
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
pub enum SelectorBarEvent {
    SelectionChanged {
        selected_index: i32,
        selected_id: Option<String>,
    },
}
#[derive(Debug)]
#[non_exhaustive]
pub enum SelectorBarMessage {}

impl Component for SelectorBar {
    type Error = Error;
    type Event = SelectorBarEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = SelectorBarMessage;
    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeSelectorBar::new()?;
        AutomationProperties::SetAutomationId(&native, &HSTRING::from("selector-bar-control"))?;
        let event_native = native.clone();
        let event_sender = sender.clone();
        let token = native.SelectionChanged(&TypedEventHandler::<
            NativeSelectorBar,
            SelectorBarSelectionChangedEventArgs,
        >::new(move |_, _| {
            let selected = event_native.SelectedItem().ok();
            let index = selected
                .as_ref()
                .and_then(|s| {
                    let items = event_native.Items().ok()?;
                    (0..items.Size().ok()?).find(|i| items.GetAt(*i).ok().as_ref() == Some(s))
                })
                .and_then(|i| i32::try_from(i).ok())
                .unwrap_or(-1);
            let id = selected
                .and_then(|s| {
                    s.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()
                        .ok()
                })
                .and_then(|e| e.Tag().ok())
                .and_then(|v| v.cast::<IPropertyValue>().ok())
                .and_then(|v| v.GetString().ok())
                .map(|v| v.to_string_lossy());
            event_sender.output(SelectorBarEvent::SelectionChanged {
                selected_index: index,
                selected_id: id,
            });
            Ok(())
        }))?;
        let fe = native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, fe)?;
        Ok(Self {
            native,
            element,
            descriptors: Vec::new(),
            items: Vec::new(),
            selection_changed_token: token,
        })
    }
}
impl AsWidget for SelectorBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
impl Drop for SelectorBar {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectionChanged(self.selection_changed_token);
    }
}
