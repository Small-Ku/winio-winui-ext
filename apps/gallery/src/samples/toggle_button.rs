use crate::{
    chrome::{CardLayout, GalleryExample, GalleryExampleEvent, GalleryExampleInit},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ToggleButton, ToggleButtonEvent};

pub struct ToggleButtonSample {
    example: Child<GalleryExample>,
    toggle: Child<ToggleButton>,
    three_state: Child<ToggleButton>,
    viewport: Rect,
    layout: CardLayout,
    source_expanded: bool,
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
        let mut example = Child::<GalleryExample>::init(GalleryExampleInit {
            parent,
            title: "ToggleButton",
            description: "A nullable WinUI toggle with a live three-state option.",
            source_path: "apps/gallery/src/samples/toggle_button.rs · winio_winui_ext::ToggleButton",
            source: include_str!("toggle_button.rs"),
            has_output: true,
            has_options: true,
        }).await?;
        example.set_output("indeterminate")?;
        init! {
            toggle: ToggleButton = (&example) => { text: "Native WinUI ToggleButton" },
            three_state: ToggleButton = (&example) => { text: "Three-state: On" },
        }
        toggle.set_three_state(true)?;
        toggle.set_checked(None)?;
        three_state.set_three_state(false)?;
        three_state.set_checked(Some(true))?;
        let layout = CardLayout::calculate(
            800.0,
            true,
            true,
            false,
            include_str!("toggle_button.rs").len(),
        );
        Ok(Self {
            example,
            toggle,
            three_state,
            viewport: Rect::zero(),
            layout,
            source_expanded: false,
            skip_initial_event: true,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.example => {
                GalleryExampleEvent::SourceExpanded(expanded) => GallerySampleMessage::SourceExpanded(expanded),
            },
            self.toggle => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::Emit(state_name(state).to_owned()),
            },
            self.three_state => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::ToggleThreeState(state == Some(true)),
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
                self.layout = CardLayout::calculate(
                    viewport.size.width,
                    true,
                    true,
                    self.source_expanded,
                    include_str!("toggle_button.rs").len(),
                );
                self.example.set_view(viewport, self.layout)?;
                self.place_controls()?;
            }
            GallerySampleMessage::SourceExpanded(expanded) => {
                self.source_expanded = expanded;
                self.layout = CardLayout::calculate(
                    self.viewport.size.width,
                    true,
                    true,
                    expanded,
                    include_str!("toggle_button.rs").len(),
                );
                self.example.set_view(self.viewport, self.layout)?;
                self.place_controls()?;
            }
            GallerySampleMessage::Emit(value) => {
                if self.skip_initial_event && value == "indeterminate" {
                    self.skip_initial_event = false;
                } else {
                    self.skip_initial_event = false;
                    self.example.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "toggle-button",
                        value,
                    });
                }
            }
            GallerySampleMessage::ToggleThreeState(enabled) => {
                self.toggle.set_three_state(enabled)?;
                self.three_state.set_text(if enabled {
                    "Three-state: On"
                } else {
                    "Three-state: Off"
                })?;
                if !enabled {
                    self.toggle.set_checked(Some(false))?;
                }
            }
            GallerySampleMessage::NumberStepOption(_) | GallerySampleMessage::OpenTeachingTip => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.example, self.toggle, self.three_state)
    }

    fn render_children(&mut self) -> Result<()> {
        self.example.render()?;
        self.toggle.render()?;
        self.three_state.render()
    }
}

impl ToggleButtonSample {
    fn place_controls(&mut self) -> Result<()> {
        let primary = self.layout.primary;
        self.toggle
            .set_loc(Point::new(primary.origin.x + 20.0, primary.origin.y + 44.0))?;
        self.toggle.set_size(Size::new(
            (primary.size.width - 40.0).clamp(0.0, 320.0),
            44.0,
        ))?;
        if let Some(options) = self.layout.options {
            self.three_state.set_visible(true)?;
            self.three_state
                .set_loc(Point::new(options.origin.x + 12.0, options.origin.y + 38.0))?;
            self.three_state
                .set_size(Size::new((options.size.width - 24.0).max(0.0), 36.0))?;
        } else {
            self.three_state.set_visible(false)?;
        }
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
