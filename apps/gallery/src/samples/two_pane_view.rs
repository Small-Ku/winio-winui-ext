use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    ProgressBar, ToggleButton, ToggleButtonEvent, TwoPaneView, TwoPaneViewWideModeConfiguration,
};

pub struct TwoPaneViewSample {
    page: GalleryPage,
    card: GalleryExample,
    view: Child<TwoPaneView>,
    progress: Child<ProgressBar>,
    pause: Child<ToggleButton>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}

impl Component for TwoPaneViewSample {
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
            "TwoPaneView",
            "A native adaptive two-pane host with wrapper-owned Winio Canvas containers.",
            "apps/gallery/src/samples/two_pane_view.rs · winio_winui_ext::TwoPaneView",
            include_str!("two_pane_view.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("wide · unpaused")?;

        let mut view = Child::<TwoPaneView>::init(card.as_container()).await?;
        view.set_wide_mode_configuration(TwoPaneViewWideModeConfiguration::LeftRight)?;
        view.set_min_wide_mode_width(560.0)?;
        let mut progress = Child::<ProgressBar>::init(view.pane1_container()).await?;
        progress.set_minimum(0.0)?;
        progress.set_maximum(100.0)?;
        progress.set_value(65.0)?;
        let mut pause = Child::<ToggleButton>::init(view.pane2_container()).await?;
        pause.set_text("Pause progress")?;
        pause.set_three_state(false)?;

        Ok(Self {
            page,
            card,
            view,
            progress,
            pause,
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
            self.pause => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::Emit(
                    if state == Some(true) { "pause".to_owned() } else { "resume".to_owned() }
                ),
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
            GallerySampleMessage::Emit(value) if value == "pause" || value == "resume" => {
                let paused = value == "pause";
                self.progress.set_show_paused(paused)?;
                self.card.set_output(if paused {
                    "wide · paused"
                } else {
                    "wide · unpaused"
                })?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "two-pane-view",
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
        update_children!(self.view, self.progress, self.pause)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.view.render()?;
        self.progress.render()?;
        self.pause.render()
    }
}

impl TwoPaneViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.view.set_loc(Point::new(
            layout.primary.origin.x + 12.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.view
            .set_size(Size::new(layout.primary.size.width - 24.0, 216.0))?;
        self.progress.set_loc(Point::new(24.0, 70.0))?;
        self.progress.set_size(Size::new(260.0, 32.0))?;
        self.pause.set_loc(Point::new(24.0, 70.0))?;
        self.pause.set_size(Size::new(220.0, 36.0))?;
        Ok(())
    }
}
