use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{FlipView, FlipViewEvent, FlipViewPage, ToggleButtonEvent};

const PAGE_IDS: [&str; 3] = ["first", "second", "third"];

pub struct FlipViewSample {
    page: GalleryPage,
    card: GalleryExample,
    flip_view: Child<FlipView>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_selection: bool,
    generation: u64,
}

impl Component for FlipViewSample {
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
            "FlipView",
            "A native paged collection with descriptor-backed content and typed selection changes.",
            "apps/gallery/src/samples/flip_view.rs · winio_winui_ext::FlipView",
            include_str!("flip_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("page:first")?;

        let mut flip_view = Child::<FlipView>::init(card.as_container()).await?;
        flip_view.set_pages(&[
            FlipViewPage::new("first", "First page"),
            FlipViewPage::new("second", "Second page"),
            FlipViewPage::new("third", "Third page"),
        ])?;
        flip_view.set_selected_index(0)?;
        flip_view.set_use_touch_animations_for_all_navigation(true)?;

        Ok(Self {
            page,
            card,
            flip_view,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_selection: true,
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
            self.flip_view => {
                FlipViewEvent::SelectionChanged { selected_index } =>
                    GallerySampleMessage::Emit(page_output(selected_index)),
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
                let initial = self.skip_initial_selection && value == "page:first";
                self.skip_initial_selection = false;
                if !initial {
                    self.card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "flip-view",
                        value,
                    });
                }
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.flip_view.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.flip_view.render()
    }
}

impl FlipViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.flip_view.set_loc(Point::new(
            layout.primary.origin.x + 16.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.flip_view.set_size(Size::new(
            (layout.primary.size.width - 32.0).max(0.0),
            (layout.primary.size.height - 24.0).max(100.0),
        ))?;
        Ok(())
    }
}

fn page_output(selected_index: i32) -> String {
    usize::try_from(selected_index)
        .ok()
        .and_then(|index| PAGE_IDS.get(index))
        .map_or_else(
            || format!("page:index-{selected_index}"),
            |id| format!("page:{id}"),
        )
}
