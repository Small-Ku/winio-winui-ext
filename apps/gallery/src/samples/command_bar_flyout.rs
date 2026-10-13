use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    CommandBarAction, CommandBarFlyout, CommandBarFlyoutEvent, ToggleButton, ToggleButtonEvent,
};

pub struct CommandBarFlyoutSample {
    page: GalleryPage,
    card: GalleryExample,
    target: Child<ToggleButton>,
    flyout: Child<CommandBarFlyout>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for CommandBarFlyoutSample {
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
            "CommandBarFlyout",
            "A native command flyout with stable primary and secondary actions.",
            "apps/gallery/src/samples/command_bar_flyout.rs · winio_winui_ext::CommandBarFlyout",
            include_str!("command_bar_flyout.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("closed")?;

        init! {
            target: ToggleButton = (&card) => { text: "Open CommandBarFlyout" },
        }

        let mut flyout = Child::<CommandBarFlyout>::init(card.as_container()).await?;
        flyout.set_primary_actions(&[
            CommandBarAction::new("copy", "Copy"),
            CommandBarAction::new("share", "Share"),
        ])?;
        flyout.set_secondary_actions(&[CommandBarAction::new("delete", "Delete")])?;

        Ok(Self {
            page,
            card,
            target,
            flyout,
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
                ToggleButtonEvent::StateChanged(_) =>
                    GallerySampleMessage::OpenCommandBarFlyout,
            },
            self.flyout => {
                CommandBarFlyoutEvent::Opened =>
                    GallerySampleMessage::Emit("opened".to_owned()),
                CommandBarFlyoutEvent::Closed => GallerySampleMessage::Emit(String::new()),
                CommandBarFlyoutEvent::ActionInvoked { id } =>
                    GallerySampleMessage::Emit(format!("action:{id}")),
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
            GallerySampleMessage::OpenCommandBarFlyout => {
                self.flyout.show_at(self.target.as_widget())?;
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "command-bar-flyout",
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
        update_children!(self.target, self.flyout)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.target.render()?;
        self.flyout.render()
    }
}

impl CommandBarFlyoutSample {
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
            (layout.primary.size.width - 44.0).clamp(0.0, 320.0),
            44.0,
        ))?;
        Ok(())
    }
}
