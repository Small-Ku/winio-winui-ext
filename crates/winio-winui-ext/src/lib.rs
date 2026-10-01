//! Native WinUI extensions that integrate with Winio without forking Winio.

#![cfg(windows)]

mod native;

mod controls {
    mod auto_suggest_box;
    mod color_picker;
    mod command_bar;
    mod content_dialog;
    mod drop_down_button;
    mod expander;
    mod flip_view;
    mod info_bar;
    mod integrated_title_bar;
    mod menu_bar;
    mod navigation_view;
    mod number_box;
    mod person_picture;
    mod rating_control;
    mod split_button;
    mod teaching_tip;
    mod time_picker;
    mod toggle_button;
    mod toggle_switch;
    mod tree_view;

    pub use auto_suggest_box::{
        AutoSuggestBox, AutoSuggestBoxEvent, AutoSuggestBoxItemDescriptor, AutoSuggestBoxMessage,
        AutoSuggestBoxTextChangeReason,
    };
    pub use color_picker::{ColorPicker, ColorPickerEvent, ColorPickerMessage};
    pub use command_bar::{CommandBar, CommandBarAction, CommandBarEvent, CommandBarMessage};
    pub use content_dialog::{
        ContentDialog, ContentDialogCloseResult, ContentDialogDefaultButton, ContentDialogEvent,
        ContentDialogMessage,
    };
    pub use drop_down_button::{DropDownButton, DropDownButtonEvent, DropDownButtonMessage};
    pub use expander::{Expander, ExpanderEvent, ExpanderMessage};
    pub use flip_view::{FlipView, FlipViewEvent, FlipViewMessage, FlipViewPage};
    pub use info_bar::{InfoBar, InfoBarMessage};
    pub use integrated_title_bar::{
        IntegratedTitleBar, IntegratedTitleBarEvent, IntegratedTitleBarInit,
        IntegratedTitleBarMessage,
    };
    pub use menu_bar::{MenuBar, MenuBarAction, MenuBarEvent, MenuBarMenu, MenuBarMessage};
    pub use navigation_view::{
        NavigationView, NavigationViewEvent, NavigationViewItemDescriptor, NavigationViewItemIcon,
        NavigationViewMessage, NavigationViewPaneDisplayMode,
    };
    pub use number_box::{NumberBox, NumberBoxEvent, NumberBoxMessage, NumberBoxSpinPlacement};
    pub use person_picture::{PersonPicture, PersonPictureMessage};
    pub use rating_control::{RatingControl, RatingControlEvent, RatingControlMessage};
    pub use split_button::{SplitButton, SplitButtonEvent, SplitButtonMessage};
    pub use teaching_tip::{
        TeachingTip, TeachingTipCloseReason, TeachingTipEvent, TeachingTipMessage,
        TeachingTipPlacement,
    };
    pub use time_picker::{TimePicker, TimePickerEvent, TimePickerMessage};
    pub use toggle_button::{ToggleButton, ToggleButtonEvent, ToggleButtonMessage};
    pub use toggle_switch::{ToggleSwitch, ToggleSwitchEvent, ToggleSwitchMessage};
    pub use tree_view::{TreeView, TreeViewEvent, TreeViewMessage, TreeViewNodeDescriptor};
}

pub use controls::*;
