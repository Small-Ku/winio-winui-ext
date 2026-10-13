use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{Flyout, FlyoutEvent, ToggleButton, ToggleButtonEvent};

pub struct FlyoutSample {
    page: GalleryPage,
    card: GalleryExample,
    target: Child<ToggleButton>,
    flyout: Child<Flyout>,
    flyout_label: Child<Label>,
    flyout_action: Child<ToggleButton>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for FlyoutSample {
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
            "Flyout",
            "A native popup with Winio-owned interactive content.",
            "apps/gallery/src/samples/flyout.rs · winio_winui_ext::Flyout",
            include_str!("flyout.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("closed")?;

        init! {
            target: ToggleButton = (&card) => { text: "Open Flyout" },
        }

        let mut flyout = Child::<Flyout>::init(card.as_container()).await?;
        flyout.set_content_size(Size::new(280.0, 112.0))?;

        let mut flyout_label = Child::<Label>::init(flyout.content_container()).await?;
        flyout_label.set_text("Winio-owned flyout content")?;
        flyout_label.set_loc(Point::new(16.0, 12.0))?;
        flyout_label.set_size(Size::new(248.0, 28.0))?;

        let mut flyout_action = Child::<ToggleButton>::init(flyout.content_container()).await?;
        flyout_action.set_text("Acknowledge")?;
        flyout_action.set_loc(Point::new(16.0, 52.0))?;
        flyout_action.set_size(Size::new(160.0, 44.0))?;

        Ok(Self {
            page,
            card,
            target,
            flyout,
            flyout_label,
            flyout_action,
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
                ToggleButtonEvent::StateChanged(_) => GallerySampleMessage::OpenFlyout,
            },
            self.flyout_action => {
                ToggleButtonEvent::StateChanged(_) => GallerySampleMessage::ActivateFlyoutContent,
            },
            self.flyout => {
                FlyoutEvent::Opened => GallerySampleMessage::Emit("opened".to_owned()),
                FlyoutEvent::Closed => GallerySampleMessage::Emit("closed".to_owned()),
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
            GallerySampleMessage::OpenFlyout => {
                self.flyout.show_at(self.target.as_widget())?;
            }
            GallerySampleMessage::ActivateFlyoutContent => {
                let value = "content action".to_owned();
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "flyout",
                    value,
                });
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "flyout",
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
        update_children!(
            self.target,
            self.flyout,
            self.flyout_label,
            self.flyout_action
        )
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.target.render()?;
        self.flyout.render()?;
        self.flyout_label.render()?;
        self.flyout_action.render()
    }
}

impl FlyoutSample {
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
