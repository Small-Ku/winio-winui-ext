use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ToggleButtonEvent, ToggleSwitch, ToggleSwitchEvent};

pub struct ToggleSwitchSample {
    page: GalleryPage,
    card: GalleryExample,
    toggle: Child<ToggleSwitch>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for ToggleSwitchSample {
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
            "ToggleSwitch",
            "A native WinUI switch with a typed on/off event.",
            "apps/gallery/src/samples/toggle_switch.rs · winio_winui_ext::ToggleSwitch",
            include_str!("toggle_switch.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("off")?;

        init! {
            toggle: ToggleSwitch = (&card) => { text: "Wi-Fi" },
        }
        toggle.set_is_on(false)?;

        Ok(Self {
            page,
            card,
            toggle,
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
            self.toggle => {
                ToggleSwitchEvent::Toggled(is_on) =>
                    GallerySampleMessage::Emit(if is_on { "on" } else { "off" }.to_owned()),
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
                    sample_id: "toggle-switch",
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
        self.toggle.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.toggle.render()
    }
}

impl ToggleSwitchSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.toggle.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 34.0,
        ))?;
        let preferred = self.toggle.preferred_size()?;
        self.toggle.set_size(Size::new(
            (layout.primary.size.width - 40.0)
                .min(preferred.width.max(220.0))
                .max(0.0),
            preferred.height.max(56.0),
        ))?;
        Ok(())
    }
}
