//! Native WinUI extensions that integrate with Winio without forking Winio.

#![cfg(windows)]

mod native;

mod controls {
    mod info_bar;
    mod navigation_view;
    mod number_box;
    mod teaching_tip;
    mod toggle_button;
    mod tree_view;

    pub use info_bar::{InfoBar, InfoBarMessage};
    pub use navigation_view::{
        NavigationView, NavigationViewEvent, NavigationViewItemDescriptor, NavigationViewMessage,
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
