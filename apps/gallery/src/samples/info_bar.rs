use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};
use winio::prelude::*;
use winio_winui_ext::InfoBar;
pub struct InfoBarSample {
    root: Child<View>,
    info: Child<InfoBar>,
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
        init! {root:View=(parent),info:InfoBar=(&root)=>{size:Size::new(480.0,82.0)}}
        info.set_message("Supplemental InfoBar canary")?;
        info.set_is_closable(false)?;
        info.set_is_open(true)?;
        assert_eq!(info.message()?, "Supplemental InfoBar canary");
        Ok(Self { root, info })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {sender,default:GallerySampleMessage::Emit(String::new()),self.root=>{},self.info=>{}}
    }
    async fn update(&mut self, m: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
        if let GallerySampleMessage::SetBounds(r) = m {
            self.root.set_rect(r)?;
            self.info.set_loc(Point::new(24.0, 28.0))?;
            self.info
                .set_size(Size::new((r.size.width - 48.0).max(0.0), 82.0))?;
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.root, self.info)
    }
    fn render_children(&mut self) -> Result<()> {
        self.root.render()?;
        self.info.render()
    }
}
