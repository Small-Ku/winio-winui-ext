use windows::Foundation::{IPropertyValue, PropertyValue, TypedEventHandler};
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    AutoSuggestBox as NativeAutoSuggestBox, AutoSuggestBoxQuerySubmittedEventArgs,
    AutoSuggestBoxSuggestionChosenEventArgs, AutoSuggestBoxTextChangedEventArgs,
    AutoSuggestionBoxTextChangeReason as NativeTextChangeReason,
};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;

use crate::native::NativeElement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoSuggestBoxItemDescriptor {
    pub id: String,
    pub text: String,
}

impl AutoSuggestBoxItemDescriptor {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AutoSuggestBoxTextChangeReason {
    UserInput,
    ProgrammaticChange,
    SuggestionChosen,
    Unknown(i32),
}

fn map_text_change_reason(reason: NativeTextChangeReason) -> AutoSuggestBoxTextChangeReason {
    match reason.0 {
        0 => AutoSuggestBoxTextChangeReason::UserInput,
        1 => AutoSuggestBoxTextChangeReason::ProgrammaticChange,
        2 => AutoSuggestBoxTextChangeReason::SuggestionChosen,
        value => AutoSuggestBoxTextChangeReason::Unknown(value),
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum AutoSuggestBoxEvent {
    TextChanged {
        text: String,
        reason: AutoSuggestBoxTextChangeReason,
    },
    SuggestionChosen {
        id: Option<String>,
    },
    QuerySubmitted {
        query_text: String,
        chosen_id: Option<String>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum AutoSuggestBoxMessage {}

#[derive(Debug)]
pub struct AutoSuggestBox {
    native: NativeAutoSuggestBox,
    element: NativeElement,
    text_changed_token: i64,
    suggestion_chosen_token: i64,
    query_submitted_token: i64,
}

impl Failable for AutoSuggestBox {
    type Error = Error;
}

impl AutoSuggestBox {
    pub fn text(&self) -> Result<String> {
        Ok(self.native.Text()?.to_string_lossy())
    }
    pub fn set_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }
    pub fn placeholder_text(&self) -> Result<String> {
        Ok(self.native.PlaceholderText()?.to_string_lossy())
    }
    pub fn set_placeholder_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .SetPlaceholderText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }
    pub fn is_suggestion_list_open(&self) -> Result<bool> {
        self.native.IsSuggestionListOpen()
    }
    pub fn set_is_suggestion_list_open(&mut self, value: bool) -> Result<()> {
        self.native.SetIsSuggestionListOpen(value)?;
        Ok(())
    }

    /// Replaces suggestions with native upstream TextBlocks carrying stable IDs in `Tag`.
    pub fn set_suggestions(&mut self, suggestions: &[AutoSuggestBoxItemDescriptor]) -> Result<()> {
        let items = self.native.cast::<MUXC::ItemsControl>()?.Items()?;
        items.Clear()?;
        for descriptor in suggestions {
            let text = MUXC::TextBlock::new()?;
            text.SetText(&HSTRING::from(descriptor.text.as_str()))?;
            let id = PropertyValue::CreateString(&HSTRING::from(descriptor.id.as_str()))?;
            text.SetTag(&id)?;
            items.Append(&text)?;
        }
        Ok(())
    }
}

fn suggestion_id(item: &IInspectable) -> Option<String> {
    // AutoSuggestBox.SelectedItem is the object stored in Items. We store TextBlocks,
    // but also accept a FrameworkElement carrying Tag or a boxed string if WinUI
    // normalizes the runtime item representation.
    item.cast::<MUXC::TextBlock>()
        .ok()
        .and_then(|text| text.Tag().ok())
        .or_else(|| {
            item.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()
                .ok()
                .and_then(|element| element.Tag().ok())
        })
        .and_then(|tag| tag.cast::<IPropertyValue>().ok())
        .and_then(|tag| tag.GetString().ok())
        .map(|id| id.to_string_lossy())
        .or_else(|| {
            item.cast::<IPropertyValue>()
                .ok()
                .and_then(|value| value.GetString().ok())
                .map(|id| id.to_string_lossy())
        })
}

impl Visible for AutoSuggestBox {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }
    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}
impl Enable for AutoSuggestBox {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }
    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}
impl Layoutable for AutoSuggestBox {
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

impl Component for AutoSuggestBox {
    type Error = Error;
    type Event = AutoSuggestBoxEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = AutoSuggestBoxMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeAutoSuggestBox::new()?;
        let text_native = native.clone();
        let text_sender = sender.clone();
        let text_changed_token = native.TextChanged(&TypedEventHandler::<
            NativeAutoSuggestBox,
            AutoSuggestBoxTextChangedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args.as_ref() {
                let text = text_native.Text()?.to_string_lossy();
                let reason = map_text_change_reason(args.Reason()?);
                text_sender.output(AutoSuggestBoxEvent::TextChanged { text, reason });
            }
            Ok(())
        }))?;
        let chosen_sender = sender.clone();
        let suggestion_chosen_token =
            native.SuggestionChosen(&TypedEventHandler::<
                NativeAutoSuggestBox,
                AutoSuggestBoxSuggestionChosenEventArgs,
            >::new(move |_, args| {
                let id = args
                    .as_ref()
                    .and_then(|args| args.SelectedItem().ok())
                    .and_then(|item| suggestion_id(&item));
                chosen_sender.output(AutoSuggestBoxEvent::SuggestionChosen { id });
                Ok(())
            }))?;
        let submitted_sender = sender.clone();
        let query_submitted_token = native.QuerySubmitted(&TypedEventHandler::<
            NativeAutoSuggestBox,
            AutoSuggestBoxQuerySubmittedEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref() {
                let query_text = args.QueryText()?.to_string_lossy();
                let chosen_id = args
                    .ChosenSuggestion()
                    .ok()
                    .and_then(|item| suggestion_id(&item));
                submitted_sender.output(AutoSuggestBoxEvent::QuerySubmitted {
                    query_text,
                    chosen_id,
                });
            }
            Ok(())
        }))?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self {
            native,
            element,
            text_changed_token,
            suggestion_chosen_token,
            query_submitted_token,
        })
    }
}
impl AsWidget for AutoSuggestBox {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
impl Drop for AutoSuggestBox {
    fn drop(&mut self) {
        let _ = self.native.RemoveTextChanged(self.text_changed_token);
        let _ = self
            .native
            .RemoveSuggestionChosen(self.suggestion_chosen_token);
        let _ = self.native.RemoveQuerySubmitted(self.query_submitted_token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_native_text_change_reasons_including_unknown_values() {
        assert_eq!(
            map_text_change_reason(NativeTextChangeReason::UserInput),
            AutoSuggestBoxTextChangeReason::UserInput
        );
        assert_eq!(
            map_text_change_reason(NativeTextChangeReason::ProgrammaticChange),
            AutoSuggestBoxTextChangeReason::ProgrammaticChange
        );
        assert_eq!(
            map_text_change_reason(NativeTextChangeReason::SuggestionChosen),
            AutoSuggestBoxTextChangeReason::SuggestionChosen
        );
        assert_eq!(
            map_text_change_reason(NativeTextChangeReason(77)),
            AutoSuggestBoxTextChangeReason::Unknown(77)
        );
    }
}
