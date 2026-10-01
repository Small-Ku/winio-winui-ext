use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{RadioButtonOption, RadioButtons, RadioButtonsEvent, ToggleButtonEvent};
pub struct RadioButtonsSample {
    page: GalleryPage,
    card: GalleryExample,
    control: Child<RadioButtons>,
    viewport: Rect,
    source_expanded: bool,
    generation: u64,
}
impl Component for RadioButtonsSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;
    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let page = GalleryPage::new(init.parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "RadioButtons",
            "Select one native option; changes report its stable descriptor ID.",
            "apps/gallery/src/samples/radio_buttons.rs · winio_winui_ext::RadioButtons",
            include_str!("radio_buttons.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("choice:standard")?;
        let mut control = Child::<RadioButtons>::init(card.as_container()).await?;
        control.set_options(&[
            RadioButtonOption::new("standard", "Standard"),
            RadioButtonOption::new("compact", "Compact"),
            RadioButtonOption::new("comfortable", "Comfortable"),
        ])?;
        control.set_selected_index(0)?;
        Ok(Self {
            page,
            card,
            control,
            viewport: Rect::zero(),
            source_expanded: false,
            generation: init.generation,
        })
    }
    async fn start(&mut self, s: &ComponentSender<Self>) -> ! {
        let card = self.card.id();
        start! {s,default:GallerySampleMessage::Emit(String::new()),self.card.source_toggle=>{ToggleButtonEvent::StateChanged(v)=>GallerySampleMessage::SourceExpanded{card,expanded:v==Some(true)},},self.control=>{RadioButtonsEvent::SelectionChanged{selected_id:Some(id),..}=>GallerySampleMessage::Emit(format!("choice:{id}")),}}
    }
    async fn update(&mut self, m: Self::Message, s: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(v) => {
                self.viewport = v;
                self.reflow()?
            }
            GallerySampleMessage::SourceExpanded {
                card: CardId::MAIN,
                expanded,
            } => {
                self.source_expanded = expanded;
                self.reflow()?
            }
            GallerySampleMessage::Emit(v) if v.starts_with("choice:") => {
                self.card.set_output(&v)?;
                s.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "radio-buttons",
                    value: v,
                })
            }
            _ => {}
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.control.update().await
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.control.render()
    }
}
impl RadioButtonsSample {
    fn reflow(&mut self) -> Result<()> {
        let l = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, l)?;
        self.page.set_layout(self.viewport, l.content_height)?;
        self.control.set_loc(Point::new(
            l.primary.origin.x + 28.0,
            l.primary.origin.y + 22.0,
        ))?;
        self.control
            .set_size(Size::new((l.primary.size.width - 56.0).max(0.0), 180.0))?;
        Ok(())
    }
}
