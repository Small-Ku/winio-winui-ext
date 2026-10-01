use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{TabView, TabViewEvent, TabViewTabDescriptor, ToggleButtonEvent};

pub struct TabViewSample {
    page: GalleryPage,
    card: GalleryExample,
    tabs: Child<TabView>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for TabViewSample {
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
            "TabView",
            "A native TabView with stable tab IDs independent of visible titles.",
            "apps/gallery/src/samples/tab_view.rs · winio_winui_ext::TabView",
            include_str!("tab_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("selected:overview")?;
        let mut tabs = Child::<TabView>::init(card.as_container()).await?;
        tabs.set_tabs(&[
            TabViewTabDescriptor::new("overview", "Overview", "Overview tab content"),
            TabViewTabDescriptor::new("details", "Details", "Details tab content"),
            TabViewTabDescriptor::new("activity", "Activity", "Recent activity tab content"),
        ])?;
        Ok(Self {
            page,
            card,
            tabs,
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
            self.tabs => {
                TabViewEvent::SelectionChanged { selected_id: Some(id), .. } =>
                    GallerySampleMessage::Emit(format!("selected:{id}")),
                TabViewEvent::SelectionChanged { selected_id: None, .. } =>
                    GallerySampleMessage::Emit("selected:none".to_owned()),
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
            GallerySampleMessage::Emit(value) if value.starts_with("selected:") => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "tab-view",
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
        update_children!(self.tabs)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.tabs.render()
    }
}

impl TabViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.tabs.set_loc(Point::new(
            layout.primary.origin.x + 12.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.tabs.set_size(Size::new(
            (layout.primary.size.width - 24.0).max(0.0),
            220.0,
        ))?;
        Ok(())
    }
}
