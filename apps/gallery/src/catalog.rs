use winio_winui_ext::{
    AutoSuggestBoxItemDescriptor, NavigationViewItemDescriptor, NavigationViewItemIcon,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GallerySectionId {
    BasicInput,
    MenusToolbars,
    StatusInfo,
    Text,
    DialogsFlyouts,
    Layout,
    Collections,
}

impl GallerySectionId {
    pub const ALL: [Self; 7] = [
        Self::BasicInput,
        Self::MenusToolbars,
        Self::StatusInfo,
        Self::Text,
        Self::DialogsFlyouts,
        Self::Layout,
        Self::Collections,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::BasicInput => "basic-input",
            Self::MenusToolbars => "menus-toolbars",
            Self::StatusInfo => "status-info",
            Self::Text => "text",
            Self::DialogsFlyouts => "dialogs-flyouts",
            Self::Layout => "layout",
            Self::Collections => "collections",
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::BasicInput => "Basic input",
            Self::MenusToolbars => "Menus and toolbars",
            Self::StatusInfo => "Status and info",
            Self::Text => "Text",
            Self::DialogsFlyouts => "Dialogs and flyouts",
            Self::Layout => "Layout",
            Self::Collections => "Collections",
        }
    }

    pub const fn icon(self) -> NavigationViewItemIcon {
        match self {
            Self::BasicInput => NavigationViewItemIcon::Edit,
            Self::MenusToolbars => NavigationViewItemIcon::List,
            Self::StatusInfo => NavigationViewItemIcon::Info,
            Self::Text => NavigationViewItemIcon::Document,
            Self::DialogsFlyouts => NavigationViewItemIcon::Comment,
            Self::Layout => NavigationViewItemIcon::ViewAll,
            Self::Collections => NavigationViewItemIcon::List,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GallerySampleKind {
    Overview,
    CommandBar,
    CommandBarFlyout,
    BreadcrumbBar,
    CalendarDatePicker,
    CalendarView,
    ColorPicker,
    ContentDialog,
    DatePicker,
    DropDownButton,
    ToggleButton,
    ToggleSwitch,
    ToggleSplitButton,
    Expander,
    FlipView,
    Flyout,
    GridView,
    Image,
    ItemsView,
    InfoBar,
    InfoBadge,
    MenuBar,
    MenuFlyout,
    NumberBox,
    PersonPicture,
    PipsPager,
    ProgressBar,
    ProgressRing,
    PasswordBox,
    RatingControl,
    RepeatButton,
    RichEditBox,
    RadioButtons,
    SplitButton,
    SplitView,
    SelectorBar,
    TeachingTip,
    TimePicker,
    TreeView,
    TwoPaneView,
    Viewbox,
    TabView,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GallerySampleDescriptor {
    pub kind: GallerySampleKind,
    pub id: &'static str,
    pub title: &'static str,
    pub section: Option<GallerySectionId>,
    pub keywords: &'static [&'static str],
    pub description: &'static str,
}

pub const CATALOG: &[GallerySampleDescriptor] = &[
    GallerySampleDescriptor {
        kind: GallerySampleKind::Overview,
        id: "overview",
        title: "Home",
        section: None,
        keywords: &["gallery", "samples"],
        description: "Explore WinUI controls running inside a Winio application.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ToggleButton,
        id: "toggle-button",
        title: "ToggleButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "button", "nullable", "state"],
        description: "Switch between on, off, and indeterminate states.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RepeatButton,
        id: "repeat-button",
        title: "RepeatButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["repeat", "button", "hold", "delay", "interval"],
        description: "Press and hold to trigger repeated clicks after a configurable delay.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ToggleSwitch,
        id: "toggle-switch",
        title: "ToggleSwitch",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "switch", "boolean", "state"],
        description: "Turn a switch on or off and see its state change.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RadioButtons,
        id: "radio-buttons",
        title: "RadioButtons",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["radio", "selection", "options"],
        description: "Choose a radio option and read its stable ID.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CalendarDatePicker,
        id: "calendar-date-picker",
        title: "CalendarDatePicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["calendar", "date", "picker", "nullable", "flyout"],
        description: "Choose a date or clear the selected date.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CalendarView,
        id: "calendar-view",
        title: "CalendarView",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["calendar", "date", "inline", "selection", "month"],
        description: "Select one date from an inline calendar.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::DatePicker,
        id: "date-picker",
        title: "DatePicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["date", "picker", "calendar", "day", "month", "year"],
        description: "Choose a date using the native picker.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::DropDownButton,
        id: "drop-down-button",
        title: "DropDownButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["drop", "down", "button", "flyout", "menu"],
        description: "Open a flyout containing Winio controls.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Expander,
        id: "expander",
        title: "Expander",
        section: Some(GallerySectionId::Layout),
        keywords: &["expander", "expand", "collapse", "layout", "content"],
        description: "Expand or collapse content containing Winio controls.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TwoPaneView,
        id: "two-pane-view",
        title: "TwoPaneView",
        section: Some(GallerySectionId::Layout),
        keywords: &["layout", "panes", "adaptive", "split"],
        description: "Resize a layout containing Winio controls in two panes.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::SplitView,
        id: "split-view",
        title: "SplitView",
        section: Some(GallerySectionId::Layout),
        keywords: &["pane", "content", "layout", "overlay", "inline"],
        description: "Open and close a side pane next to the main content.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Viewbox,
        id: "viewbox",
        title: "Viewbox",
        section: Some(GallerySectionId::Layout),
        keywords: &["layout", "scale", "content"],
        description: "Scale a Winio control inside a Viewbox.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Image,
        id: "image",
        title: "Image",
        section: Some(GallerySectionId::Layout),
        keywords: &["image", "bitmap", "media", "source", "stretch"],
        description: "Display an image from a bitmap or URI.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::PersonPicture,
        id: "person-picture",
        title: "PersonPicture",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["person", "picture", "avatar", "initials", "badge"],
        description: "Display a name, initials, and a badge.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::InfoBar,
        id: "info-bar",
        title: "InfoBar",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["info", "status", "message"],
        description: "Show a message that can be closed.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::InfoBadge,
        id: "info-badge",
        title: "InfoBadge",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["badge", "notification", "count", "value"],
        description: "Display a numeric badge and read its value.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::NumberBox,
        id: "number-box",
        title: "NumberBox",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["number", "numeric", "spin"],
        description: "Enter a number or change it using spin buttons.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::PasswordBox,
        id: "password-box",
        title: "PasswordBox",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["password", "secure", "input", "reveal"],
        description: "Enter a password; the sample reports its length, not its contents.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ProgressBar,
        id: "progress-bar",
        title: "ProgressBar",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["progress", "loading", "range"],
        description: "Show a measured value or indeterminate progress.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ProgressRing,
        id: "progress-ring",
        title: "ProgressRing",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["progress", "ring", "loading", "indeterminate"],
        description: "Show progress as a ring with a value or animation.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RichEditBox,
        id: "rich-edit-box",
        title: "RichEditBox",
        section: Some(GallerySectionId::Text),
        keywords: &["rich", "text", "editor", "document", "formatting"],
        description: "Edit formatted text and access its native RichEditTextDocument.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RatingControl,
        id: "rating-control",
        title: "RatingControl",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["rating", "stars", "score", "value"],
        description: "Set a rating and read the selected value.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::SplitButton,
        id: "split-button",
        title: "SplitButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["split", "button", "flyout", "action"],
        description: "Run the primary action or open its flyout.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ToggleSplitButton,
        id: "toggle-split-button",
        title: "ToggleSplitButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "split", "button", "flyout", "checked"],
        description: "Toggle the button state or open its flyout.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ColorPicker,
        id: "color-picker",
        title: "ColorPicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["color", "picker", "rgba", "hex"],
        description: "Choose a color and read its RGBA value.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CommandBar,
        id: "command-bar",
        title: "CommandBar",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["command", "bar", "toolbar", "overflow", "action"],
        description: "Invoke primary commands or open overflow actions.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CommandBarFlyout,
        id: "command-bar-flyout",
        title: "CommandBarFlyout",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["command", "flyout", "toolbar", "menu", "action"],
        description: "Open a command flyout with primary and secondary actions.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::MenuBar,
        id: "menu-bar",
        title: "MenuBar",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["menu", "bar", "file", "help", "flyout"],
        description: "Open a menu and invoke a command.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::MenuFlyout,
        id: "menu-flyout",
        title: "MenuFlyout",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["menu", "flyout", "context", "action"],
        description: "Open a flyout menu and invoke a command.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ContentDialog,
        id: "content-dialog",
        title: "ContentDialog",
        section: Some(GallerySectionId::DialogsFlyouts),
        keywords: &["content", "dialog", "modal", "primary", "secondary"],
        description: "Open a dialog containing Winio controls and handle its result.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Flyout,
        id: "flyout",
        title: "Flyout",
        section: Some(GallerySectionId::DialogsFlyouts),
        keywords: &["flyout", "popup", "content", "anchor"],
        description: "Open a popup containing interactive Winio controls.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TimePicker,
        id: "time-picker",
        title: "TimePicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["time", "picker", "clock", "duration", "minute"],
        description: "Choose a time of day using the native picker.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TeachingTip,
        id: "teaching-tip",
        title: "TeachingTip",
        section: Some(GallerySectionId::DialogsFlyouts),
        keywords: &["teaching", "tip", "popup"],
        description: "Open a teaching tip and close it with the native button.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::BreadcrumbBar,
        id: "breadcrumb-bar",
        title: "BreadcrumbBar",
        section: Some(GallerySectionId::Collections),
        keywords: &["breadcrumb", "path", "navigation", "hierarchy"],
        description: "Navigate a path using native breadcrumb items.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::PipsPager,
        id: "pips-pager",
        title: "PipsPager",
        section: Some(GallerySectionId::Collections),
        keywords: &["pips", "pager", "page", "carousel", "navigation"],
        description: "Change pages using previous and next buttons.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::SelectorBar,
        id: "selector-bar",
        title: "SelectorBar",
        section: Some(GallerySectionId::Collections),
        keywords: &["selector", "tabs", "selection"],
        description: "Select an item and read its stable ID.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TabView,
        id: "tab-view",
        title: "TabView",
        section: Some(GallerySectionId::Collections),
        keywords: &["tabs", "selection", "collection"],
        description: "Select tabs and host Winio controls inside a tab page.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::FlipView,
        id: "flip-view",
        title: "FlipView",
        section: Some(GallerySectionId::Collections),
        keywords: &["flip", "view", "pager", "carousel", "selection"],
        description: "Move between pages and read the selected page.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::GridView,
        id: "grid-view",
        title: "GridView",
        section: Some(GallerySectionId::Collections),
        keywords: &["grid", "tiles", "selection", "collection"],
        description: "Select a tile and read its stable ID.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ItemsView,
        id: "items-view",
        title: "ItemsView",
        section: Some(GallerySectionId::Collections),
        keywords: &["items", "list", "selection", "collection"],
        description: "Select one list item and read its stable ID.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TreeView,
        id: "tree-view",
        title: "TreeView",
        section: Some(GallerySectionId::Collections),
        keywords: &["tree", "hierarchy", "collection"],
        description: "Expand a tree node and invoke a leaf item.",
    },
];

pub fn by_id(id: &str) -> Option<&'static GallerySampleDescriptor> {
    CATALOG
        .iter()
        .find(|entry| entry.id.eq_ignore_ascii_case(id))
}

pub fn exact_query(query: &str) -> Option<&'static GallerySampleDescriptor> {
    let query = query.trim();
    CATALOG.iter().find(|entry| {
        entry.id.eq_ignore_ascii_case(query) || entry.title.eq_ignore_ascii_case(query)
    })
}

pub fn search(query: &str) -> Vec<&'static GallerySampleDescriptor> {
    let needle = query.trim().to_lowercase();
    CATALOG
        .iter()
        .filter(|entry| {
            needle.is_empty()
                || entry.title.to_lowercase().contains(&needle)
                || entry.id.to_lowercase().contains(&needle)
                || entry
                    .keywords
                    .iter()
                    .any(|word| word.to_lowercase().contains(&needle))
        })
        .collect()
}

pub fn navigation_items() -> Vec<NavigationViewItemDescriptor> {
    let home = by_id("overview").expect("overview route must exist");
    let mut items = vec![
        NavigationViewItemDescriptor::new(home.id, home.title)
            .with_icon(NavigationViewItemIcon::Home),
    ];
    for section in GallerySectionId::ALL {
        let children = CATALOG
            .iter()
            .filter(|entry| entry.section == Some(section))
            .map(|entry| NavigationViewItemDescriptor::new(entry.id, entry.title))
            .collect::<Vec<_>>();
        if !children.is_empty() {
            items.push(
                NavigationViewItemDescriptor::new(section.id(), section.title())
                    .with_icon(section.icon())
                    .with_children(children),
            );
        }
    }
    items
}

pub fn suggestions(query: &str) -> Vec<AutoSuggestBoxItemDescriptor> {
    search(query)
        .into_iter()
        .map(|entry| AutoSuggestBoxItemDescriptor::new(entry.id, entry.title))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_ids_are_unique_and_membership_is_stable() {
        let ids = CATALOG.iter().map(|entry| entry.id).collect::<HashSet<_>>();
        assert_eq!(ids.len(), CATALOG.len());
        assert_eq!(by_id("overview").unwrap().section, None);
        assert_eq!(
            by_id("toggle-button").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("number-box").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("toggle-switch").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("calendar-date-picker").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("date-picker").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("drop-down-button").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("expander").unwrap().section,
            Some(GallerySectionId::Layout)
        );
        assert_eq!(
            by_id("two-pane-view").unwrap().section,
            Some(GallerySectionId::Layout)
        );
        assert_eq!(
            by_id("split-view").unwrap().section,
            Some(GallerySectionId::Layout)
        );
        assert_eq!(
            by_id("viewbox").unwrap().section,
            Some(GallerySectionId::Layout)
        );
        assert_eq!(
            by_id("rating-control").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("split-button").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("toggle-split-button").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("password-box").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("progress-bar").unwrap().section,
            Some(GallerySectionId::StatusInfo)
        );
        assert_eq!(
            by_id("progress-ring").unwrap().section,
            Some(GallerySectionId::StatusInfo)
        );
        assert_eq!(
            by_id("person-picture").unwrap().section,
            Some(GallerySectionId::StatusInfo)
        );
        assert_eq!(
            by_id("info-bar").unwrap().section,
            Some(GallerySectionId::StatusInfo)
        );
        assert_eq!(
            by_id("menu-bar").unwrap().section,
            Some(GallerySectionId::MenusToolbars)
        );
        assert_eq!(
            by_id("color-picker").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("command-bar").unwrap().section,
            Some(GallerySectionId::MenusToolbars)
        );
        assert_eq!(
            by_id("time-picker").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("teaching-tip").unwrap().section,
            Some(GallerySectionId::DialogsFlyouts)
        );
        assert_eq!(
            by_id("content-dialog").unwrap().section,
            Some(GallerySectionId::DialogsFlyouts)
        );
        assert_eq!(
            by_id("breadcrumb-bar").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("pips-pager").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("flip-view").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("grid-view").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("items-view").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("tree-view").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("info-badge").unwrap().section,
            Some(GallerySectionId::StatusInfo)
        );
        assert_eq!(
            by_id("radio-buttons").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("selector-bar").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        assert_eq!(
            by_id("tab-view").unwrap().section,
            Some(GallerySectionId::Collections)
        );
        for (id, kind) in [
            ("info-badge", GallerySampleKind::InfoBadge),
            ("progress-bar", GallerySampleKind::ProgressBar),
            ("two-pane-view", GallerySampleKind::TwoPaneView),
            ("split-view", GallerySampleKind::SplitView),
            ("viewbox", GallerySampleKind::Viewbox),
            ("tab-view", GallerySampleKind::TabView),
            ("toggle-split-button", GallerySampleKind::ToggleSplitButton),
            ("progress-bar", GallerySampleKind::ProgressBar),
            ("radio-buttons", GallerySampleKind::RadioButtons),
            ("selector-bar", GallerySampleKind::SelectorBar),
            ("grid-view", GallerySampleKind::GridView),
            ("items-view", GallerySampleKind::ItemsView),
        ] {
            assert_eq!(by_id(id).unwrap().kind, kind, "catalog kind for {id}");
        }
        assert!(
            navigation_items()
                .iter()
                .any(|item| item.id == "basic-input")
        );
    }

    #[test]
    fn query_search_and_exact_title_route_from_catalog() {
        assert_eq!(search("tree")[0].id, "tree-view");
        assert_eq!(exact_query("NumberBox").unwrap().id, "number-box");
    }
}
