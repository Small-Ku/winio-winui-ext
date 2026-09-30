use winio::prelude::*;
use winio_winui_ext::{
    InfoBar, IntegratedTitleBar, NavigationView, NavigationViewEvent, NavigationViewItemDescriptor,
    NumberBox, NumberBoxEvent, NumberBoxSpinPlacement, TeachingTip, TeachingTipCloseReason,
    TeachingTipEvent, TeachingTipPlacement, ToggleButton, ToggleButtonEvent, TreeView,
    TreeViewEvent, TreeViewNodeDescriptor,
};

const INFO_BAR_MESSAGE: &str = "Supplemental InfoBar canary";

struct MainModel {
    window: Child<Window>,
    title_bar: Child<IntegratedTitleBar>,
    info_bar: Child<InfoBar>,
    navigation: Child<NavigationView>,
    number: Child<NumberBox>,
    teaching_tip: Child<TeachingTip>,
    tree: Child<TreeView>,
    toggle: Child<ToggleButton>,
}

#[derive(Debug)]
enum MainMessage {
    Noop,
    ToggleStateChanged(Option<bool>),
    NavigationSelected(Option<String>),
    NumberChanged(f64),
    TeachingTipClosed(TeachingTipCloseReason),
    TreeExpanded(Option<String>),
    TreeInvoked(Option<String>),
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
            title_bar: IntegratedTitleBar = (&window) => {
                size: Size::new(840.0, 48.0),
            },
            info_bar: InfoBar = (&window) => {
                size: Size::new(480.0, 80.0),
            },
            navigation: NavigationView = (&window) => {
                size: Size::new(300.0, 560.0),
            },
            number: NumberBox = (&window) => {
                size: Size::new(260.0, 44.0),
            },
            teaching_tip: TeachingTip = (&window) => {
                size: Size::new(1.0, 1.0),
            },
            tree: TreeView = (&window) => {
                size: Size::new(300.0, 230.0),
            },
            toggle: ToggleButton = (&window) => {
                text: "Native WinUI ToggleButton",
                size: Size::new(260.0, 44.0),
            },
        }

        title_bar.set_title("Winio WinUI Extensions")?;
        title_bar.set_subtitle("Native integrated WinUI title bar")?;
        assert_eq!(title_bar.title()?, "Winio WinUI Extensions");
        assert_eq!(title_bar.subtitle()?, "Native integrated WinUI title bar");
        assert!(title_bar.is_integrated()?);

        navigation.set_items(&[
            NavigationViewItemDescriptor::new("overview", "Overview"),
            NavigationViewItemDescriptor::new("details", "Details"),
        ])?;
        tree.set_nodes(&[TreeViewNodeDescriptor::new("root", "Root node")
            .with_children([TreeViewNodeDescriptor::new("leaf", "Leaf node")])])?;

        number.set_minimum(0.0)?;
        number.set_maximum(100.0)?;
        number.set_small_change(1.0)?;
        number.set_spin_button_placement(NumberBoxSpinPlacement::Inline)?;
        number.set_value(10.0)?;
        assert_eq!(number.minimum()?, 0.0);
        assert_eq!(number.maximum()?, 100.0);
        assert_eq!(number.small_change()?, 1.0);
        assert_eq!(number.value()?, 10.0);

        toggle.set_three_state(true)?;
        toggle.set_checked(None)?;
        assert_eq!(toggle.is_checked()?, None);

        info_bar.set_message(INFO_BAR_MESSAGE)?;
        info_bar.set_is_closable(false)?;
        info_bar.set_is_open(true)?;
        assert_eq!(info_bar.message()?, INFO_BAR_MESSAGE);
        assert!(info_bar.is_open()?);

        teaching_tip.set_title("Native TeachingTip canary")?;
        teaching_tip.set_subtitle("Popup lifecycle through Winio")?;
        teaching_tip.set_close_button_content("Close TeachingTip")?;
        teaching_tip.set_preferred_placement(TeachingTipPlacement::Bottom)?;
        teaching_tip.set_target(toggle.as_widget())?;
        assert_eq!(teaching_tip.title()?, "Native TeachingTip canary");
        assert_eq!(teaching_tip.subtitle()?, "Popup lifecycle through Winio");

        assert!(!teaching_tip.is_open()?);
        window.show()?;
        Ok(Self {
            window,
            title_bar,
            info_bar,
            navigation,
            number,
            teaching_tip,
            tree,
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
            self.title_bar => {},
            self.toggle => {
                ToggleButtonEvent::StateChanged(state) => MainMessage::ToggleStateChanged(state),
            },
            self.info_bar => {},
            self.navigation => {
                NavigationViewEvent::SelectionChanged { selected_id } => MainMessage::NavigationSelected(selected_id),
            },
            self.number => {
                NumberBoxEvent::ValueChanged { old_value: _, new_value } => MainMessage::NumberChanged(new_value),
            },
            self.teaching_tip => {
                TeachingTipEvent::Closed { reason } => MainMessage::TeachingTipClosed(reason),
            },
            self.tree => {
                TreeViewEvent::Expanded { id } => MainMessage::TreeExpanded(id),
                TreeViewEvent::ItemInvoked { id } => MainMessage::TreeInvoked(id),
            },
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(
            self.window,
            self.title_bar,
            self.info_bar,
            self.navigation,
            self.number,
            self.teaching_tip,
            self.tree,
            self.toggle
        )
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
                self.teaching_tip.set_is_open(true)?;
                Ok(false)
            }
            MainMessage::NavigationSelected(Some(id)) => {
                self.window
                    .set_text(format!("Winio WinUI Extensions — navigation: {id}"))?;
                Ok(false)
            }
            MainMessage::NavigationSelected(None) => Ok(true),
            MainMessage::NumberChanged(new_value) => {
                self.window
                    .set_text(format!("Winio WinUI Extensions — number: {new_value}"))?;
                Ok(false)
            }
            MainMessage::TeachingTipClosed(reason) => {
                self.window.set_text(format!(
                    "Winio WinUI Extensions — teaching tip closed: {reason:?}"
                ))?;
                Ok(false)
            }
            MainMessage::TreeExpanded(Some(id)) => {
                self.window
                    .set_text(format!("Winio WinUI Extensions — tree expanded: {id}"))?;
                Ok(false)
            }
            MainMessage::TreeInvoked(Some(id)) => {
                self.window
                    .set_text(format!("Winio WinUI Extensions — tree invoked: {id}"))?;
                Ok(false)
            }
            MainMessage::TreeExpanded(None) | MainMessage::TreeInvoked(None) => Ok(true),
            MainMessage::Close => {
                sender.output(());
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        let client = self.window.client_size()?;
        let title_bar_height = 48.0;
        self.title_bar.set_loc(Point::new(0.0, 0.0))?;
        self.title_bar
            .set_size(Size::new(client.width.max(0.0), title_bar_height))?;

        let nav_size = self.navigation.size()?;
        self.navigation
            .set_loc(Point::new(10.0, title_bar_height + 10.0))?;
        self.navigation.set_size(Size::new(
            nav_size.width.min(300.0),
            (client.height - title_bar_height - 20.0).max(0.0),
        ))?;
        let info_bar_left = 330.0;
        let info_bar_top = title_bar_height + 18.0;
        let info_bar_width = (client.width - info_bar_left - 20.0).max(0.0);
        self.info_bar
            .set_loc(Point::new(info_bar_left, info_bar_top))?;
        self.info_bar.set_size(Size::new(info_bar_width, 82.0))?;
        self.toggle
            .set_loc(Point::new(440.0, title_bar_height + 150.0))?;
        self.number
            .set_loc(Point::new(440.0, title_bar_height + 220.0))?;
        self.teaching_tip.set_loc(Point::new(0.0, 0.0))?;
        self.tree
            .set_loc(Point::new(420.0, title_bar_height + 290.0))?;
        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        self.window.render()?;
        self.title_bar.render()?;
        self.info_bar.render()?;
        self.navigation.render()?;
        self.number.render()?;
        self.teaching_tip.render()?;
        self.tree.render()?;
        self.toggle.render()
    }
}

fn main() -> Result<()> {
    App::builder()
        .name("de.kwoo.winio-winui-ext.gallery")
        .build()?
        .block_on(MainModel::run_until_event(()))
}
