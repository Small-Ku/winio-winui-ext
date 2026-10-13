use windows::Foundation::EventHandler;
use windows::core::{HSTRING, IInspectable, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{BorrowedContainer, BorrowedWidget};
use winio::primitive::Failable;
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::Primitives::FlyoutBase;
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    MenuFlyout as NativeMenuFlyout, MenuFlyoutItem, MenuFlyoutItemBase,
};
use winio_winui3::Microsoft::UI::Xaml::RoutedEventHandler;

use super::menu_bar::MenuBarAction;

#[derive(Debug)]
struct ActionBinding {
    item: MenuFlyoutItem,
    click_token: i64,
}

/// A native WinUI MenuFlyout with stable descriptor-backed actions.
#[derive(Debug)]
pub struct MenuFlyout {
    native: NativeMenuFlyout,
    flyout_base: FlyoutBase,
    sender: ComponentSender<Self>,
    action_bindings: Vec<ActionBinding>,
    opened_token: i64,
    closed_token: i64,
}

impl Failable for MenuFlyout {
    type Error = Error;
}

impl MenuFlyout {
    pub fn set_actions(&mut self, actions: &[MenuBarAction]) -> Result<()> {
        remove_bindings(&mut self.action_bindings);
        let items = self.native.Items()?;
        items.Clear()?;

        for action in actions {
            let item = MenuFlyoutItem::new()?;
            item.SetText(&HSTRING::from(action.text.as_str()))?;

            let id = action.id.clone();
            let click_sender = self.sender.clone();
            let click_token = item.Click(&RoutedEventHandler::new(move |_, _| {
                click_sender.output(MenuFlyoutEvent::ActionInvoked { id: id.clone() });
                Ok(())
            }))?;

            items.Append(&item.cast::<MenuFlyoutItemBase>()?)?;
            self.action_bindings
                .push(ActionBinding { item, click_token });
        }

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

fn remove_bindings(bindings: &mut Vec<ActionBinding>) {
    for binding in bindings.drain(..) {
        let _ = binding.item.RemoveClick(binding.click_token);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MenuFlyoutEvent {
    Opened,
    Closed,
    ActionInvoked { id: String },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MenuFlyoutMessage {}

impl Component for MenuFlyout {
    type Error = Error;
    type Event = MenuFlyoutEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = MenuFlyoutMessage;

    async fn init(_parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeMenuFlyout::new()?;
        let flyout_base = native.cast::<FlyoutBase>()?;

        let opened_sender = sender.clone();
        let opened_token =
            flyout_base.Opened(&EventHandler::<IInspectable>::new(move |_, _| {
                opened_sender.output(MenuFlyoutEvent::Opened);
                Ok(())
            }))?;

        let closed_sender = sender.clone();
        let closed_token =
            flyout_base.Closed(&EventHandler::<IInspectable>::new(move |_, _| {
                closed_sender.output(MenuFlyoutEvent::Closed);
                Ok(())
            }))?;

        Ok(Self {
            native,
            flyout_base,
            sender: sender.clone(),
            action_bindings: Vec::new(),
            opened_token,
            closed_token,
        })
    }
}

impl Drop for MenuFlyout {
    fn drop(&mut self) {
        remove_bindings(&mut self.action_bindings);
        let _ = self.flyout_base.RemoveOpened(self.opened_token);
        let _ = self.flyout_base.RemoveClosed(self.closed_token);
        let _ = self.flyout_base.Hide();
    }
}
