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
        description: "Browse native WinUI extension samples and shell capabilities.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ToggleButton,
        id: "toggle-button",
        title: "ToggleButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "button", "nullable", "state"],
        description: "Three-state native ToggleButton.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RepeatButton,
        id: "repeat-button",
        title: "RepeatButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["repeat", "button", "hold", "delay", "interval"],
        description: "Native RepeatButton with explicit delay/interval and typed repeated clicks.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ToggleSwitch,
        id: "toggle-switch",
        title: "ToggleSwitch",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "switch", "boolean", "state"],
        description: "Native ToggleSwitch with typed on/off events.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RadioButtons,
        id: "radio-buttons",
        title: "RadioButtons",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["radio", "selection", "options"],
        description: "Native RadioButtons container with stable option IDs.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CalendarDatePicker,
        id: "calendar-date-picker",
        title: "CalendarDatePicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["calendar", "date", "picker", "nullable", "flyout"],
        description: "Native CalendarDatePicker reusing time::Date for nullable date selection.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CalendarView,
        id: "calendar-view",
        title: "CalendarView",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["calendar", "date", "inline", "selection", "month"],
        description: "Native inline CalendarView with a bounded single-date selection contract.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::DatePicker,
        id: "date-picker",
        title: "DatePicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["date", "picker", "calendar", "day", "month", "year"],
        description: "Native DatePicker using time::Date calendar-day values.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::DropDownButton,
        id: "drop-down-button",
        title: "DropDownButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["drop", "down", "button", "flyout", "menu"],
        description: "Native DropDownButton with Winio-owned attached flyout content.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Expander,
        id: "expander",
        title: "Expander",
        section: Some(GallerySectionId::Layout),
        keywords: &["expander", "expand", "collapse", "layout", "content"],
        description: "Native Expander hosting Winio-owned nested content.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TwoPaneView,
        id: "two-pane-view",
        title: "TwoPaneView",
        section: Some(GallerySectionId::Layout),
        keywords: &["layout", "panes", "adaptive", "split"],
        description: "Adaptive two-pane layout with wrapper-owned Winio pane containers.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::SplitView,
        id: "split-view",
        title: "SplitView",
        section: Some(GallerySectionId::Layout),
        keywords: &["pane", "content", "layout", "overlay", "inline"],
        description: "Native two-region pane/content layout with typed pane lifecycle events.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Viewbox,
        id: "viewbox",
        title: "Viewbox",
        section: Some(GallerySectionId::Layout),
        keywords: &["layout", "scale", "content"],
        description: "Native Viewbox scaling a wrapper-owned Canvas of Winio content.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Image,
        id: "image",
        title: "Image",
        section: Some(GallerySectionId::Layout),
        keywords: &["image", "bitmap", "media", "source", "stretch"],
        description: "Native Image with explicit ImageSource ownership and offline bitmap loading.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::PersonPicture,
        id: "person-picture",
        title: "PersonPicture",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["person", "picture", "avatar", "initials", "badge"],
        description: "Native PersonPicture with display-name, initials, and badge state.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::InfoBar,
        id: "info-bar",
        title: "InfoBar",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["info", "status", "message"],
        description: "Supplemental native InfoBar.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::InfoBadge,
        id: "info-badge",
        title: "InfoBadge",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["badge", "notification", "count", "value"],
        description: "Native InfoBadge with a numeric value round-trip.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::NumberBox,
        id: "number-box",
        title: "NumberBox",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["number", "numeric", "spin"],
        description: "Native NumberBox with range and inline spin buttons.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::PasswordBox,
        id: "password-box",
        title: "PasswordBox",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["password", "secure", "input", "reveal"],
        description: "Native PasswordBox with safe typed change notification.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ProgressBar,
        id: "progress-bar",
        title: "ProgressBar",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["progress", "loading", "range"],
        description: "Native ProgressBar with determinate and indeterminate state.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ProgressRing,
        id: "progress-ring",
        title: "ProgressRing",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["progress", "ring", "loading", "indeterminate"],
        description: "Native ProgressRing with determinate and indeterminate states.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RichEditBox,
        id: "rich-edit-box",
        title: "RichEditBox",
        section: Some(GallerySectionId::Text),
        keywords: &["rich", "text", "editor", "document", "formatting"],
        description: "Native RichEditBox with plain-text convenience and a full RichEditTextDocument escape hatch.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::RatingControl,
        id: "rating-control",
        title: "RatingControl",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["rating", "stars", "score", "value"],
        description: "Native RatingControl with typed value changes.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::SplitButton,
        id: "split-button",
        title: "SplitButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["split", "button", "flyout", "action"],
        description: "Native SplitButton with owned flyout content.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ToggleSplitButton,
        id: "toggle-split-button",
        title: "ToggleSplitButton",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "split", "button", "flyout", "checked"],
        description: "Native ToggleSplitButton with typed check-state and flyout lifecycle events.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ColorPicker,
        id: "color-picker",
        title: "ColorPicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["color", "picker", "rgba", "hex"],
        description: "Native compact ColorPicker with typed RGBA changes.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CommandBar,
        id: "command-bar",
        title: "CommandBar",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["command", "bar", "toolbar", "overflow", "action"],
        description: "Native CommandBar with stable primary and overflow actions.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::CommandBarFlyout,
        id: "command-bar-flyout",
        title: "CommandBarFlyout",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["command", "flyout", "toolbar", "menu", "action"],
        description: "Native CommandBarFlyout with stable primary and secondary actions.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::MenuBar,
        id: "menu-bar",
        title: "MenuBar",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["menu", "bar", "file", "help", "flyout"],
        description: "Native MenuBar with stable descriptor-backed actions.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::MenuFlyout,
        id: "menu-flyout",
        title: "MenuFlyout",
        section: Some(GallerySectionId::MenusToolbars),
        keywords: &["menu", "flyout", "context", "action"],
        description: "Native MenuFlyout with stable descriptor-backed actions.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ContentDialog,
        id: "content-dialog",
        title: "ContentDialog",
        section: Some(GallerySectionId::DialogsFlyouts),
        keywords: &["content", "dialog", "modal", "primary", "secondary"],
        description: "Native ContentDialog with Winio-owned content and typed lifecycle events.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::Flyout,
        id: "flyout",
        title: "Flyout",
        section: Some(GallerySectionId::DialogsFlyouts),
        keywords: &["flyout", "popup", "content", "anchor"],
        description: "Native Flyout with Winio-owned interactive content and lifecycle events.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TimePicker,
        id: "time-picker",
        title: "TimePicker",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["time", "picker", "clock", "duration", "minute"],
        description: "Native TimePicker using std::time::Duration for time-of-day values.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TeachingTip,
        id: "teaching-tip",
        title: "TeachingTip",
        section: Some(GallerySectionId::DialogsFlyouts),
        keywords: &["teaching", "tip", "popup"],
        description: "Native TeachingTip popup lifecycle.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::BreadcrumbBar,
        id: "breadcrumb-bar",
        title: "BreadcrumbBar",
        section: Some(GallerySectionId::Collections),
        keywords: &["breadcrumb", "path", "navigation", "hierarchy"],
        description: "Native BreadcrumbBar backed by a WinUI-bindable iterable and stable descriptor IDs.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::PipsPager,
        id: "pips-pager",
        title: "PipsPager",
        section: Some(GallerySectionId::Collections),
        keywords: &["pips", "pager", "page", "carousel", "navigation"],
        description: "Native PipsPager with explicit navigation buttons and typed selection changes.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::SelectorBar,
        id: "selector-bar",
        title: "SelectorBar",
        section: Some(GallerySectionId::Collections),
        keywords: &["selector", "tabs", "selection"],
        description: "Native SelectorBar with stable item IDs.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TabView,
        id: "tab-view",
        title: "TabView",
        section: Some(GallerySectionId::Collections),
        keywords: &["tabs", "selection", "collection"],
        description: "Native TabView with stable tab IDs and typed selection changes.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::FlipView,
        id: "flip-view",
        title: "FlipView",
        section: Some(GallerySectionId::Collections),
        keywords: &["flip", "view", "pager", "carousel", "selection"],
        description: "Native FlipView with descriptor-backed pages and typed selection changes.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::GridView,
        id: "grid-view",
        title: "GridView",
        section: Some(GallerySectionId::Collections),
        keywords: &["grid", "tiles", "selection", "collection"],
        description: "Native selectable tiles backed by stable descriptor IDs.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::ItemsView,
        id: "items-view",
        title: "ItemsView",
        section: Some(GallerySectionId::Collections),
        keywords: &["items", "list", "selection", "collection"],
        description: "Native virtualized list with stable descriptor IDs and single selection.",
    },
    GallerySampleDescriptor {
        kind: GallerySampleKind::TreeView,
        id: "tree-view",
        title: "TreeView",
        section: Some(GallerySectionId::Collections),
        keywords: &["tree", "hierarchy", "collection"],
        description: "Native TreeView with stable node IDs.",
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
