use winio::prelude::*;
use winio_winui_ext::{ToggleButton, ToggleButtonEvent};

struct MainModel {
    window: Child<Window>,
    toggle: Child<ToggleButton>,
}

#[derive(Debug)]
enum MainMessage {
    Noop,
    ToggleStateChanged(Option<bool>),
    Close,
}

fn state_title(state: Option<bool>) -> &'static str {
    match state {
        Some(true) => "Winio WinUI Extensions — state: checked",
        Some(false) => "Winio WinUI Extensions — state: unchecked",
        None => "Winio WinUI Extensions — state: indeterminate",
    }
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

        // Exercise the nullable WinRT state through the public wrapper API.
        // The smoke test verifies both this initial round-trip and a later UIA
        // state change through the native event -> ComponentSender path.
        toggle.set_three_state(true)?;
        toggle.set_checked(None)?;
        window.set_text(state_title(toggle.is_checked()?))?;

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
                ToggleButtonEvent::StateChanged(state) => MainMessage::ToggleStateChanged(state),
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
            MainMessage::ToggleStateChanged(state) => {
                self.window.set_text(state_title(state))?;
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
