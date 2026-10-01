//! Native WinUI extensions that integrate with Winio without forking Winio.

#![cfg(windows)]

mod native;

mod controls {
    mod auto_suggest_box;
    mod calendar_date_picker;
    mod color_picker;
    mod command_bar;
    mod content_dialog;
    mod date_picker;
    mod drop_down_button;
    mod expander;
    mod flip_view;
    mod info_badge;
    mod info_bar;
    mod integrated_title_bar;
    mod menu_bar;
    mod navigation_view;
    mod number_box;
    mod password_box;
    mod person_picture;
    mod pips_pager;
    mod progress_bar;
    mod progress_ring;
    mod radio_buttons;
    mod rating_control;
    mod selector_bar;
    mod split_button;
    mod split_view;
    mod tab_view;
    mod teaching_tip;
    mod time_picker;
    mod toggle_button;
    mod toggle_split_button;
    mod toggle_switch;
    mod tree_view;
    mod two_pane_view;
    mod viewbox;

    pub use auto_suggest_box::{
        AutoSuggestBox, AutoSuggestBoxEvent, AutoSuggestBoxItemDescriptor, AutoSuggestBoxMessage,
        AutoSuggestBoxTextChangeReason,
    };
    pub use calendar_date_picker::{
        CalendarDatePicker, CalendarDatePickerEvent, CalendarDatePickerMessage,
    };
    pub use color_picker::{ColorPicker, ColorPickerEvent, ColorPickerMessage};
    pub use command_bar::{CommandBar, CommandBarAction, CommandBarEvent, CommandBarMessage};
    pub use content_dialog::{
        ContentDialog, ContentDialogCloseResult, ContentDialogDefaultButton, ContentDialogEvent,
        ContentDialogMessage,
    };
    pub use date_picker::{Date, DatePicker, DatePickerEvent, DatePickerMessage, Month};
    pub use drop_down_button::{DropDownButton, DropDownButtonEvent, DropDownButtonMessage};
    pub use expander::{Expander, ExpanderEvent, ExpanderMessage};
    pub use flip_view::{FlipView, FlipViewEvent, FlipViewMessage, FlipViewPage};
    pub use info_badge::{InfoBadge, InfoBadgeMessage};
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
    pub use password_box::{PasswordBox, PasswordBoxEvent, PasswordBoxMessage};
    pub use person_picture::{PersonPicture, PersonPictureMessage};
    pub use pips_pager::{
        PipsPager, PipsPagerButtonVisibility, PipsPagerEvent, PipsPagerMessage, PipsPagerWrapMode,
    };
    pub use progress_bar::{ProgressBar, ProgressBarMessage};
    pub use progress_ring::{ProgressRing, ProgressRingMessage};
    pub use radio_buttons::{
        RadioButtonOption, RadioButtons, RadioButtonsEvent, RadioButtonsMessage,
    };
    pub use rating_control::{RatingControl, RatingControlEvent, RatingControlMessage};
    pub use selector_bar::{
        SelectorBar, SelectorBarEvent, SelectorBarItemDescriptor, SelectorBarMessage,
    };
    pub use split_button::{SplitButton, SplitButtonEvent, SplitButtonMessage};
    pub use split_view::{
        SplitView, SplitViewDisplayMode, SplitViewEvent, SplitViewMessage, SplitViewPanePlacement,
    };
    pub use tab_view::{TabView, TabViewEvent, TabViewMessage, TabViewTabDescriptor};
    pub use teaching_tip::{
        TeachingTip, TeachingTipCloseReason, TeachingTipEvent, TeachingTipMessage,
        TeachingTipPlacement,
    };
    pub use time_picker::{TimePicker, TimePickerEvent, TimePickerMessage};
    pub use toggle_button::{ToggleButton, ToggleButtonEvent, ToggleButtonMessage};
    pub use toggle_split_button::{
        ToggleSplitButton, ToggleSplitButtonEvent, ToggleSplitButtonMessage,
    };
    pub use toggle_switch::{ToggleSwitch, ToggleSwitchEvent, ToggleSwitchMessage};
    pub use tree_view::{TreeView, TreeViewEvent, TreeViewMessage, TreeViewNodeDescriptor};
    pub use two_pane_view::{
        TwoPaneView, TwoPaneViewMessage, TwoPaneViewMode, TwoPaneViewPriority,
        TwoPaneViewTallModeConfiguration, TwoPaneViewWideModeConfiguration,
    };
    pub use viewbox::Viewbox;
}

pub use controls::*;
