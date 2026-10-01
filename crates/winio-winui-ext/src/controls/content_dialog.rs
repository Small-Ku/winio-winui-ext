use windows::Foundation::TypedEventHandler;
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::BorrowedContainer;
use winio::primitive::Failable;
use winio::{Error, Result};
use winio_winui3::Microsoft::UI::Xaml::Controls as MUXC;
use winio_winui3::Microsoft::UI::Xaml::Controls::{
    ContentDialog as NativeContentDialog, ContentDialogButton as NativeContentDialogButton,
    ContentDialogButtonClickEventArgs, ContentDialogClosingEventArgs, ContentDialogOpenedEventArgs,
    ContentDialogResult as NativeContentDialogResult, TextBlock,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContentDialogDefaultButton {
    #[default]
    None,
    Primary,
    Secondary,
    Close,
}

impl ContentDialogDefaultButton {
    fn native(self) -> NativeContentDialogButton {
        match self {
            Self::None => NativeContentDialogButton::None,
            Self::Primary => NativeContentDialogButton::Primary,
            Self::Secondary => NativeContentDialogButton::Secondary,
            Self::Close => NativeContentDialogButton::Close,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContentDialogCloseResult {
    #[default]
    None,
    Primary,
    Secondary,
    Unknown(i32),
}

impl From<NativeContentDialogResult> for ContentDialogCloseResult {
    fn from(value: NativeContentDialogResult) -> Self {
        if value == NativeContentDialogResult::None {
            Self::None
        } else if value == NativeContentDialogResult::Primary {
            Self::Primary
        } else if value == NativeContentDialogResult::Secondary {
            Self::Secondary
        } else {
            Self::Unknown(value.0)
        }
    }
}

/// A native WinUI ContentDialog that owns a Winio-compatible content Canvas.
///
/// ContentDialog is popup-backed rather than a child in the routed page visual
/// tree. The parent is used only to inherit the page's XamlRoot, while Winio
/// children created through content_container live directly inside the
/// dialog's native content surface.
#[derive(Debug)]
pub struct ContentDialog {
    native: NativeContentDialog,
    title: TextBlock,
    content: MUXC::Canvas,
    _show_operation: Option<IInspectable>,
    opened_token: i64,
    closing_token: i64,
    primary_token: i64,
    secondary_token: i64,
    close_token: i64,
}

impl Failable for ContentDialog {
    type Error = Error;
}

impl ContentDialog {
    pub fn title(&self) -> Result<String> {
        Ok(self.title.Text()?.to_string_lossy())
    }

    pub fn set_title(&mut self, title: impl AsRef<str>) -> Result<()> {
        self.title.SetText(&HSTRING::from(title.as_ref()))?;
        Ok(())
    }

    pub fn primary_button_text(&self) -> Result<String> {
        Ok(self.native.PrimaryButtonText()?.to_string_lossy())
    }

    pub fn set_primary_button_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.native
            .SetPrimaryButtonText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }

    pub fn secondary_button_text(&self) -> Result<String> {
        Ok(self.native.SecondaryButtonText()?.to_string_lossy())
    }

    pub fn set_secondary_button_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.native
            .SetSecondaryButtonText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }

    pub fn close_button_text(&self) -> Result<String> {
        Ok(self.native.CloseButtonText()?.to_string_lossy())
    }

    pub fn set_close_button_text(&mut self, text: impl AsRef<str>) -> Result<()> {
        self.native
            .SetCloseButtonText(&HSTRING::from(text.as_ref()))?;
        Ok(())
    }

    pub fn set_default_button(&mut self, button: ContentDialogDefaultButton) -> Result<()> {
        self.native.SetDefaultButton(button.native())?;
        Ok(())
    }

    pub fn content_container(&self) -> BorrowedContainer<'_> {
        BorrowedContainer::winui(&self.content)
    }

    pub fn set_content_size(&mut self, size: winio::primitive::Size) -> Result<()> {
        self.content.SetWidth(size.width)?;
        self.content.SetHeight(size.height)?;
        Ok(())
    }

    pub fn show(&mut self) -> Result<()> {
        let operation = self.native.ShowAsync()?;
        self._show_operation = Some(operation.cast::<IInspectable>()?);
        Ok(())
    }

    pub fn hide(&mut self) -> Result<()> {
        self.native.Hide()?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ContentDialogEvent {
    Opened,
    Closing { result: ContentDialogCloseResult },
    PrimaryButtonClick,
    SecondaryButtonClick,
    CloseButtonClick,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ContentDialogMessage {}

impl Component for ContentDialog {
    type Error = Error;
    type Event = ContentDialogEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = ContentDialogMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeContentDialog::new()?;
        let title = TextBlock::new()?;
        let content = MUXC::Canvas::new()?;
        content.SetWidth(360.0)?;
        content.SetHeight(96.0)?;

        native.SetTitle(&title)?;
        native.SetContent(&content)?;

        let xaml_root = parent.as_winui().XamlRoot()?;
        native.SetXamlRoot(&xaml_root)?;

        let opened_sender = sender.clone();
        let opened_token = native.Opened(&TypedEventHandler::<
            NativeContentDialog,
            ContentDialogOpenedEventArgs,
        >::new(move |_, _| {
            opened_sender.output(ContentDialogEvent::Opened);
            Ok(())
        }))?;

        let closing_sender = sender.clone();
        let closing_token = native.Closing(&TypedEventHandler::<
            NativeContentDialog,
            ContentDialogClosingEventArgs,
        >::new(move |_, args| {
            if let Some(args) = args.as_ref()
                && let Ok(result) = args.Result()
            {
                closing_sender.output(ContentDialogEvent::Closing {
                    result: result.into(),
                });
            }
            Ok(())
        }))?;

        let primary_sender = sender.clone();
        let primary_token = native.PrimaryButtonClick(&TypedEventHandler::<
            NativeContentDialog,
            ContentDialogButtonClickEventArgs,
        >::new(move |_, _| {
            primary_sender.output(ContentDialogEvent::PrimaryButtonClick);
            Ok(())
        }))?;

        let secondary_sender = sender.clone();
        let secondary_token = native.SecondaryButtonClick(&TypedEventHandler::<
            NativeContentDialog,
            ContentDialogButtonClickEventArgs,
        >::new(move |_, _| {
            secondary_sender.output(ContentDialogEvent::SecondaryButtonClick);
            Ok(())
        }))?;

        let close_sender = sender.clone();
        let close_token = native.CloseButtonClick(&TypedEventHandler::<
            NativeContentDialog,
            ContentDialogButtonClickEventArgs,
        >::new(move |_, _| {
            close_sender.output(ContentDialogEvent::CloseButtonClick);
            Ok(())
        }))?;

        Ok(Self {
            native,
            title,
            content,
            _show_operation: None,
            opened_token,
            closing_token,
            primary_token,
            secondary_token,
            close_token,
        })
    }
}

impl Drop for ContentDialog {
    fn drop(&mut self) {
        let _ = self.native.RemoveOpened(self.opened_token);
        let _ = self.native.RemoveClosing(self.closing_token);
        let _ = self.native.RemovePrimaryButtonClick(self.primary_token);
        let _ = self.native.RemoveSecondaryButtonClick(self.secondary_token);
        let _ = self.native.RemoveCloseButtonClick(self.close_token);
    }
}
