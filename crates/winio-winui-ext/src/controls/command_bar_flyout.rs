use windows::Foundation::EventHandler;
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{BorrowedContainer, BorrowedWidget};
use winio::primitive::Failable;
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::FlyoutBase;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    AppBarButton, CommandBarFlyout as NativeCommandBarFlyout, ICommandBarElement,
};
use winio_winui3::Microsoft::UI::Xaml::{Controls as MUXC, RoutedEventHandler};

use super::command_bar::CommandBarAction;

#[derive(Debug)]
struct ActionBinding {
    button: MUXC::Button,
    click_token: i64,
}

/// A native WinUI CommandBarFlyout with stable primary and secondary actions.
#[derive(Debug)]
pub struct CommandBarFlyout {
    native: NativeCommandBarFlyout,
    flyout_base: FlyoutBase,
    sender: ComponentSender<Self>,
    primary_bindings: Vec<ActionBinding>,
    secondary_bindings: Vec<ActionBinding>,
    opened_token: i64,
    closed_token: i64,
}

impl Failable for CommandBarFlyout {
    type Error = Error;
}

impl CommandBarFlyout {
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

    pub fn always_expanded(&self) -> Result<bool> {
        self.native.AlwaysExpanded()
    }

    pub fn set_always_expanded(&mut self, value: bool) -> Result<()> {
        self.native.SetAlwaysExpanded(value)?;
        Ok(())
    }

    pub fn show_at(&self, target: BorrowedWidget<'_>) -> Result<()> {
        self.flyout_base.ShowAt(target.as_winui())?;
        Ok(())
    }

    pub fn hide(&self) -> Result<()> {
        self.flyout_base.Hide()?;
        Ok(())
    }
}

fn replace_actions(
    commands: windows::Foundation::Collections::IObservableVector<ICommandBarElement>,
    actions: &[CommandBarAction],
    sender: &ComponentSender<CommandBarFlyout>,
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
            click_sender.output(CommandBarFlyoutEvent::ActionInvoked { id: id.clone() });
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CommandBarFlyoutEvent {
    Opened,
    Closed,
    ActionInvoked { id: String },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CommandBarFlyoutMessage {}

impl Component for CommandBarFlyout {
    type Error = Error;
    type Event = CommandBarFlyoutEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = CommandBarFlyoutMessage;

    async fn init(_parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeCommandBarFlyout::new()?;
        let flyout_base = native.cast::<FlyoutBase>()?;

        let opened_sender = sender.clone();
        let opened_token =
            flyout_base.Opened(&EventHandler::<IInspectable>::new(move |_, _| {
                opened_sender.output(CommandBarFlyoutEvent::Opened);
                Ok(())
            }))?;

        let closed_sender = sender.clone();
        let closed_token =
            flyout_base.Closed(&EventHandler::<IInspectable>::new(move |_, _| {
                closed_sender.output(CommandBarFlyoutEvent::Closed);
                Ok(())
            }))?;

        Ok(Self {
            native,
            flyout_base,
            sender: sender.clone(),
            primary_bindings: Vec::new(),
            secondary_bindings: Vec::new(),
            opened_token,
            closed_token,
        })
    }
}

impl Drop for CommandBarFlyout {
    fn drop(&mut self) {
        remove_bindings(&mut self.primary_bindings);
        remove_bindings(&mut self.secondary_bindings);
        let _ = self.flyout_base.RemoveOpened(self.opened_token);
        let _ = self.flyout_base.RemoveClosed(self.closed_token);
        let _ = self.flyout_base.Hide();
    }
}
