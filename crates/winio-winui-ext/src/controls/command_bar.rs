use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    AppBarButton, CommandBar as NativeCommandBar, ICommandBarElement,
};
use winio_winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use crate::native::NativeElement;

/// One stable action hosted by a CommandBar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandBarAction {
    pub id: String,
    pub label: String,
}

impl CommandBarAction {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug)]
struct ActionBinding {
    button: MUXC::Button,
    click_token: i64,
}

/// A native WinUI CommandBar with stable primary and overflow actions.
///
/// Actions are descriptors rather than Winio child widgets because WinUI owns
/// CommandBar elements through its PrimaryCommands/SecondaryCommands vectors.
#[derive(Debug)]
pub struct CommandBar {
    native: NativeCommandBar,
    element: NativeElement,
    sender: ComponentSender<Self>,
    primary_bindings: Vec<ActionBinding>,
    secondary_bindings: Vec<ActionBinding>,
}

impl Failable for CommandBar {
    type Error = Error;
}

impl CommandBar {
    pub fn set_primary_actions(&mut self, actions: &[CommandBarAction]) -> Result<()> {
        replace_actions(
            self.native.PrimaryCommands()?,
            actions,
            &self.sender,
            &mut self.primary_bindings,
        )
    }

    pub fn set_secondary_actions(&mut self, actions: &[CommandBarAction]) -> Result<()> {
        replace_actions(
            self.native.SecondaryCommands()?,
            actions,
            &self.sender,
            &mut self.secondary_bindings,
        )
    }

    pub fn is_open(&self) -> Result<bool> {
        self.native.IsOpen()
    }

    pub fn set_is_open(&mut self, value: bool) -> Result<()> {
        self.native.SetIsOpen(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

fn replace_actions(
    commands: windows::Foundation::Collections::IObservableVector<ICommandBarElement>,
    actions: &[CommandBarAction],
    sender: &ComponentSender<CommandBar>,
    bindings: &mut Vec<ActionBinding>,
) -> Result<()> {
    remove_bindings(bindings);
    commands.Clear()?;

    for action in actions {
        let native = AppBarButton::new()?;
        native.SetLabel(&HSTRING::from(action.label.as_str()))?;

        let button = native.cast::<MUXC::Button>()?;
        let id = action.id.clone();
        let click_sender = sender.clone();
        let click_token = button.Click(&RoutedEventHandler::new(move |_, _| {
            click_sender.output(CommandBarEvent::ActionInvoked { id: id.clone() });
            Ok(())
        }))?;

        commands.Append(&native.cast::<ICommandBarElement>()?)?;
        bindings.push(ActionBinding {
            button,
            click_token,
        });
    }
    Ok(())
}

fn remove_bindings(bindings: &mut Vec<ActionBinding>) {
    for binding in bindings.drain(..) {
        let _ = binding.button.RemoveClick(binding.click_token);
    }
}

impl Visible for CommandBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for CommandBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for CommandBar {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }

    fn set_loc(&mut self, point: Point) -> Result<()> {
        self.element.set_loc(point)
    }

    fn size(&self) -> Result<Size> {
        self.element.size()
    }

    fn set_size(&mut self, size: Size) -> Result<()> {
        self.element.set_size(size)
    }

    fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CommandBarEvent {
    ActionInvoked { id: String },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CommandBarMessage {}

impl Component for CommandBar {
    type Error = Error;
    type Event = CommandBarEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = CommandBarMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeCommandBar::new()?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            sender: sender.clone(),
            primary_bindings: Vec::new(),
            secondary_bindings: Vec::new(),
        })
    }
}

impl AsWidget for CommandBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for CommandBar {
    fn drop(&mut self) {
        remove_bindings(&mut self.primary_bindings);
        remove_bindings(&mut self.secondary_bindings);
    }
}
