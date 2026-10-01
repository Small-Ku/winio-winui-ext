use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ToggleButtonEvent, ToggleSplitButton, ToggleSplitButtonEvent};

pub struct ToggleSplitButtonSample {
    page: GalleryPage,
    card: GalleryExample,
    control: Child<ToggleSplitButton>,
    flyout_label: Child<Label>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for ToggleSplitButtonSample {
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
            "ToggleSplitButton",
            "The primary button toggles checked state; the split arrow opens a separately owned flyout.",
            "apps/gallery/src/samples/toggle_split_button.rs · winio_winui_ext::ToggleSplitButton",
            include_str!("toggle_split_button.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("unchecked")?;
        let mut control = Child::<ToggleSplitButton>::init(card.as_container()).await?;
        control.set_text("Format")?;
        let mut flyout_label = Child::<Label>::init(control.flyout_container()).await?;
        flyout_label.set_text("Choose formatting options")?;

        Ok(Self {
            page,
            card,
            control,
            flyout_label,
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
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded {
                    card: card_id,
                    expanded: state == Some(true),
                },
            },
            self.control => {
                ToggleSplitButtonEvent::Click => GallerySampleMessage::Emit("primary click".to_owned()),
                ToggleSplitButtonEvent::CheckedChanged(value) => GallerySampleMessage::Emit(
                    if value { "checked".to_owned() } else { "unchecked".to_owned() }
                ),
                ToggleSplitButtonEvent::FlyoutOpened => GallerySampleMessage::Emit("flyout opened".to_owned()),
                ToggleSplitButtonEvent::FlyoutClosed => GallerySampleMessage::Emit("flyout closed".to_owned()),
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
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "toggle-split-button",
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
        update_children!(self.control, self.flyout_label)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.control.render()?;
        self.flyout_label.render()
    }
}

impl ToggleSplitButtonSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.control.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 42.0,
        ))?;
        self.control.set_size(Size::new(
            (layout.primary.size.width - 40.0).clamp(0.0, 280.0),
            44.0,
        ))?;
        self.flyout_label.set_loc(Point::new(16.0, 16.0))?;
        self.flyout_label.set_size(Size::new(228.0, 32.0))?;
        Ok(())
    }
}
