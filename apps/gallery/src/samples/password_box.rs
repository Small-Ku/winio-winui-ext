use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{PasswordBox, PasswordBoxEvent, ToggleButtonEvent};

pub struct PasswordBoxSample {
    page: GalleryPage,
    card: GalleryExample,
    password: Child<PasswordBox>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for PasswordBoxSample {
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
            "PasswordBox",
            "A native password field that reports changes without exposing the password in sample output.",
            "apps/gallery/src/samples/password_box.rs · winio_winui_ext::PasswordBox",
            include_str!("password_box.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("password length:0")?;

        let mut password = Child::<PasswordBox>::init(card.as_container()).await?;
        password.set_header("Password")?;
        password.set_placeholder_text("Enter password")?;
        password.set_max_length(16)?;
        password.set_is_password_reveal_button_enabled(true)?;

        Ok(Self {
            page,
            card,
            password,
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
            self.password => {
                PasswordBoxEvent::PasswordChanged =>
                    GallerySampleMessage::Emit("password-changed".to_owned()),
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
            GallerySampleMessage::Emit(value) if value == "password-changed" => {
                let value = format!(
                    "password length:{}",
                    self.password.password()?.chars().count()
                );
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "password-box",
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
        self.password.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.password.render()
    }
}

impl PasswordBoxSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.password.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 28.0,
        ))?;
        self.password.set_size(Size::new(
            (layout.primary.size.width - 40.0).clamp(0.0, 380.0),
            76.0,
        ))?;
        Ok(())
    }
}
