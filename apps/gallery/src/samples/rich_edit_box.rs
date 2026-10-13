use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{RichEditBox, RichEditBoxEvent, ToggleButtonEvent};
use winio_winui3::Microsoft::UI::Xaml::TextWrapping;

pub struct RichEditBoxSample {
    page: GalleryPage,
    card: GalleryExample,
    editor: Child<RichEditBox>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for RichEditBoxSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "RichEditBox",
            "A native rich-text editor with plain-text convenience and direct document access for ranges and formatting.",
            "apps/gallery/src/samples/rich_edit_box.rs · winio_winui_ext::RichEditBox",
            include_str!("rich_edit_box.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("Rich editing")?;

        let mut editor = Child::<RichEditBox>::init(card.as_container()).await?;
        editor.set_placeholder_text("Edit rich text")?;
        editor.set_accepts_return(true)?;
        editor.set_text_wrapping(TextWrapping::Wrap)?;
        editor.set_is_spell_check_enabled(false)?;
        editor.set_is_text_prediction_enabled(false)?;
        editor.set_rtf(r"{\rtf1\ansi Rich \b editing\b0}")?;
        card.set_output(&editor.text()?)?;

        // Exercise the public rich-document escape hatch without hiding it behind
        // a second wrapper type. Callers can keep this object for ranges/formatting.
        let document = editor.document()?;
        let _selection = document.Selection()?;

        Ok(Self {
            page,
            card,
            editor,
            viewport: Rect::zero(),
            source_expanded: false,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => {
                ToggleButtonEvent::StateChanged(state) =>
                    GallerySampleMessage::SourceExpanded {
                        card: card_id,
                        expanded: state == Some(true),
                    },
            },
            self.editor => {
                RichEditBoxEvent::TextChanged => GallerySampleMessage::RichEditChanged,
            },
        }
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            GallerySampleMessage::SetBounds(viewport) => {
                self.viewport = viewport;
                self.reflow()?;
            }
            GallerySampleMessage::SourceExpanded {
                card: CardId::MAIN,
                expanded,
            } => {
                self.source_expanded = expanded;
                self.reflow()?;
            }
            GallerySampleMessage::RichEditChanged => {
                let value = "changed".to_owned();
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "rich-edit-box",
                    value,
                });
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "rich-edit-box",
                    value,
                });
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.editor.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.editor.render()
    }
}

impl RichEditBoxSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.editor.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 20.0,
        ))?;
        self.editor.set_size(Size::new(
            (layout.primary.size.width - 40.0).max(0.0),
            (layout.primary.size.height - 40.0).max(0.0),
        ))?;
        Ok(())
    }
}
