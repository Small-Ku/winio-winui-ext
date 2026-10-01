use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{CommandBar, CommandBarAction, CommandBarEvent, ToggleButtonEvent};

pub struct CommandBarSample {
    page: GalleryPage,
    card: GalleryExample,
    command_bar: Child<CommandBar>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for CommandBarSample {
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
            "CommandBar",
            "Primary actions stay visible while secondary actions live in the native overflow menu.",
            "apps/gallery/src/samples/command_bar.rs · winio_winui_ext::CommandBar",
            include_str!("command_bar.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("ready")?;

        let mut command_bar = Child::<CommandBar>::init(card.as_container()).await?;
        command_bar.set_primary_actions(&[
            CommandBarAction::new("save", "Save"),
            CommandBarAction::new("share", "Share"),
        ])?;
        command_bar.set_secondary_actions(&[CommandBarAction::new("settings", "Settings")])?;

        Ok(Self {
            page,
            card,
            command_bar,
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
            self.command_bar => {
                CommandBarEvent::ActionInvoked { id } =>
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
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "command-bar",
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
        self.command_bar.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.command_bar.render()
    }
}

impl CommandBarSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.command_bar.set_loc(Point::new(
            layout.primary.origin.x + 16.0,
            layout.primary.origin.y + 32.0,
        ))?;
        self.command_bar
            .set_size(Size::new((layout.primary.size.width - 32.0).max(0.0), 56.0))?;
        Ok(())
    }
}
