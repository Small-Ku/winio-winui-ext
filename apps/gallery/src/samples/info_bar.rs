use crate::{
    chrome::{CardLayout, GalleryExample, GalleryExampleEvent, GalleryExampleInit},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::InfoBar;

pub struct InfoBarSample {
    example: Child<GalleryExample>,
    info: Child<InfoBar>,
    viewport: Rect,
    layout: CardLayout,
    source_expanded: bool,
}

impl Component for InfoBarSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit {
            parent,
            generation: _,
        } = init;
        let mut example = Child::<GalleryExample>::init(GalleryExampleInit {
            parent,
            title: "InfoBar",
            description: "A persistent informational status message.",
            source_path: "apps/gallery/src/samples/info_bar.rs · winio_winui_ext::InfoBar",
            source: include_str!("info_bar.rs"),
            has_output: true,
            has_options: false,
        })
        .await?;
        example.set_output("Open · informational status")?;
        init! { info: InfoBar = (&example) => { size: Size::new(480.0, 82.0) } }
        info.set_message("Supplemental InfoBar canary")?;
        info.set_is_closable(false)?;
        info.set_is_open(true)?;
        let layout =
            CardLayout::calculate(800.0, true, false, false, include_str!("info_bar.rs").len());
        Ok(Self {
            example,
            info,
            viewport: Rect::zero(),
            layout,
            source_expanded: false,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! { sender, default: GallerySampleMessage::Emit(String::new()),
            self.example => { GalleryExampleEvent::SourceExpanded(expanded) => GallerySampleMessage::SourceExpanded(expanded), },
            self.info => {},
        }
    }

    async fn update(&mut self, m: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(viewport) => {
                self.viewport = viewport;
                self.layout = CardLayout::calculate(
                    viewport.size.width,
                    true,
                    false,
                    self.source_expanded,
                    include_str!("info_bar.rs").len(),
                );
                self.example.set_view(viewport, self.layout)?;
                self.place_control()?;
            }
            GallerySampleMessage::SourceExpanded(expanded) => {
                self.source_expanded = expanded;
                self.layout = CardLayout::calculate(
                    self.viewport.size.width,
                    true,
                    false,
                    expanded,
                    include_str!("info_bar.rs").len(),
                );
                self.example.set_view(self.viewport, self.layout)?;
                self.place_control()?;
            }
            GallerySampleMessage::Emit(value) => {
                if !value.is_empty() {
                    self.example.set_output(value)?;
                }
            }
            GallerySampleMessage::OpenTeachingTip
            | GallerySampleMessage::ToggleThreeState(_)
            | GallerySampleMessage::NumberStepOption(_) => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.example, self.info)
    }
    fn render_children(&mut self) -> Result<()> {
        self.example.render()?;
        self.info.render()
    }
}

impl InfoBarSample {
    fn place_control(&mut self) -> Result<()> {
        self.info.set_loc(Point::new(
            self.layout.primary.origin.x + 18.0,
            self.layout.primary.origin.y + 32.0,
        ))?;
        self.info.set_size(Size::new(
            (self.layout.primary.size.width - 36.0).max(0.0),
            82.0,
        ))?;
        Ok(())
    }
}
