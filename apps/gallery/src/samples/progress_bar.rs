use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ProgressBar, ToggleButton, ToggleButtonEvent};

pub struct ProgressBarSample {
    page: GalleryPage,
    card: GalleryExample,
    bar: Child<ProgressBar>,
    indeterminate: Child<ToggleButton>,
    viewport: Rect,
    source_expanded: bool,
}

impl Component for ProgressBarSample {
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
            "ProgressBar",
            "A native determinate ProgressBar with a live indeterminate option.",
            "apps/gallery/src/samples/progress_bar.rs · winio_winui_ext::ProgressBar",
            include_str!("progress_bar.rs"),
            true,
            true,
        )
        .await?;
        card.set_output("value:65 · determinate")?;

        let mut bar = Child::<ProgressBar>::init(card.as_container()).await?;
        bar.set_minimum(0.0)?;
        bar.set_maximum(100.0)?;
        bar.set_value(65.0)?;
        bar.set_is_indeterminate(false)?;

        let mut indeterminate = Child::<ToggleButton>::init(card.as_container()).await?;
        indeterminate.set_text("Indeterminate")?;
        indeterminate.set_three_state(false)?;

        Ok(Self {
            page,
            card,
            bar,
            indeterminate,
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
            self.indeterminate => {
                ToggleButtonEvent::StateChanged(state) =>
                    GallerySampleMessage::Emit(if state == Some(true) {
                        "progress-bar:indeterminate".to_owned()
                    } else {
                        "progress-bar:determinate".to_owned()
                    }),
            },
            self.bar => {},
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
            GallerySampleMessage::Emit(value) if value == "progress-bar:indeterminate" => {
                self.bar.set_is_indeterminate(true)?;
                self.card.set_output("indeterminate")?;
            }
            GallerySampleMessage::Emit(value) if value == "progress-bar:determinate" => {
                self.bar.set_is_indeterminate(false)?;
                self.card.set_output("value:65 · determinate")?;
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.bar, self.indeterminate)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.bar.render()?;
        self.indeterminate.render()
    }
}

impl ProgressBarSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.bar.set_loc(Point::new(
            layout.primary.origin.x + 56.0,
            layout.primary.origin.y + 36.0,
        ))?;
        self.bar.set_size(Size::new(240.0, 36.0))?;

        if let Some(options) = layout.options {
            self.indeterminate.set_visible(true)?;
            self.indeterminate
                .set_loc(Point::new(options.origin.x + 12.0, options.origin.y + 38.0))?;
            self.indeterminate
                .set_size(Size::new((options.size.width - 24.0).max(0.0), 36.0))?;
        } else {
            self.indeterminate.set_visible(false)?;
        }
        Ok(())
    }
}
