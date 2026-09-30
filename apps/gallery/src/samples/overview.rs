use winio::prelude::*;

use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};

pub struct OverviewSample {
    root: Child<View>,
    heading: Child<Label>,
    groups: Child<Label>,
}

impl Component for OverviewSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;
    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit {
            parent,
            generation: _,
        } = init;
        init! {
            root: View = (parent),
            heading: Label = (&root) => { text: "WinUI extension gallery" },
            groups: Label = (&root) => { text: "Current groups: Basic input · Status and info · Dialogs and flyouts · Collections" },
        }
        Ok(Self {
            root,
            heading,
            groups,
        })
    }
    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! { sender, default: GallerySampleMessage::Emit(String::new()), self.root => {}, self.heading => {}, self.groups => {} }
    }
    async fn update(&mut self, message: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
        if let GallerySampleMessage::SetBounds(rect) = message {
            self.root.set_rect(rect)?;
            self.heading.set_loc(Point::new(24.0, 30.0))?;
            self.heading
                .set_size(Size::new((rect.size.width - 48.0).max(0.0), 40.0))?;
            self.groups.set_loc(Point::new(24.0, 84.0))?;
            self.groups
                .set_size(Size::new((rect.size.width - 48.0).max(0.0), 60.0))?;
        }
        Ok(false)
    }
    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.root, self.heading, self.groups)
    }
    fn render_children(&mut self) -> Result<()> {
        self.root.render()?;
        self.heading.render()?;
        self.groups.render()
    }
}
