use winio::prelude::*;

use crate::sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage};

pub struct OverviewSample {
    page: Child<ScrollView>,
    heading: Child<Label>,
    summary: Child<Label>,
    groups: Child<Label>,
    extent: Child<Label>,
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
            page: ScrollView = (parent),
            heading: Label = (&page) => { text: "WinUI extension gallery" },
            summary: Label = (&page) => { text: "Explore native WinUI controls hosted as first-class Winio components." },
            groups: Label = (&page) => { text: "Current groups: Basic input · Status and info · Dialogs and flyouts · Collections" },
            extent: Label = (&page) => { text: "" },
        }
        Ok(Self {
            page,
            heading,
            summary,
            groups,
            extent,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! { sender, default: GallerySampleMessage::Emit(String::new()), self.page => {}, self.heading => {}, self.summary => {}, self.groups => {}, self.extent => {} }
    }

    async fn update(&mut self, message: Self::Message, _: &ComponentSender<Self>) -> Result<bool> {
        if let GallerySampleMessage::SetBounds(rect) = message {
            self.page.set_loc(rect.origin)?;
            self.page.set_size(rect.size)?;
            let width = rect.size.width.min(1064.0);
            self.heading.set_loc(Point::new(12.0, 24.0))?;
            self.heading
                .set_size(Size::new((width - 24.0).max(0.0), 38.0))?;
            self.summary.set_loc(Point::new(12.0, 76.0))?;
            self.summary
                .set_size(Size::new((width - 24.0).max(0.0), 38.0))?;
            self.groups.set_loc(Point::new(12.0, 132.0))?;
            self.groups
                .set_size(Size::new((width - 24.0).max(0.0), 42.0))?;
            self.extent.set_loc(Point::new(0.0, 220.0))?;
            self.extent.set_size(Size::new(1.0, 1.0))?;
            self.page.set_size(rect.size)?;
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(
            self.page,
            self.heading,
            self.summary,
            self.groups,
            self.extent
        )
    }
    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.heading.render()?;
        self.summary.render()?;
        self.groups.render()?;
        self.extent.render()
    }
}
