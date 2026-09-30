use winio_winui_ext::{
    AutoSuggestBoxItemDescriptor, NavigationViewItemDescriptor, NavigationViewItemIcon,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GallerySectionId {
    BasicInput,
    StatusInfo,
    Text,
    DialogsFlyouts,
    Layout,
    Collections,
}

impl GallerySectionId {
    pub const ALL: [Self; 6] = [
        Self::BasicInput,
        Self::StatusInfo,
        Self::Text,
        Self::DialogsFlyouts,
        Self::Layout,
        Self::Collections,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::BasicInput => "basic-input",
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
    ToggleButton,
    ToggleSwitch,
    Expander,
    InfoBar,
    NumberBox,
    RatingControl,
    TeachingTip,
    TreeView,
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
        kind: GallerySampleKind::ToggleSwitch,
        id: "toggle-switch",
        title: "ToggleSwitch",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["toggle", "switch", "boolean", "state"],
        description: "Native ToggleSwitch with typed on/off events.",
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
        kind: GallerySampleKind::InfoBar,
        id: "info-bar",
        title: "InfoBar",
        section: Some(GallerySectionId::StatusInfo),
        keywords: &["info", "status", "message"],
        description: "Supplemental native InfoBar.",
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
        kind: GallerySampleKind::RatingControl,
        id: "rating-control",
        title: "RatingControl",
        section: Some(GallerySectionId::BasicInput),
        keywords: &["rating", "stars", "score", "value"],
        description: "Native RatingControl with typed value changes.",
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
            by_id("expander").unwrap().section,
            Some(GallerySectionId::Layout)
        );
        assert_eq!(
            by_id("rating-control").unwrap().section,
            Some(GallerySectionId::BasicInput)
        );
        assert_eq!(
            by_id("info-bar").unwrap().section,
            Some(GallerySectionId::StatusInfo)
        );
        assert_eq!(
            by_id("teaching-tip").unwrap().section,
            Some(GallerySectionId::DialogsFlyouts)
        );
        assert_eq!(
            by_id("tree-view").unwrap().section,
            Some(GallerySectionId::Collections)
        );
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
