use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    SplitView, SplitViewDisplayMode, SplitViewEvent, SplitViewPanePlacement, ToggleButton,
    ToggleButtonEvent,
};

pub struct SplitViewSample {
    page: GalleryPage,
    card: GalleryExample,
    split: Child<SplitView>,
    pane_label: Child<Label>,
    content_label: Child<Label>,
    toggle: Child<ToggleButton>,
    viewport: Rect,
    source_expanded: bool,
    event_log: Vec<&'static str>,
    generation: u64,
}

impl Component for SplitViewSample {
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
            "SplitView",
            "A native two-region pane and content layout with independent wrapper-owned Winio hosts.",
            "apps/gallery/src/samples/split_view.rs · winio_winui_ext::SplitView",
            include_str!("split_view.rs"),
            true,
            false,
        )
        .await?;

        let mut split = Child::<SplitView>::init(card.as_container()).await?;
        split.set_display_mode(SplitViewDisplayMode::Overlay)?;
        split.set_pane_placement(SplitViewPanePlacement::Right)?;
        split.set_is_pane_open(false)?;
        split.set_open_pane_length(176.0)?;
        split.set_compact_pane_length(40.0)?;
        card.set_output(format!(
            "open:{} · mode:{:?} · placement:{:?}",
            split.is_pane_open()?,
            split.display_mode()?,
            split.pane_placement()?
        ))?;

        let mut pane_label = Child::<Label>::init(split.pane_container()).await?;
        pane_label.set_text("Pane region")?;
        let mut content_label = Child::<Label>::init(split.content_container()).await?;
        content_label.set_text("Main content remains in place")?;
        let mut toggle = Child::<ToggleButton>::init(split.content_container()).await?;
        toggle.set_text("Toggle pane")?;
        toggle.set_three_state(false)?;

        Ok(Self {
            page,
            card,
            split,
            pane_label,
            content_label,
            toggle,
            viewport: Rect::zero(),
            source_expanded: false,
            event_log: Vec::new(),
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
            self.toggle => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::Emit(
                    if state == Some(true) { "toggle-open".to_owned() } else { "toggle-close".to_owned() }
                ),
            },
            self.split => {
                SplitViewEvent::PaneOpening => GallerySampleMessage::Emit("event:opening".to_owned()),
                SplitViewEvent::PaneOpened => GallerySampleMessage::Emit("event:opened".to_owned()),
                SplitViewEvent::PaneClosing { cancelled } => GallerySampleMessage::Emit(
                    if cancelled { "event:closing-cancelled".to_owned() } else { "event:closing".to_owned() }
                ),
                SplitViewEvent::PaneClosed => GallerySampleMessage::Emit("event:closed".to_owned()),
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
            GallerySampleMessage::Emit(value)
                if value == "toggle-close" || value == "toggle-open" =>
            {
                self.split.set_is_pane_open(value == "toggle-open")?;
            }
            GallerySampleMessage::Emit(value) if value.starts_with("event:") => {
                let event = match value.as_str() {
                    "event:opening" => "opening",
                    "event:opened" => "opened",
                    "event:closing" => "closing",
                    "event:closing-cancelled" => "closing-cancelled",
                    "event:closed" => "closed",
                    _ => unreachable!(),
                };
                self.event_log.push(event);
                let summary = self.event_log.join("|");
                self.card.set_output(&summary)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "split-view",
                    value: summary,
                });
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.split, self.pane_label, self.content_label, self.toggle)
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.split.render()?;
        self.pane_label.render()?;
        self.content_label.render()?;
        self.toggle.render()
    }
}

impl SplitViewSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.split.set_loc(Point::new(
            layout.primary.origin.x + 12.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.split.set_size(Size::new(
            (layout.primary.size.width - 24.0).max(0.0),
            210.0,
        ))?;
        self.pane_label.set_loc(Point::new(16.0, 22.0))?;
        self.pane_label.set_size(Size::new(140.0, 32.0))?;
        self.content_label.set_loc(Point::new(24.0, 24.0))?;
        self.content_label.set_size(Size::new(280.0, 32.0))?;
        self.toggle.set_loc(Point::new(24.0, 82.0))?;
        self.toggle.set_size(Size::new(180.0, 40.0))?;
        Ok(())
    }
}
