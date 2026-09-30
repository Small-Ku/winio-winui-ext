use crate::{
    chrome::{CardLayout, GalleryExample, GalleryExampleEvent, GalleryExampleInit},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{
    TeachingTip, TeachingTipEvent, TeachingTipPlacement, ToggleButton, ToggleButtonEvent,
};

pub struct TeachingTipSample {
    example: Child<GalleryExample>,
    target: Child<ToggleButton>,
    tip: Child<TeachingTip>,
    viewport: Rect,
    layout: CardLayout,
    source_expanded: bool,
    generation: u64,
}

impl Component for TeachingTipSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        let mut example = Child::<GalleryExample>::init(GalleryExampleInit {
            parent,
            title: "TeachingTip",
            description: "A native top-level popup anchored to a Winio target.",
            source_path: "apps/gallery/src/samples/teaching_tip.rs · winio_winui_ext::TeachingTip",
            source: include_str!("teaching_tip.rs"),
            has_output: true,
            has_options: false,
        })
        .await?;
        example.set_output("Closed")?;
        init! {
            target: ToggleButton = (&example) => { text: "Open TeachingTip" },
            tip: TeachingTip = (&example) => { size: Size::new(1.0, 1.0) },
        }
        tip.set_title("Native TeachingTip canary")?;
        tip.set_subtitle("Popup lifecycle through Winio")?;
        tip.set_close_button_content("Close TeachingTip")?;
        tip.set_preferred_placement(TeachingTipPlacement::Bottom)?;
        tip.set_target(target.as_widget())?;
        let layout = CardLayout::calculate(
            800.0,
            true,
            false,
            false,
            include_str!("teaching_tip.rs").len(),
        );
        Ok(Self {
            example,
            target,
            tip,
            viewport: Rect::zero(),
            layout,
            source_expanded: false,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.example => { GalleryExampleEvent::SourceExpanded(expanded) => GallerySampleMessage::SourceExpanded(expanded), },
            self.target => { ToggleButtonEvent::StateChanged(_) => GallerySampleMessage::OpenTeachingTip, },
            self.tip => { TeachingTipEvent::Closed { reason } => GallerySampleMessage::Emit(format!("closed:{reason:?}")), },
        }
    }

    async fn update(&mut self, m: Self::Message, s: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(viewport) => {
                self.viewport = viewport;
                self.layout = CardLayout::calculate(
                    viewport.size.width,
                    true,
                    false,
                    self.source_expanded,
                    include_str!("teaching_tip.rs").len(),
                );
                self.example.set_view(viewport, self.layout)?;
                self.place_target()?;
            }
            GallerySampleMessage::SourceExpanded(expanded) => {
                self.source_expanded = expanded;
                self.layout = CardLayout::calculate(
                    self.viewport.size.width,
                    true,
                    false,
                    expanded,
                    include_str!("teaching_tip.rs").len(),
                );
                self.example.set_view(self.viewport, self.layout)?;
                self.place_target()?;
            }
            GallerySampleMessage::OpenTeachingTip => {
                self.tip.set_is_open(true)?;
                let value = "opened".to_owned();
                self.example.set_output(&value)?;
                s.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "teaching-tip",
                    value,
                });
            }
            GallerySampleMessage::Emit(value) => {
                self.example.set_output(&value)?;
                s.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "teaching-tip",
                    value,
                });
            }
            GallerySampleMessage::ToggleThreeState(_)
            | GallerySampleMessage::NumberStepOption(_) => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.example, self.target, self.tip)
    }
    fn render_children(&mut self) -> Result<()> {
        self.example.render()?;
        self.target.render()?;
        self.tip.render()
    }
}

impl TeachingTipSample {
    fn place_target(&mut self) -> Result<()> {
        self.target.set_loc(Point::new(
            self.layout.primary.origin.x + 22.0,
            self.layout.primary.origin.y + 44.0,
        ))?;
        self.target.set_size(Size::new(
            (self.layout.primary.size.width - 44.0).clamp(0.0, 300.0),
            44.0,
        ))?;
        Ok(())
    }
}
