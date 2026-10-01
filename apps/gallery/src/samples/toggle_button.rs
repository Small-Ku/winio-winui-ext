use crate::{
    chrome::{CardId, GalleryExample, GalleryPage, stack_cards},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ToggleButton, ToggleButtonEvent};

pub struct ToggleButtonSample {
    page: GalleryPage,
    three_card: GalleryExample,
    basic_card: GalleryExample,
    three_toggle: Child<ToggleButton>,
    three_option: Child<ToggleButton>,
    basic_toggle: Child<ToggleButton>,
    viewport: Rect,
    three_source_expanded: bool,
    basic_source_expanded: bool,
    skip_initial_event: bool,
    generation: u64,
}

impl Component for ToggleButtonSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        let page = GalleryPage::new(parent).await?;
        let mut three_card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "ToggleButton — three-state",
            "A nullable WinUI toggle with a live three-state option.",
            "apps/gallery/src/samples/toggle_button.rs · three-state example",
            include_str!("toggle_button.rs"),
            true,
            true,
        )
        .await?;
        let mut basic_card = GalleryExample::new(
            page.as_container(),
            CardId::BASIC,
            "ToggleButton — basic",
            "A separate two-state ToggleButton with its own output and source disclosure.",
            "apps/gallery/src/samples/toggle_button.rs · basic example",
            include_str!("toggle_button.rs"),
            true,
            false,
        )
        .await?;
        three_card.set_output("indeterminate")?;
        basic_card.set_output("unchecked")?;
        init! {
            three_toggle: ToggleButton = (&three_card) => { text: "Three-state sample" },
            three_option: ToggleButton = (&three_card) => { text: "Three-state: On" },
            basic_toggle: ToggleButton = (&basic_card) => { text: "Two-state sample" },
        }
        three_toggle.set_three_state(true)?;
        three_toggle.set_checked(None)?;
        three_option.set_three_state(false)?;
        three_option.set_checked(Some(true))?;
        basic_toggle.set_three_state(false)?;
        basic_toggle.set_checked(Some(false))?;
        Ok(Self {
            page,
            three_card,
            basic_card,
            three_toggle,
            three_option,
            basic_toggle,
            viewport: Rect::zero(),
            three_source_expanded: false,
            basic_source_expanded: false,
            skip_initial_event: true,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let three_card_id = self.three_card.id();
        let basic_card_id = self.basic_card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.three_card.source_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded { card: three_card_id, expanded: state == Some(true) }, },
            self.basic_card.source_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded { card: basic_card_id, expanded: state == Some(true) }, },
            self.three_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::Emit(state_name(state).to_owned()), },
            self.three_option => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::ToggleThreeState(state == Some(true)), },
            self.basic_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::ExampleChanged { card: CardId::BASIC, value: state_name(state).to_owned() }, },
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
            GallerySampleMessage::SourceExpanded { card, expanded } => {
                if card == CardId::MAIN {
                    self.three_source_expanded = expanded;
                } else if card == CardId::BASIC {
                    self.basic_source_expanded = expanded;
                }
                self.reflow()?;
            }
            GallerySampleMessage::Emit(value) => {
                if self.skip_initial_event && value == "indeterminate" {
                    self.skip_initial_event = false;
                } else {
                    self.skip_initial_event = false;
                    self.three_card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "toggle-button",
                        value,
                    });
                }
            }
            GallerySampleMessage::ExampleChanged {
                card: CardId::BASIC,
                value,
            } => {
                self.basic_card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "toggle-button",
                    value: format!("basic {value}"),
                });
            }
            GallerySampleMessage::ToggleThreeState(enabled) => {
                self.three_toggle.set_three_state(enabled)?;
                self.three_option.set_text(if enabled {
                    "Three-state: On"
                } else {
                    "Three-state: Off"
                })?;
                if !enabled {
                    self.three_toggle.set_checked(Some(false))?;
                }
            }
            GallerySampleMessage::NumberStepOption(_)
            | GallerySampleMessage::OpenContentDialog
            | GallerySampleMessage::OpenTeachingTip
            | GallerySampleMessage::ExampleChanged { .. } => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.three_card.update_children().await?;
        self.basic_card.update_children().await?;
        update_children!(self.three_toggle, self.three_option, self.basic_toggle)
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.three_card.render()?;
        self.basic_card.render()?;
        self.three_toggle.render()?;
        self.three_option.render()?;
        self.basic_toggle.render()
    }
}

impl ToggleButtonSample {
    fn reflow(&mut self) -> Result<()> {
        let width = self.viewport.size.width.min(1064.0);
        let a = self
            .three_card
            .calculate_layout(width, self.three_source_expanded);
        let b = self
            .basic_card
            .calculate_layout(width, self.basic_source_expanded);
        let (tops, extent) = stack_cards(&[a, b]);
        self.three_card.set_layout(tops[0], a)?;
        self.basic_card.set_layout(tops[1], b)?;
        self.page.set_layout(self.viewport, extent)?;
        self.three_toggle.set_loc(Point::new(
            a.primary.origin.x + 20.0,
            a.primary.origin.y + 44.0,
        ))?;
        self.three_toggle.set_size(Size::new(
            (a.primary.size.width - 40.0).clamp(0.0, 320.0),
            44.0,
        ))?;
        if let Some(options) = a.options {
            self.three_option.set_visible(true)?;
            self.three_option
                .set_loc(Point::new(options.origin.x + 12.0, options.origin.y + 38.0))?;
            self.three_option
                .set_size(Size::new((options.size.width - 24.0).max(0.0), 36.0))?;
        } else {
            self.three_option.set_visible(false)?;
        }
        self.basic_toggle.set_loc(Point::new(
            b.primary.origin.x + 20.0,
            b.primary.origin.y + 44.0,
        ))?;
        self.basic_toggle.set_size(Size::new(
            (b.primary.size.width - 40.0).clamp(0.0, 320.0),
            44.0,
        ))?;
        Ok(())
    }
}

fn state_name(state: Option<bool>) -> &'static str {
    match state {
        Some(true) => "checked",
        Some(false) => "unchecked",
        None => "indeterminate",
    }
}
