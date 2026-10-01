use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{PersonPicture, ToggleButtonEvent};

pub struct PersonPictureSample {
    page: GalleryPage,
    card: GalleryExample,
    picture: Child<PersonPicture>,
    viewport: Rect,
    source_expanded: bool,
}

impl Component for PersonPictureSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit {
            parent,
            generation: _,
        } = init;
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "PersonPicture",
            "A native avatar surface using initials, display name, and badge state.",
            "apps/gallery/src/samples/person_picture.rs · winio_winui_ext::PersonPicture",
            include_str!("person_picture.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("Ada Lovelace · initials AL · badge 3")?;

        let mut picture = Child::<PersonPicture>::init(card.as_container()).await?;
        picture.set_display_name("Ada Lovelace")?;
        picture.set_initials("AL")?;
        picture.set_badge_number(3)?;
        picture.set_badge_text("3")?;
        picture.set_is_group(false)?;
        picture.set_prefer_small_image(false)?;

        card.set_output(format!(
            "{} · initials {} · badge {}",
            picture.display_name()?,
            picture.initials()?,
            picture.badge_number()?,
        ))?;

        Ok(Self {
            page,
            card,
            picture,
            viewport: Rect::zero(),
            source_expanded: false,
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
            self.picture => {},
        }
    }

    async fn update(&mut self, message: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
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
                self.card.set_output(value)?;
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.picture.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.picture.render()
    }
}

impl PersonPictureSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        let size = layout.primary.size.height.clamp(0.0, 128.0);
        self.picture.set_loc(Point::new(
            layout.primary.origin.x + 28.0,
            layout.primary.origin.y + 20.0,
        ))?;
        self.picture.set_size(Size::new(size, size))?;
        Ok(())
    }
}
