use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::RichEditBox as NativeRichEditBox;
use winio_winui3::Microsoft::UI::Text::{RichEditTextDocument, TextGetOptions, TextSetOptions};
use winio_winui3::Microsoft::UI::Xaml::{RoutedEventHandler, TextWrapping};

use crate::native::NativeElement;

/// A native WinUI RichEditBox with a document escape hatch.
///
/// text() and set_text() are plain-text conveniences. Call document() when the
/// caller needs ranges, selections, formatting, undo/redo, RTF, or stream APIs.
#[derive(Debug)]
pub struct RichEditBox {
    native: NativeRichEditBox,
    element: NativeElement,
    text_changed_token: i64,
}

impl Failable for RichEditBox {
    type Error = Error;
}

impl RichEditBox {
    pub fn document(&self) -> Result<RichEditTextDocument> {
        self.native.Document()
    }

    pub fn text(&self) -> Result<String> {
        plain_text(&self.native)
    }

    pub fn set_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .Document()?
            .SetText(TextSetOptions::None, &HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn rtf(&self) -> Result<String> {
        let document = self.native.Document()?;
        let mut value = HSTRING::new();
        document.GetText(TextGetOptions::FormatRtf, &mut value)?;
        Ok(value.to_string_lossy())
    }

    pub fn set_rtf(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native
            .Document()?
            .SetText(TextSetOptions::FormatRtf, &HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn is_read_only(&self) -> Result<bool> {
        self.native.IsReadOnly()
    }

    pub fn set_is_read_only(&mut self, value: bool) -> Result<()> {
        self.native.SetIsReadOnly(value)?;
        Ok(())
    }

    pub fn accepts_return(&self) -> Result<bool> {
        self.native.AcceptsReturn()
    }

    pub fn set_accepts_return(&mut self, value: bool) -> Result<()> {
        self.native.SetAcceptsReturn(value)?;
        Ok(())
    }

    pub fn text_wrapping(&self) -> Result<TextWrapping> {
        self.native.TextWrapping()
    }

    pub fn set_text_wrapping(&mut self, value: TextWrapping) -> Result<()> {
        self.native.SetTextWrapping(value)?;
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

    pub fn max_length(&self) -> Result<i32> {
        self.native.MaxLength()
    }

    pub fn set_max_length(&mut self, value: i32) -> Result<()> {
        self.native.SetMaxLength(value)?;
        Ok(())
    }

    pub fn is_spell_check_enabled(&self) -> Result<bool> {
        self.native.IsSpellCheckEnabled()
    }

    pub fn set_is_spell_check_enabled(&mut self, value: bool) -> Result<()> {
        self.native.SetIsSpellCheckEnabled(value)?;
        Ok(())
    }

    pub fn is_text_prediction_enabled(&self) -> Result<bool> {
        self.native.IsTextPredictionEnabled()
    }

    pub fn set_is_text_prediction_enabled(&mut self, value: bool) -> Result<()> {
        self.native.SetIsTextPredictionEnabled(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for RichEditBox {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for RichEditBox {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for RichEditBox {
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
pub enum RichEditBoxEvent {
    TextChanged,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RichEditBoxMessage {}

impl Component for RichEditBox {
    type Error = Error;
    type Event = RichEditBoxEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = RichEditBoxMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeRichEditBox::new()?;

        let event_sender = sender.clone();
        let text_changed_token = native.TextChanged(&RoutedEventHandler::new(move |_, _| {
            event_sender.output(RichEditBoxEvent::TextChanged);
            Ok(())
        }))?;

        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            text_changed_token,
        })
    }
}

impl AsWidget for RichEditBox {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for RichEditBox {
    fn drop(&mut self) {
        let _ = self.native.RemoveTextChanged(self.text_changed_token);
    }
}

fn plain_text(native: &NativeRichEditBox) -> windows_core::Result<String> {
    let document = native.Document()?;
    let mut value = HSTRING::new();
    document.GetText(TextGetOptions::None, &mut value)?;
    Ok(strip_final_eop(value.to_string_lossy()))
}

fn strip_final_eop(mut value: String) -> String {
    if value.ends_with("\r\n") {
        value.truncate(value.len() - 2);
    } else if value.ends_with(['\r', '\n']) {
        value.pop();
    }
    value
}

#[cfg(test)]
mod tests {
    use super::strip_final_eop;

    #[test]
    fn removes_only_the_story_final_eop() {
        assert_eq!(strip_final_eop("Rich editing\r".to_owned()), "Rich editing");
        assert_eq!(
            strip_final_eop("Rich editing\r\n".to_owned()),
            "Rich editing"
        );
        assert_eq!(
            strip_final_eop("Rich editing\r\r".to_owned()),
            "Rich editing\r"
        );
        assert_eq!(strip_final_eop("Rich editing".to_owned()), "Rich editing");
    }
}
