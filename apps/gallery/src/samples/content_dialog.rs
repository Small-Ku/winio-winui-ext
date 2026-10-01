use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    ContentDialog, ContentDialogDefaultButton, ContentDialogEvent, ToggleButton, ToggleButtonEvent,
};

pub struct ContentDialogSample {
    page: GalleryPage,
    card: GalleryExample,
    target: Child<ToggleButton>,
    dialog: Child<ContentDialog>,
    dialog_label: Child<Label>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for ContentDialogSample {
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
            "ContentDialog",
            "A native modal dialog bound to the routed page XamlRoot.",
            "apps/gallery/src/samples/content_dialog.rs · winio_winui_ext::ContentDialog",
            include_str!("content_dialog.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("closed")?;

        init! {
            target: ToggleButton = (&card) => { text: "Open ContentDialog" },
        }

        let mut dialog = Child::<ContentDialog>::init(card.as_container()).await?;
        dialog.set_title("Save changes?")?;
        dialog.set_primary_button_text("Save")?;
        dialog.set_secondary_button_text("Don't save")?;
        dialog.set_close_button_text("Cancel")?;
        dialog.set_default_button(ContentDialogDefaultButton::Primary)?;
        dialog.set_content_size(Size::new(360.0, 88.0))?;

        let mut dialog_label = Child::<Label>::init(dialog.content_container()).await?;
        dialog_label.set_text("This content is owned by Winio inside the native ContentDialog.")?;
        dialog_label.set_loc(Point::new(12.0, 12.0))?;
        dialog_label.set_size(Size::new(336.0, 64.0))?;

        Ok(Self {
            page,
            card,
            target,
            dialog,
            dialog_label,
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
            self.target => {
                ToggleButtonEvent::StateChanged(_) => GallerySampleMessage::OpenContentDialog,
            },
            self.dialog => {
                ContentDialogEvent::Opened =>
                    GallerySampleMessage::Emit("opened".to_owned()),
                ContentDialogEvent::PrimaryButtonClick =>
                    GallerySampleMessage::Emit("primary click".to_owned()),
                ContentDialogEvent::SecondaryButtonClick =>
                    GallerySampleMessage::Emit("secondary click".to_owned()),
                ContentDialogEvent::CloseButtonClick =>
                    GallerySampleMessage::Emit("close click".to_owned()),
                ContentDialogEvent::Closing { result } =>
                    GallerySampleMessage::Emit(format!("closing:{result:?}")),
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
            GallerySampleMessage::OpenContentDialog => {
                self.dialog.show()?;
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "content-dialog",
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
        update_children!(self.target, self.dialog, self.dialog_label)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.target.render()?;
        self.dialog.render()?;
        self.dialog_label.render()
    }
}

impl ContentDialogSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.target.set_loc(Point::new(
            layout.primary.origin.x + 22.0,
            layout.primary.origin.y + 44.0,
        ))?;
        self.target.set_size(Size::new(
            (layout.primary.size.width - 44.0).clamp(0.0, 300.0),
            44.0,
        ))?;
        Ok(())
    }
}
