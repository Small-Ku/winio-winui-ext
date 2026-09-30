use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    NumberBox, NumberBoxEvent, NumberBoxSpinPlacement, ToggleButton, ToggleButtonEvent,
};

pub struct NumberBoxSample {
    page: GalleryPage,
    card: GalleryExample,
    number: Child<NumberBox>,
    step_option: Child<ToggleButton>,
    viewport: Rect,
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
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "NumberBox",
            "A numeric editor with a live step-size option.",
            "apps/gallery/src/samples/number_box.rs · winio_winui_ext::NumberBox",
            include_str!("number_box.rs"),
            true,
            true,
        )
        .await?;
        card.set_output("10")?;
        init! { number: NumberBox = (&card) => { size: Size::new(260.0, 32.0) }, step_option: ToggleButton = (&card) => { text: "Step by 1" }, }
        number.set_minimum(0.0)?;
        number.set_maximum(100.0)?;
        number.set_small_change(1.0)?;
        number.set_spin_button_placement(NumberBoxSpinPlacement::Inline)?;
        number.set_value(10.0)?;
        step_option.set_three_state(false)?;
        Ok(Self {
            page,
            card,
            number,
            step_option,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_value: true,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! { sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::SourceExpanded { card: card_id, expanded: state == Some(true) }, },
            self.number => { NumberBoxEvent::ValueChanged { old_value: _, new_value } => GallerySampleMessage::Emit(new_value.to_string()), },
            self.step_option => { ToggleButtonEvent::StateChanged(state) => GallerySampleMessage::NumberStepOption(state == Some(true)), },
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
            GallerySampleMessage::Emit(value) => {
                let initial = self.skip_initial_value && value == "10";
                self.skip_initial_value = false;
                if !initial {
                    self.card.set_output(&value)?;
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
            _ => {}
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        update_children!(self.number, self.step_option)
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.number.render()?;
        self.step_option.render()
    }
}
impl NumberBoxSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;
        self.number.set_loc(Point::new(
            layout.primary.origin.x + 20.0,
            layout.primary.origin.y + 48.0,
        ))?;
        self.number.set_size(Size::new(
            (layout.primary.size.width - 40.0).clamp(0.0, 320.0),
            self.number.preferred_size()?.height.max(32.0),
        ))?;
        if let Some(options) = layout.options {
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
