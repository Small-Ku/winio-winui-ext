use winio::prelude::*;
use winio_winui_ext::{
    InfoBar, NavigationView, NavigationViewEvent, NavigationViewItemDescriptor, ToggleButton,
    ToggleButtonEvent,
};

const INFO_BAR_MESSAGE: &str = "Supplemental InfoBar canary";

struct MainModel {
    window: Child<Window>,
    info_bar: Child<InfoBar>,
    navigation: Child<NavigationView>,
    toggle: Child<ToggleButton>,
}

#[derive(Debug)]
enum MainMessage {
    Noop,
    ToggleStateChanged(Option<bool>),
    NavigationSelected(Option<String>),
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
                size: Size::new(840.0, 600.0),
            },
            info_bar: InfoBar = (&window) => {
                size: Size::new(480.0, 80.0),
            },
            navigation: NavigationView = (&window) => {
                size: Size::new(300.0, 560.0),
            },
            toggle: ToggleButton = (&window) => {
                text: "Native WinUI ToggleButton",
                size: Size::new(260.0, 44.0),
            },
        }

        navigation.set_items(&[
            NavigationViewItemDescriptor::new("overview", "Overview"),
            NavigationViewItemDescriptor::new("details", "Details"),
        ])?;

        // Exercise the nullable WinRT state through the public wrapper API.
        // The smoke test verifies both this initial round-trip and a later UIA
        // state change through the native event -> ComponentSender path.
        toggle.set_three_state(true)?;
        toggle.set_checked(None)?;
        window.set_text(state_title(toggle.is_checked()?))?;

        // Exercise the generated InfoBar property setters/getters before it is
        // exposed to the UI Automation smoke test.
        info_bar.set_message(INFO_BAR_MESSAGE)?;
        info_bar.set_is_closable(false)?;
        info_bar.set_is_open(true)?;
        assert_eq!(info_bar.message()?, INFO_BAR_MESSAGE);
        assert!(info_bar.is_open()?);

        window.show()?;
        Ok(Self {
            window,
            info_bar,
            navigation,
            toggle,
        })
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
            self.info_bar => {},
            self.navigation => {
                NavigationViewEvent::SelectionChanged { selected_id } => MainMessage::NavigationSelected(selected_id),
            },
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.window, self.info_bar, self.navigation, self.toggle)
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
            MainMessage::NavigationSelected(Some(id)) => {
                self.window
                    .set_text(format!("Winio WinUI Extensions — navigation: {id}"))?;
                Ok(false)
            }
            MainMessage::NavigationSelected(None) => Ok(true),
            MainMessage::Close => {
                sender.output(());
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        let client = self.window.client_size()?;
        let nav_size = self.navigation.size()?;
        self.navigation.set_loc(Point::new(10.0, 10.0))?;
        self.navigation.set_size(Size::new(
            nav_size.width.min(300.0),
            (client.height - 20.0).max(0.0),
        ))?;
        let info_bar_left = 330.0;
        let info_bar_top = 18.0;
        let info_bar_width = (client.width - info_bar_left - 20.0).max(0.0);
        let info_bar_height = 82.0;
        self.info_bar
            .set_loc(Point::new(info_bar_left, info_bar_top))?;
        self.info_bar
            .set_size(Size::new(info_bar_width, info_bar_height))?;
        self.toggle.set_loc(Point::new(440.0, 150.0))?;
        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        self.window.render()?;
        self.info_bar.render()?;
        self.navigation.render()?;
        self.toggle.render()
    }
}

fn main() -> Result<()> {
    App::builder()
        .name("de.kwoo.winio-winui-ext.gallery")
        .build()?
        .block_on(MainModel::run_until_event(()))
}
