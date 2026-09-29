//! Native WinUI extensions that integrate with Winio without forking Winio.

#![cfg(windows)]

mod native;

mod controls {
    mod info_bar;
    mod navigation_view;
    mod toggle_button;

    pub use info_bar::{InfoBar, InfoBarMessage};
    pub use navigation_view::{
        NavigationView, NavigationViewEvent, NavigationViewItemDescriptor, NavigationViewMessage,
    };
    pub use toggle_button::{ToggleButton, ToggleButtonEvent, ToggleButtonMessage};
}

pub use controls::*;
