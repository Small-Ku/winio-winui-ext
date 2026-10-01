use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    PipsPager, PipsPagerButtonVisibility, PipsPagerEvent, PipsPagerWrapMode, ToggleButtonEvent,
};

pub struct PipsPagerSample {
    page: GalleryPage,
    card: GalleryExample,
    pager: Child<PipsPager>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_selection: bool,
    generation: u64,
}

impl Component for PipsPagerSample {
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
            "PipsPager",
            "A native page-position control with explicit previous/next buttons and typed selection changes.",
            "apps/gallery/src/samples/pips_pager.rs · winio_winui_ext::PipsPager",
            include_str!("pips_pager.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("page:1")?;

        let mut pager = Child::<PipsPager>::init(card.as_container()).await?;
        pager.set_number_of_pages(5)?;
        pager.set_max_visible_pips(5)?;
        pager.set_previous_button_visibility(PipsPagerButtonVisibility::Visible)?;
        pager.set_next_button_visibility(PipsPagerButtonVisibility::Visible)?;
        pager.set_wrap_mode(PipsPagerWrapMode::None)?;
        pager.set_selected_page_index(0)?;

        Ok(Self {
            page,
            card,
            pager,
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
            self.pager => {
                PipsPagerEvent::SelectedIndexChanged { selected_page_index } =>
                    GallerySampleMessage::Emit(format!("page:{}", selected_page_index + 1)),
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
                let initial = self.skip_initial_selection && value == "page:1";
                self.skip_initial_selection = false;
                if !initial {
                    self.card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "pips-pager",
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
        self.pager.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.pager.render()
    }
}

impl PipsPagerSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.pager.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 46.0,
        ))?;
        self.pager
            .set_size(Size::new((layout.primary.size.width - 40.0).max(0.0), 64.0))?;
        Ok(())
    }
}
