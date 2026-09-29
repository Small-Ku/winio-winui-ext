use winio::prelude::*;
use winio_winui_ext::{ToggleButton, ToggleButtonEvent};

struct MainModel {
    window: Child<Window>,
    toggle: Child<ToggleButton>,
}

#[derive(Debug)]
enum MainMessage {
    Noop,
    ToggleClicked,
    Close,
}

impl Component for MainModel {
    type Error = Error;
    type Event = ();
    type Init<'a> = ();
    type Message = MainMessage;

    async fn init(_init: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        init! {
            window: Window = (()) => {
                text: "Winio WinUI Extensions",
                size: Size::new(640.0, 360.0),
            },
            toggle: ToggleButton = (&window) => {
                text: "Native WinUI ToggleButton",
                size: Size::new(240.0, 44.0),
            },
        }

        window.show()?;
        Ok(Self { window, toggle })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Close => MainMessage::Close,
                WindowEvent::Resize => MainMessage::Noop,
            },
            self.toggle => {
                ToggleButtonEvent::Click => MainMessage::ToggleClicked,
            },
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.window, self.toggle)
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            MainMessage::Noop => Ok(true),
            MainMessage::ToggleClicked => {
                self.window
                    .set_text("Winio WinUI Extensions — native event received")?;
                Ok(false)
            }
            MainMessage::Close => {
                sender.output(());
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        let client = self.window.client_size()?;
        let button = self.toggle.size()?;
        self.toggle.set_loc(Point::new(
            (client.width - button.width) / 2.0,
            (client.height - button.height) / 2.0,
        ))?;
        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        self.window.render()?;
        self.toggle.render()
    }
}

fn main() -> Result<()> {
    App::builder()
        .name("de.kwoo.winio-winui-ext.gallery")
        .build()?
        .block_on(MainModel::run_until_event(()))
}
