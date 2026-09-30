use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};
use winio::prelude::*;
use winio_winui_ext::{ToggleButton, ToggleButtonEvent};

pub struct ToggleButtonSample {
    root: Child<View>,
    toggle: Child<ToggleButton>,
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
        init! { root: View=(parent), toggle: ToggleButton=(&root) => { text: "Native WinUI ToggleButton", size: Size::new(260.0,44.0) } }
        toggle.set_three_state(true)?;
        toggle.set_checked(None)?;
        assert_eq!(toggle.is_checked()?, None);
        Ok(Self {
            root,
            toggle,
            skip_initial_event: true,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {sender,default:GallerySampleMessage::Emit(String::new()),self.root=>{},self.toggle=>{ToggleButtonEvent::StateChanged(state)=>GallerySampleMessage::Emit(match state{Some(true)=>"checked",Some(false)=>"unchecked",None=>"indeterminate"}.into()),}}
    }
    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            GallerySampleMessage::SetBounds(rect) => {
                self.root.set_rect(rect)?;
                self.toggle.set_loc(Point::new(24.0, 32.0))?;
                self.toggle.set_size(Size::new(260.0, 44.0))?;
            }
            GallerySampleMessage::Emit(value) => {
                if self.skip_initial_event && value == "indeterminate" {
                    // set_checked(None) can deliver its native change event after start().
                    self.skip_initial_event = false;
                } else {
                    self.skip_initial_event = false;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "toggle-button",
                        value,
                    });
                }
            }
            GallerySampleMessage::OpenTeachingTip => {}
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.root, self.toggle)
    }
    fn render_children(&mut self) -> Result<()> {
        self.root.render()?;
        self.toggle.render()
    }
}
