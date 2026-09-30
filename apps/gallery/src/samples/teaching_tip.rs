use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};
use winio::prelude::*;
use winio_winui_ext::{
    TeachingTip, TeachingTipEvent, TeachingTipPlacement, ToggleButton, ToggleButtonEvent,
};
pub struct TeachingTipSample {
    root: Child<View>,
    target: Child<ToggleButton>,
    tip: Child<TeachingTip>,
    generation: u64,
}
impl Component for TeachingTipSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;
    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        init! {root:View=(parent),target:ToggleButton=(&root)=>{text:"Open TeachingTip",size:Size::new(260.0,44.0)},tip:TeachingTip=(&root)=>{size:Size::new(1.0,1.0)}}
        tip.set_title("Native TeachingTip canary")?;
        tip.set_subtitle("Popup lifecycle through Winio")?;
        tip.set_close_button_content("Close TeachingTip")?;
        tip.set_preferred_placement(TeachingTipPlacement::Bottom)?;
        tip.set_target(target.as_widget())?;
        assert!(!tip.is_open()?);
        Ok(Self {
            root,
            target,
            tip,
            generation,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {sender,default:GallerySampleMessage::Emit(String::new()),self.root=>{},self.target=>{ToggleButtonEvent::StateChanged(_)=>GallerySampleMessage::OpenTeachingTip,},self.tip=>{TeachingTipEvent::Closed{reason}=>GallerySampleMessage::Emit(format!("closed:{reason:?}")),}}
    }
    async fn update(&mut self, m: Self::Message, s: &ComponentSender<Self>) -> Result<bool> {
        match m {
            GallerySampleMessage::SetBounds(r) => {
                self.root.set_rect(r)?;
                self.target.set_loc(Point::new(24.0, 32.0))?;
                self.target.set_size(Size::new(260.0, 44.0))?;
            }
            GallerySampleMessage::OpenTeachingTip => self.tip.set_is_open(true)?,
            GallerySampleMessage::Emit(v) => s.output(GallerySampleEvent::Output {
                generation: self.generation,
                sample_id: "teaching-tip",
                value: v,
            }),
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.root, self.target, self.tip)
    }
    fn render_children(&mut self) -> Result<()> {
        self.root.render()?;
        self.target.render()?;
        self.tip.render()
    }
}
