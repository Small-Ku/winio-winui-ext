use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};
use winio::prelude::*;
use winio_winui_ext::{NumberBox, NumberBoxEvent, NumberBoxSpinPlacement};
pub struct NumberBoxSample {
    root: Child<View>,
    number: Child<NumberBox>,
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
        init! {root:View=(parent),number:NumberBox=(&root)=>{size:Size::new(260.0,44.0)}}
        number.set_minimum(0.0)?;
        number.set_maximum(100.0)?;
        number.set_small_change(1.0)?;
        number.set_spin_button_placement(NumberBoxSpinPlacement::Inline)?;
        number.set_value(10.0)?;
        assert_eq!(number.value()?, 10.0);
        Ok(Self {
            root,
            number,
            skip_initial_value: true,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {sender,default:GallerySampleMessage::Emit(String::new()),self.root=>{},self.number=>{NumberBoxEvent::ValueChanged{old_value:_,new_value}=>GallerySampleMessage::Emit(new_value.to_string()),}}
    }
    async fn update(&mut self, m: Self::Message, s: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(r) => {
                self.root.set_rect(r)?;
                self.number.set_loc(Point::new(24.0, 32.0))?;
                self.number.set_size(Size::new(260.0, 44.0))?;
            }
            GallerySampleMessage::Emit(v) => {
                let initial_echo = self.skip_initial_value && v == "10";
                self.skip_initial_value = false;
                if !initial_echo {
                    s.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "number-box",
                        value: v,
                    });
                }
            }
            GallerySampleMessage::OpenTeachingTip => {}
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.root, self.number)
    }
    fn render_children(&mut self) -> Result<()> {
        self.root.render()?;
        self.number.render()
    }
}
