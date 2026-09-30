//! Native WinUI extensions that integrate with Winio without forking Winio.

#![cfg(windows)]

mod native;

mod controls {
    mod auto_suggest_box;
    mod info_bar;
    mod integrated_title_bar;
    mod navigation_view;
    mod number_box;
    mod teaching_tip;
    mod toggle_button;
    mod tree_view;

    pub use auto_suggest_box::{
        AutoSuggestBox, AutoSuggestBoxEvent, AutoSuggestBoxItemDescriptor, AutoSuggestBoxMessage,
        AutoSuggestBoxTextChangeReason,
    };
    pub use info_bar::{InfoBar, InfoBarMessage};
    pub use integrated_title_bar::{
        IntegratedTitleBar, IntegratedTitleBarInit, IntegratedTitleBarMessage,
    };
    pub use navigation_view::{
        NavigationView, NavigationViewEvent, NavigationViewItemDescriptor, NavigationViewMessage,
        NavigationViewPaneDisplayMode,
    };
    pub use number_box::{NumberBox, NumberBoxEvent, NumberBoxMessage, NumberBoxSpinPlacement};
    pub use teaching_tip::{
        TeachingTip, TeachingTipCloseReason, TeachingTipEvent, TeachingTipMessage,
        TeachingTipPlacement,
    };
    pub use toggle_button::{ToggleButton, ToggleButtonEvent, ToggleButtonMessage};
    pub use tree_view::{TreeView, TreeViewEvent, TreeViewMessage, TreeViewNodeDescriptor};
}

pub use controls::*;
