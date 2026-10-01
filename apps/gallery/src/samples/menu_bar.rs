use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{MenuBar, MenuBarAction, MenuBarEvent, MenuBarMenu, ToggleButtonEvent};

pub struct MenuBarSample {
    page: GalleryPage,
    card: GalleryExample,
    menu_bar: Child<MenuBar>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for MenuBarSample {
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
            "MenuBar",
            "Top-level native menus with stable descriptor-backed actions.",
            "apps/gallery/src/samples/menu_bar.rs · winio_winui_ext::MenuBar",
            include_str!("menu_bar.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("ready")?;

        let mut menu_bar = Child::<MenuBar>::init(card.as_container()).await?;
        menu_bar.set_menus(&[
            MenuBarMenu::new("File").with_actions([
                MenuBarAction::new("new", "New"),
                MenuBarAction::new("open", "Open"),
            ]),
            MenuBarMenu::new("Help").with_actions([MenuBarAction::new("about", "About")]),
        ])?;

        Ok(Self {
            page,
            card,
            menu_bar,
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
            self.menu_bar => {
                MenuBarEvent::ActionInvoked { id } =>
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
                    sample_id: "menu-bar",
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
        self.menu_bar.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.menu_bar.render()
    }
}

impl MenuBarSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.menu_bar.set_loc(Point::new(
            layout.primary.origin.x + 16.0,
            layout.primary.origin.y + 42.0,
        ))?;
        self.menu_bar
            .set_size(Size::new((layout.primary.size.width - 32.0).max(0.0), 44.0))?;
        Ok(())
    }
}
