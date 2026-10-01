use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::{
    MenuBar as NativeMenuBar, MenuBarItem, MenuFlyoutItem, MenuFlyoutItemBase,
};
use winio_winui3::Microsoft::UI::Xaml::RoutedEventHandler;

use crate::native::NativeElement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuBarAction {
    pub id: String,
    pub text: String,
}

impl MenuBarAction {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuBarMenu {
    pub title: String,
    pub actions: Vec<MenuBarAction>,
}

impl MenuBarMenu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            actions: Vec::new(),
        }
    }

    pub fn with_actions(mut self, actions: impl IntoIterator<Item = MenuBarAction>) -> Self {
        self.actions.extend(actions);
        self
    }
}

#[derive(Debug)]
struct ActionBinding {
    item: MenuFlyoutItem,
    click_token: i64,
}

/// A native WinUI MenuBar with descriptor-backed top-level menus and actions.
#[derive(Debug)]
pub struct MenuBar {
    native: NativeMenuBar,
    element: NativeElement,
    sender: ComponentSender<Self>,
    action_bindings: Vec<ActionBinding>,
}

impl Failable for MenuBar {
    type Error = Error;
}

impl MenuBar {
    pub fn set_menus(&mut self, menus: &[MenuBarMenu]) -> Result<()> {
        remove_bindings(&mut self.action_bindings);
        let native_menus = self.native.Items()?;
        native_menus.Clear()?;

        for menu in menus {
            let native_menu = MenuBarItem::new()?;
            native_menu.SetTitle(&HSTRING::from(menu.title.as_str()))?;
            let native_actions = native_menu.Items()?;

            for action in &menu.actions {
                let native_action = MenuFlyoutItem::new()?;
                native_action.SetText(&HSTRING::from(action.text.as_str()))?;

                let id = action.id.clone();
                let click_sender = self.sender.clone();
                let click_token = native_action.Click(&RoutedEventHandler::new(move |_, _| {
                    click_sender.output(MenuBarEvent::ActionInvoked { id: id.clone() });
                    Ok(())
                }))?;

                native_actions.Append(&native_action.cast::<MenuFlyoutItemBase>()?)?;
                self.action_bindings.push(ActionBinding {
                    item: native_action,
                    click_token,
                });
            }

            native_menus.Append(&native_menu)?;
        }
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

fn remove_bindings(bindings: &mut Vec<ActionBinding>) {
    for binding in bindings.drain(..) {
        let _ = binding.item.RemoveClick(binding.click_token);
    }
}

impl Visible for MenuBar {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for MenuBar {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for MenuBar {
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
pub enum MenuBarEvent {
    ActionInvoked { id: String },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MenuBarMessage {}

impl Component for MenuBar {
    type Error = Error;
    type Event = MenuBarEvent;
    type Init<'a> = BorrowedContainer<'a>;
    type Message = MenuBarMessage;

    async fn init(parent: Self::Init<'_>, sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativeMenuBar::new()?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;

        Ok(Self {
            native,
            element,
            sender: sender.clone(),
            action_bindings: Vec::new(),
        })
    }
}

impl AsWidget for MenuBar {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}

impl Drop for MenuBar {
    fn drop(&mut self) {
        remove_bindings(&mut self.action_bindings);
    }
}
