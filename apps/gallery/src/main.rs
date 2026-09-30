mod catalog;
mod sample;
mod samples;

use catalog::{GallerySampleKind, by_id, exact_query, navigation_items, suggestions};
use sample::{GallerySampleChild, GallerySampleEvent, GallerySampleMessage, create_sample};
use windows::core::Interface;
use winio::prelude::*;
use winio_winui_ext::{
    AutoSuggestBox, AutoSuggestBoxEvent, AutoSuggestBoxTextChangeReason, IntegratedTitleBar,
    IntegratedTitleBarEvent, NavigationView, NavigationViewEvent, NavigationViewPaneDisplayMode,
};
use winio_winui3::Microsoft::UI::Xaml::Controls::TextBlock as NativeTextBlock;

const APP_TITLE: &str = "Winio WinUI Extensions";
const TITLE_BAR_HEIGHT: f64 = 48.0;
const NAV_WIDTH: f64 = 300.0;
const NAV_COMPACT_WIDTH: f64 = 48.0;
const PAGE_MARGIN: f64 = 36.0;
const MIN_TITLEBAR_SEARCH_WIDTH: f64 = 240.0;
const PAGE_TITLE_TOP: f64 = TITLE_BAR_HEIGHT + 24.0;
const PAGE_DESCRIPTION_TOP: f64 = TITLE_BAR_HEIGHT + 68.0;
const SAMPLE_TOP: f64 = TITLE_BAR_HEIGHT + 122.0;

struct MainModel {
    window: Child<Window>,
    title_bar: Child<IntegratedTitleBar>,
    navigation: Child<NavigationView>,
    search: Child<AutoSuggestBox>,
    page_title: Child<Label>,
    page_description: Child<Label>,
    sample: GallerySampleChild,
    active_sample_id: &'static str,
    sample_generation: u64,
    content_bounds: Rect,
}

#[derive(Debug)]
enum MainMessage {
    Noop,
    Resize,
    NavigationSelected(Option<String>),
    SearchTextChanged(String),
    SearchSubmitted { query: String, id: Option<String> },
    TogglePane,
    SampleEvent(GallerySampleEvent),
    Close,
}

fn is_current_sample_event(
    active_sample_id: &str,
    active_generation: u64,
    event_sample_id: &str,
    event_generation: u64,
) -> bool {
    active_generation == event_generation && active_sample_id == event_sample_id
}

fn content_bounds(client: Size, pane_open: bool) -> Rect {
    let pane_width = if pane_open {
        NAV_WIDTH
    } else {
        NAV_COMPACT_WIDTH
    };
    let left = pane_width + PAGE_MARGIN;
    let top = SAMPLE_TOP;
    Rect::new(
        Point::new(left, top),
        Size::new(
            (client.width - left - PAGE_MARGIN).max(0.0),
            (client.height - top - PAGE_MARGIN).max(0.0),
        ),
    )
}

impl MainModel {
    async fn route(&mut self, kind: GallerySampleKind, selected_id: &'static str) -> Result<()> {
        let descriptor = by_id(selected_id).expect("catalog route must have a descriptor");
        if self.active_sample_id == descriptor.id {
            return Ok(());
        }
        let next_generation = self
            .sample_generation
            .checked_add(1)
            .expect("gallery sample generation overflow");
        let next = create_sample(kind, self.window.as_container(), next_generation).await?;
        self.sample = next;
        self.active_sample_id = descriptor.id;
        self.sample_generation = next_generation;
        self.sample
            .emit(GallerySampleMessage::SetBounds(self.content_bounds))
            .await?;
        if self.navigation.selected_id()?.as_deref() != Some(descriptor.id) {
            self.navigation.set_selected_id(descriptor.id)?;
        }
        self.page_title.set_text(descriptor.title)?;
        self.page_description.set_text(descriptor.description)?;
        self.window
            .set_text(format!("{APP_TITLE} — {}", descriptor.title))?;
        Ok(())
    }

    fn layout_shell(&mut self) -> Result<()> {
        let client = self.window.client_size()?;
        self.title_bar.set_loc(Point::new(0.0, 0.0))?;
        self.title_bar
            .set_size(Size::new(client.width.max(0.0), TITLE_BAR_HEIGHT))?;

        let pane_open = self.navigation.is_pane_open()?;
        let pane_width = if pane_open {
            NAV_WIDTH
        } else {
            NAV_COMPACT_WIDTH
        };
        self.navigation.set_loc(Point::new(0.0, TITLE_BAR_HEIGHT))?;
        self.navigation.set_size(Size::new(
            pane_width.min(client.width.max(0.0)),
            (client.height - TITLE_BAR_HEIGHT).max(0.0),
        ))?;

        let search_available = (client.width - 380.0).max(0.0);
        if search_available >= MIN_TITLEBAR_SEARCH_WIDTH {
            let search_width = search_available.min(580.0);
            let search_left = ((client.width - search_width) / 2.0)
                .max(190.0)
                .min((client.width - search_width - 150.0).max(0.0));
            let search_height = self.search.preferred_size()?.height.max(32.0);
            self.search.set_visible(true)?;
            self.search.set_loc(Point::new(
                search_left,
                ((TITLE_BAR_HEIGHT - search_height) / 2.0).max(0.0),
            ))?;
            self.search
                .set_size(Size::new(search_width, search_height))?;
        } else {
            self.search.set_visible(false)?;
        }

        let content_left = pane_width + PAGE_MARGIN;
        let content_width = (client.width - content_left - PAGE_MARGIN).max(0.0);
        self.page_title
            .set_loc(Point::new(content_left, PAGE_TITLE_TOP))?;
        self.page_title.set_size(Size::new(content_width, 38.0))?;
        self.page_description
            .set_loc(Point::new(content_left, PAGE_DESCRIPTION_TOP))?;
        self.page_description
            .set_size(Size::new(content_width.min(1064.0), 48.0))?;
        Ok(())
    }
}

impl Component for MainModel {
    type Error = Error;
    type Event = ();
    type Init<'a> = ();
    type Message = MainMessage;

    async fn init(_: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        init! {
            window: Window = (()) => { text: APP_TITLE, size: Size::new(840.0, 600.0) },
            title_bar: IntegratedTitleBar = (&window) => { size: Size::new(840.0, TITLE_BAR_HEIGHT) },
            navigation: NavigationView = (&window) => { size: Size::new(NAV_WIDTH, 500.0) },
            search: AutoSuggestBox = (&window) => { size: Size::new(580.0, 32.0) },
            page_title: Label = (&window) => { text: "Home" },
            page_description: Label = (&window) => { text: "Browse native WinUI extension samples and shell capabilities." },
        }
        title_bar.set_title(APP_TITLE)?;
        title_bar.set_subtitle("")?;
        title_bar.set_is_back_button_visible(false)?;
        title_bar.set_is_pane_toggle_button_visible(true)?;
        title_bar.set_mica_backdrop()?;
        assert!(title_bar.is_integrated()?);
        navigation.set_items(&navigation_items())?;
        navigation.set_footer_items(&[])?;
        navigation.set_is_settings_visible(false)?;
        navigation.set_is_back_button_visible(false)?;
        navigation.set_is_pane_toggle_button_visible(false)?;
        navigation.set_pane_display_mode(NavigationViewPaneDisplayMode::Left)?;
        navigation.set_is_pane_open(true)?;
        navigation.set_selected_id("overview")?;
        search.set_placeholder_text("Search controls and samples...")?;
        search.set_suggestions(&suggestions(""))?;
        let title_text = page_title
            .as_widget()
            .as_winui()
            .cast::<NativeTextBlock>()?;
        title_text.SetFontSize(28.0)?;
        let initial_generation = 0;
        let mut sample = create_sample(
            GallerySampleKind::Overview,
            window.as_container(),
            initial_generation,
        )
        .await?;
        let bounds = content_bounds(window.client_size()?, true);
        sample.emit(GallerySampleMessage::SetBounds(bounds)).await?;
        window.show()?;
        window.set_text(format!("{APP_TITLE} — Home"))?;
        Ok(Self {
            window,
            title_bar,
            navigation,
            search,
            page_title,
            page_description,
            sample,
            active_sample_id: "overview",
            sample_generation: initial_generation,
            content_bounds: bounds,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: MainMessage::Noop,
            self.window => { WindowEvent::Close => MainMessage::Close, WindowEvent::Resize => MainMessage::Resize, },
            self.title_bar => {
                IntegratedTitleBarEvent::PaneToggleRequested => MainMessage::TogglePane,
                IntegratedTitleBarEvent::BackRequested => MainMessage::Noop,
            },
            self.navigation => {
                NavigationViewEvent::SelectionChanged { selected_id, is_settings_selected: _ } => MainMessage::NavigationSelected(selected_id),
                NavigationViewEvent::BackRequested => MainMessage::Noop,
            },
            self.search => {
                AutoSuggestBoxEvent::QuerySubmitted { query_text, chosen_id } => MainMessage::SearchSubmitted { query: query_text, id: chosen_id },
                AutoSuggestBoxEvent::SuggestionChosen { .. } => MainMessage::Noop,
                AutoSuggestBoxEvent::TextChanged { text, reason: AutoSuggestBoxTextChangeReason::UserInput } => MainMessage::SearchTextChanged(text),
                AutoSuggestBoxEvent::TextChanged { .. } => MainMessage::Noop,
            },
            self.page_title => {},
            self.page_description => {},
            self.sample => {
                GallerySampleEvent::Output { generation, sample_id, value } => MainMessage::SampleEvent(GallerySampleEvent::Output { generation, sample_id, value }),
            },
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(
            self.window,
            self.title_bar,
            self.navigation,
            self.search,
            self.page_title,
            self.page_description,
            self.sample
        )
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            MainMessage::Noop => Ok(true),
            MainMessage::Resize => {
                self.content_bounds =
                    content_bounds(self.window.client_size()?, self.navigation.is_pane_open()?);
                self.sample
                    .post(GallerySampleMessage::SetBounds(self.content_bounds));
                self.layout_shell()?;
                Ok(true)
            }
            MainMessage::NavigationSelected(Some(id)) => {
                if let Some(descriptor) = by_id(&id) {
                    self.route(descriptor.kind, descriptor.id).await?;
                }
                Ok(false)
            }
            MainMessage::NavigationSelected(None) => Ok(true),
            MainMessage::SearchTextChanged(text) => {
                let items = suggestions(&text);
                self.search.set_suggestions(&items)?;
                self.search.set_is_suggestion_list_open(!items.is_empty())?;
                Ok(false)
            }
            MainMessage::SearchSubmitted { query, id } => {
                let descriptor = id
                    .as_deref()
                    .and_then(by_id)
                    .or_else(|| exact_query(&query));
                if let Some(descriptor) = descriptor {
                    self.route(descriptor.kind, descriptor.id).await?;
                }
                Ok(false)
            }
            MainMessage::TogglePane => {
                let open = self.navigation.is_pane_open()?;
                self.navigation.set_is_pane_open(!open)?;
                self.content_bounds = content_bounds(self.window.client_size()?, !open);
                self.sample
                    .post(GallerySampleMessage::SetBounds(self.content_bounds));
                self.layout_shell()?;
                Ok(true)
            }
            MainMessage::SampleEvent(GallerySampleEvent::Output {
                generation,
                sample_id,
                value,
            }) => {
                if is_current_sample_event(
                    self.active_sample_id,
                    self.sample_generation,
                    sample_id,
                    generation,
                ) {
                    let title = match sample_id {
                        "toggle-button" => format!("{APP_TITLE} — sample toggle-button: {value}"),
                        "number-box" => format!("{APP_TITLE} — sample number-box: {value}"),
                        "info-bar" => format!("{APP_TITLE} — sample info-bar: {value}"),
                        "teaching-tip" => {
                            format!("{APP_TITLE} — teaching tip {}", value.replace(':', " "))
                        }
                        "tree-view" => format!("{APP_TITLE} — tree {value}"),
                        _ => format!("{APP_TITLE} — sample {sample_id}: {value}"),
                    };
                    self.window.set_text(title)?;
                }
                Ok(false)
            }
            MainMessage::Close => {
                sender.output(());
                Ok(false)
            }
        }
    }

    fn render(&mut self, _: &ComponentSender<Self>) -> Result<()> {
        self.layout_shell()
    }

    fn render_children(&mut self) -> Result<()> {
        self.window.render()?;
        self.title_bar.render()?;
        self.navigation.render()?;
        self.search.render()?;
        self.page_title.render()?;
        self.page_description.render()?;
        self.sample.render()
    }
}

fn main() -> Result<()> {
    App::builder()
        .name("de.kwoo.winio-winui-ext.gallery")
        .build()?
        .block_on(MainModel::run_until_event(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_sample_outputs_are_rejected_by_generation_and_identity() {
        assert!(is_current_sample_event("number-box", 4, "number-box", 4));
        assert!(!is_current_sample_event("number-box", 4, "number-box", 3));
        assert!(!is_current_sample_event("number-box", 4, "tree-view", 4));
    }
}
