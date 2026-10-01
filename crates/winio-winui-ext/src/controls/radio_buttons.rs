use windows::Foundation::PropertyValue;
use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Automation::AutomationProperties;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::RadioButtons as NativeRadioButtons;
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioButtonOption {
    pub id: String,
    pub text: String,
}
impl RadioButtonOption {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug)]
pub struct RadioButtons {
    native: NativeRadioButtons,
    element: NativeElement,
    options: Vec<RadioButtonOption>,
    buttons: Vec<MUXC::RadioButton>,
    selection_changed_token: i64,
}
impl Failable for RadioButtons {
    type Error = Error;
}
impl RadioButtons {
    pub fn set_options(&mut self, options: &[RadioButtonOption]) -> Result<()> {
        let items = self.native.Items()?;
        items.Clear()?;
        let mut buttons = Vec::with_capacity(options.len());
        for option in options {
            let button = MUXC::RadioButton::new()?;
            let text = MUXC::TextBlock::new()?;
            text.SetText(&HSTRING::from(option.text.as_str()))?;
            button.cast::<MUXC::ContentControl>()?.SetContent(&text)?;
            let id = PropertyValue::CreateString(&HSTRING::from(option.id.as_str()))?;
            button
                .cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?
                .SetTag(&id)?;
            AutomationProperties::SetAutomationId(
                &button,
                &HSTRING::from(format!("radio-{}", option.id)),
            )?;
            items.Append(&button)?;
            buttons.push(button);
        }
        self.options = options.to_vec();
        self.buttons = buttons;
        if !self.options.is_empty() && self.native.SelectedIndex()? < 0 {
            self.native.SetSelectedIndex(0)?;
        }
        Ok(())
    }
    pub fn selected_index(&self) -> Result<i32> {
        self.native.SelectedIndex()
    }
    pub fn set_selected_index(&mut self, index: i32) -> Result<()> {
        self.native.SetSelectedIndex(index)?;
        Ok(())
    }
    pub fn selected_id(&self) -> Result<Option<&str>> {
        Ok(usize::try_from(self.selected_index()?)
            .ok()
            .and_then(|i| self.options.get(i))
            .map(|v| v.id.as_str()))
    }
    pub fn max_columns(&self) -> Result<i32> {
        self.native.MaxColumns()
    }
    pub fn set_max_columns(&mut self, value: i32) -> Result<()> {
        self.native.SetMaxColumns(value)?;
        Ok(())
    }
}
impl Visible for RadioButtons {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, v: bool) -> Result<()> {
        self.element.set_visible(v)
    }
}
impl Enable for RadioButtons {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, v: bool) -> Result<()> {
        self.element.set_enabled(v)
    }
}
impl Layoutable for RadioButtons {
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
pub enum RadioButtonsEvent {
    SelectionChanged {
        selected_index: i32,
        selected_id: Option<String>,
    },
}
#[derive(Debug)]
#[non_exhaustive]
pub enum RadioButtonsMessage {}
impl Component for RadioButtons {
    type Error = Error;
    type Event = RadioButtonsEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = RadioButtonsMessage;
    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeRadioButtons::new()?;
        let event_native = native.clone();
        let event_sender = sender.clone();
        let token =
            native.SelectionChanged(&MUXC::SelectionChangedEventHandler::new(move |_, _| {
                let index = event_native.SelectedIndex().unwrap_or(-1);
                let id = usize::try_from(index)
                    .ok()
                    .and_then(|i| event_native.ContainerFromIndex(i as i32).ok())
                    .and_then(|e| {
                        e.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()
                            .ok()
                    })
                    .and_then(|e| e.Tag().ok())
                    .and_then(|v| v.cast::<windows::Foundation::IPropertyValue>().ok())
                    .and_then(|v| v.GetString().ok())
                    .map(|v| v.to_string_lossy());
                event_sender.output(RadioButtonsEvent::SelectionChanged {
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
            options: Vec::new(),
            buttons: Vec::new(),
            selection_changed_token: token,
        })
    }
}
impl AsWidget for RadioButtons {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
impl Drop for RadioButtons {
    fn drop(&mut self) {
        let _ = self
            .native
            .RemoveSelectionChanged(self.selection_changed_token);
    }
}
