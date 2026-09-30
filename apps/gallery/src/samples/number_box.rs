use crate::{
    chrome::{CardLayout, GalleryExample, GalleryExampleEvent, GalleryExampleInit},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    NumberBox, NumberBoxEvent, NumberBoxSpinPlacement, ToggleButton, ToggleButtonEvent,
};

pub struct NumberBoxSample {
    example: Child<GalleryExample>,
    number: Child<NumberBox>,
    step_option: Child<ToggleButton>,
    viewport: Rect,
    layout: CardLayout,
    source_expanded: bool,
    skip_initial_value: bool,
    generation: u64,
}

impl Component for NumberBoxSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        let mut example = Child::<GalleryExample>::init(GalleryExampleInit {
            parent,
            title: "NumberBox",
            description: "A numeric editor with a live step-size option.",
            source_path: "apps/gallery/src/samples/number_box.rs · winio_winui_ext::NumberBox",
            source: include_str!("number_box.rs"),
            has_output: true,
            has_options: true,
        })
        .await?;
        example.set_output("10")?;
        init! {
            number: NumberBox = (&example) => { size: Size::new(260.0, 32.0) },
            step_option: ToggleButton = (&example) => { text: "Step by 1" },
        }
        number.set_minimum(0.0)?;
        number.set_maximum(100.0)?;
        number.set_small_change(1.0)?;
        number.set_spin_button_placement(NumberBoxSpinPlacement::Inline)?;
        number.set_value(10.0)?;
        step_option.set_three_state(false)?;
        let layout = CardLayout::calculate(
            800.0,
            true,
            true,
            false,
            include_str!("number_box.rs").len(),
        );
        Ok(Self {
            example,
            number,
            step_option,
            viewport: Rect::zero(),
            layout,
            source_expanded: false,
            skip_initial_value: true,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.example => {
                GalleryExampleEvent::SourceExpanded(expanded) => GallerySampleMessage::SourceExpanded(expanded),
            },
            self.number => {
                NumberBoxEvent::ValueChanged { old_value: _, new_value } => GallerySampleMessage::Emit(new_value.to_string()),
            },
            self.step_option => {
                ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::NumberStepOption(state == Some(true)),
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
                    include_str!("number_box.rs").len(),
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
                    include_str!("number_box.rs").len(),
                );
                self.example.set_view(self.viewport, self.layout)?;
                self.place_controls()?;
            }
            GallerySampleMessage::Emit(value) => {
                let initial_echo = self.skip_initial_value && value == "10";
                self.skip_initial_value = false;
                if !initial_echo {
                    self.example.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "number-box",
                        value,
                    });
                }
            }
            GallerySampleMessage::NumberStepOption(step_ten) => {
                self.number
                    .set_small_change(if step_ten { 10.0 } else { 1.0 })?;
                self.step_option
                    .set_text(if step_ten { "Step by 10" } else { "Step by 1" })?;
            }
            GallerySampleMessage::ToggleThreeState(_) | GallerySampleMessage::OpenTeachingTip => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.example, self.number, self.step_option)
    }
    fn render_children(&mut self) -> Result<()> {
        self.example.render()?;
        self.number.render()?;
        self.step_option.render()
    }
}

impl NumberBoxSample {
    fn place_controls(&mut self) -> Result<()> {
        let primary = self.layout.primary;
        let height = self.number.preferred_size()?.height.max(32.0);
        self.number
            .set_loc(Point::new(primary.origin.x + 20.0, primary.origin.y + 48.0))?;
        self.number.set_size(Size::new(
            (primary.size.width - 40.0).clamp(0.0, 320.0),
            height,
        ))?;
        if let Some(options) = self.layout.options {
            self.step_option.set_visible(true)?;
            self.step_option
                .set_loc(Point::new(options.origin.x + 12.0, options.origin.y + 38.0))?;
            self.step_option
                .set_size(Size::new((options.size.width - 24.0).max(0.0), 36.0))?;
        } else {
            self.step_option.set_visible(false)?;
        }
        Ok(())
    }
}
