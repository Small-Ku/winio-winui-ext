use windows::core::{HSTRING, Interface};
use winio::prelude::*;
use winio_winui_bindings::Microsoft::UI::Xaml::{Controls::Border, CornerRadius};
use winio_winui_ext::{ToggleButton, ToggleButtonEvent};
use winio_winui3::Microsoft::UI::Xaml::{self as MUX, Controls as MUXC};

const PAGE_MAX_WIDTH: f64 = 1064.0;
const CARD_PADDING: f64 = 12.0;
const CARD_GAP: f64 = 16.0;
const NARROW_BREAKPOINT: f64 = 820.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardLayout {
    pub width: f64,
    pub primary: Rect,
    pub output: Option<Rect>,
    pub options: Option<Rect>,
    pub source_toggle: Rect,
    pub source: Option<Rect>,
    pub content_height: f64,
    pub wide: bool,
}

impl CardLayout {
    pub fn calculate(
        viewport_width: f64,
        has_output: bool,
        has_options: bool,
        source_expanded: bool,
        source_chars: usize,
    ) -> Self {
        let width = viewport_width.clamp(0.0, PAGE_MAX_WIDTH);
        let inner = (width - 2.0 * CARD_PADDING).max(0.0);
        let wide = width >= NARROW_BREAKPOINT && (has_output || has_options);
        let mut output = None;
        let mut options = None;
        let top = 92.0;
        let (primary, section_bottom) = if wide {
            let main_width = (inner * 0.62).min(inner - 300.0).max(250.0);
            let side_left = CARD_PADDING + main_width + CARD_GAP;
            let side_width = (inner - main_width - CARD_GAP).max(220.0);
            let primary_height = 176.0;
            let mut side_y = top;
            if has_output {
                output = Some(Rect::new(
                    Point::new(side_left, side_y),
                    Size::new(side_width, 86.0),
                ));
                side_y += 86.0 + CARD_GAP;
            }
            if has_options {
                options = Some(Rect::new(
                    Point::new(side_left, side_y),
                    Size::new(side_width, 86.0),
                ));
                side_y += 86.0;
            }
            let primary = Rect::new(
                Point::new(CARD_PADDING, top),
                Size::new(main_width, primary_height),
            );
            (primary, (top + primary_height).max(side_y))
        } else {
            let primary = Rect::new(Point::new(CARD_PADDING, top), Size::new(inner, 132.0));
            let mut y = top + 132.0 + CARD_GAP;
            if has_output {
                output = Some(Rect::new(
                    Point::new(CARD_PADDING, y),
                    Size::new(inner, 76.0),
                ));
                y += 76.0 + CARD_GAP;
            }
            if has_options {
                options = Some(Rect::new(
                    Point::new(CARD_PADDING, y),
                    Size::new(inner, 76.0),
                ));
                y += 76.0;
            }
            (primary, y)
        };
        let source_toggle = Rect::new(
            Point::new(CARD_PADDING, section_bottom + CARD_GAP),
            Size::new(inner, 40.0),
        );
        let source = source_expanded.then(|| {
            let chars_per_line = (inner / 8.0).max(16.0) as usize;
            let lines = (source_chars.div_ceil(chars_per_line))
                .max(source_chars.div_ceil(80))
                .max(1);
            Rect::new(
                Point::new(CARD_PADDING, source_toggle.max_y() + 8.0),
                Size::new(inner, (lines as f64 * 18.0 + 28.0).max(64.0)),
            )
        });
        let content_height = source.map_or(source_toggle.max_y() + CARD_PADDING, |rect| {
            rect.max_y() + CARD_PADDING
        });
        Self {
            width,
            primary,
            output,
            options,
            source_toggle,
            source,
            content_height,
            wide,
        }
    }
}

pub struct GalleryExampleInit<'a> {
    pub parent: BorrowedContainer<'a>,
    pub title: &'static str,
    pub description: &'static str,
    pub source_path: &'static str,
    pub source: &'static str,
    pub has_output: bool,
    pub has_options: bool,
}

#[derive(Debug)]
pub enum GalleryExampleMessage {
    Output(String),
    SourceExpanded(bool),
}

#[derive(Debug)]
pub enum GalleryExampleEvent {
    SourceExpanded(bool),
}

pub struct GalleryExample {
    scroll: Child<ScrollView>,
    title: Child<Label>,
    description: Child<Label>,
    output_heading: Child<Label>,
    output_value: Child<Label>,
    options_heading: Child<Label>,
    source_path: Child<Label>,
    source_toggle: Child<ToggleButton>,
    extent: Child<Label>,
    card_background: NativeBorder,
    primary_background: NativeBorder,
    output_background: NativeBorder,
    options_background: NativeBorder,
    source_background: NativeBorder,
    source_panel: NativeText,
    source: &'static str,
    has_output: bool,
    has_options: bool,
}

impl Component for GalleryExample {
    type Error = Error;
    type Event = GalleryExampleEvent;
    type Init<'a> = GalleryExampleInit<'a>;
    type Message = GalleryExampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GalleryExampleInit {
            parent,
            title: title_text,
            description: description_text,
            source_path: source_path_text,
            source,
            has_output,
            has_options,
        } = init;
        let mut scroll = Child::<ScrollView>::init(parent).await?;
        scroll.set_hscroll(false)?;
        let card_background = NativeBorder::new(scroll.as_container())?;
        let primary_background = NativeBorder::new(scroll.as_container())?;
        let output_background = NativeBorder::new(scroll.as_container())?;
        let options_background = NativeBorder::new(scroll.as_container())?;
        let source_background = NativeBorder::new(scroll.as_container())?;
        init! {
            title: Label = (&scroll) => { text: title_text },
            description: Label = (&scroll) => { text: description_text },
            output_heading: Label = (&scroll) => { text: "Output" },
            output_value: Label = (&scroll) => { text: "Ready" },
            options_heading: Label = (&scroll) => { text: "Options" },
            source_path: Label = (&scroll) => { text: source_path_text },
            source_toggle: ToggleButton = (&scroll) => { text: "Source code" },
            extent: Label = (&scroll) => { text: "" },
        }
        source_toggle.set_three_state(false)?;
        source_toggle.set_checked(Some(false))?;
        let source_panel = NativeText::new(scroll.as_container(), source)?;
        source_panel.set_visible(false)?;
        Ok(Self {
            scroll,
            title,
            description,
            output_heading,
            output_value,
            options_heading,
            source_path,
            source_toggle,
            extent,
            card_background,
            primary_background,
            output_background,
            options_background,
            source_background,
            source_panel,
            source,
            has_output,
            has_options,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: GalleryExampleMessage::Output(String::new()),
            self.scroll => {}, self.title => {}, self.description => {},
            self.output_heading => {}, self.output_value => {}, self.options_heading => {},
            self.source_path => {}, self.extent => {},
            self.source_toggle => {
                ToggleButtonEvent::StateChanged(state) => GalleryExampleMessage::SourceExpanded(state == Some(true)),
            },
        }
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            GalleryExampleMessage::Output(value) => self.output_value.set_text(value)?,
            GalleryExampleMessage::SourceExpanded(expanded) => {
                let _ = self.has_output;
                let _ = self.has_options;
                let width = self.scroll.size()?.width;
                let layout = CardLayout::calculate(
                    width,
                    self.has_output,
                    self.has_options,
                    expanded,
                    self.source.len(),
                );
                self.source_panel.set_visible(expanded)?;
                self.source_background.set_visible(expanded)?;
                if let Some(rect) = layout.source {
                    self.source_panel.set_loc(rect.origin)?;
                    self.source_panel.set_size(rect.size)?;
                    self.source_background.set_bounds(rect)?;
                }
                self.card_background.set_bounds(Rect::new(
                    Point::new(0.0, 0.0),
                    Size::new(layout.width, layout.content_height),
                ))?;
                self.extent
                    .set_loc(Point::new(0.0, layout.content_height))?;
                self.extent.set_size(Size::new(1.0, 1.0))?;
                let size = self.scroll.size()?;
                self.scroll.set_size(size)?;
                sender.output(GalleryExampleEvent::SourceExpanded(expanded));
            }
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        update_children!(
            self.scroll,
            self.title,
            self.description,
            self.output_heading,
            self.output_value,
            self.options_heading,
            self.source_path,
            self.source_toggle,
            self.extent
        )
    }

    fn render_children(&mut self) -> Result<()> {
        self.scroll.render()?;
        self.title.render()?;
        self.description.render()?;
        self.output_heading.render()?;
        self.output_value.render()?;
        self.options_heading.render()?;
        self.source_path.render()?;
        self.source_toggle.render()?;
        self.extent.render()
    }
}

impl AsContainer for GalleryExample {
    fn as_container(&self) -> BorrowedContainer<'_> {
        self.scroll.as_container()
    }
}

impl GalleryExample {
    pub fn set_view(&mut self, viewport: Rect, layout: CardLayout) -> Result<()> {
        self.scroll.set_loc(viewport.origin)?;
        self.scroll.set_size(viewport.size)?;
        self.card_background.set_bounds(Rect::new(
            Point::new(0.0, 0.0),
            Size::new(layout.width, layout.content_height),
        ))?;
        self.primary_background.set_bounds(layout.primary)?;
        self.output_background
            .set_visible(layout.output.is_some())?;
        if let Some(rect) = layout.output {
            self.output_background.set_bounds(rect)?;
        }
        self.options_background
            .set_visible(layout.options.is_some())?;
        if let Some(rect) = layout.options {
            self.options_background.set_bounds(rect)?;
        }
        self.source_background
            .set_visible(layout.source.is_some())?;
        if let Some(rect) = layout.source {
            self.source_background.set_bounds(rect)?;
        }
        self.title.set_loc(Point::new(12.0, 16.0))?;
        self.title
            .set_size(Size::new((layout.width - 24.0).max(0.0), 36.0))?;
        self.description.set_loc(Point::new(12.0, 52.0))?;
        self.description
            .set_size(Size::new((layout.width - 24.0).max(0.0), 32.0))?;
        if let Some(rect) = layout.output {
            self.output_heading.set_visible(true)?;
            self.output_value.set_visible(true)?;
            self.output_heading
                .set_loc(Point::new(rect.origin.x + 12.0, rect.origin.y + 10.0))?;
            self.output_heading
                .set_size(Size::new((rect.size.width - 24.0).max(0.0), 24.0))?;
            self.output_value
                .set_loc(Point::new(rect.origin.x + 12.0, rect.origin.y + 38.0))?;
            self.output_value
                .set_size(Size::new((rect.size.width - 24.0).max(0.0), 36.0))?;
        } else {
            self.output_heading.set_visible(false)?;
            self.output_value.set_visible(false)?;
        }
        if let Some(rect) = layout.options {
            self.options_heading.set_visible(true)?;
            self.options_heading
                .set_loc(Point::new(rect.origin.x + 12.0, rect.origin.y + 10.0))?;
            self.options_heading
                .set_size(Size::new((rect.size.width - 24.0).max(0.0), 24.0))?;
        } else {
            self.options_heading.set_visible(false)?;
        }
        self.source_path
            .set_loc(Point::new(12.0, layout.source_toggle.origin.y - 22.0))?;
        self.source_path
            .set_size(Size::new((layout.width - 24.0).max(0.0), 18.0))?;
        self.source_toggle.set_loc(layout.source_toggle.origin)?;
        self.source_toggle.set_size(layout.source_toggle.size)?;
        if let Some(rect) = layout.source {
            self.source_panel.set_visible(true)?;
            self.source_panel.set_loc(rect.origin)?;
            self.source_panel.set_size(rect.size)?;
        } else {
            self.source_panel.set_visible(false)?;
        }
        self.extent
            .set_loc(Point::new(0.0, layout.content_height))?;
        self.extent.set_size(Size::new(1.0, 1.0))?;
        let size = self.scroll.size()?;
        self.scroll.set_size(size)?;
        Ok(())
    }

    pub fn set_output(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.output_value.set_text(value)
    }
}

struct NativeText {
    element: MUX::FrameworkElement,
    canvas: windows::core::Weak<MUXC::Canvas>,
}

impl NativeText {
    fn new(parent: BorrowedContainer<'_>, text: &str) -> Result<Self> {
        let xaml = HSTRING::from(
            r#"<TextBlock xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" FontFamily="Consolas" FontSize="13" TextWrapping="Wrap" />"#,
        );
        let block = MUX::Markup::XamlReader::Load(&xaml)?.cast::<MUXC::TextBlock>()?;
        block.SetText(&HSTRING::from(text))?;
        let canvas = parent.as_winui();
        canvas.Children()?.Append(&block)?;
        Ok(Self {
            element: block.cast()?,
            canvas: canvas.downgrade()?,
        })
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        self.element.SetVisibility(if visible {
            MUX::Visibility::Visible
        } else {
            MUX::Visibility::Collapsed
        })?;
        Ok(())
    }

    fn set_loc(&self, p: Point) -> Result<()> {
        MUXC::Canvas::SetLeft(&self.element, p.x)?;
        MUXC::Canvas::SetTop(&self.element, p.y)?;
        Ok(())
    }

    fn set_size(&self, size: Size) -> Result<()> {
        self.element.SetWidth(size.width)?;
        self.element.SetHeight(size.height)?;
        Ok(())
    }
}

impl Drop for NativeText {
    fn drop(&mut self) {
        let Some(canvas) = self.canvas.upgrade() else {
            return;
        };
        let Ok(children) = canvas.Children() else {
            return;
        };
        let mut index = 0;
        if children.IndexOf(&self.element, &mut index).is_ok() {
            let _ = children.RemoveAt(index);
        }
    }
}

struct NativeBorder {
    element: Border,
    canvas: windows::core::Weak<MUXC::Canvas>,
}

impl NativeBorder {
    fn new(parent: BorrowedContainer<'_>) -> Result<Self> {
        let border = Border::new()?;
        let resources = MUX::Application::Current()?.Resources()?;
        let fill_key = windows::Foundation::PropertyValue::CreateString(&HSTRING::from(
            "CardBackgroundFillColorDefaultBrush",
        ))?;
        let stroke_key = windows::Foundation::PropertyValue::CreateString(&HSTRING::from(
            "CardStrokeColorDefaultBrush",
        ))?;
        let fill = resources.Lookup(&fill_key)?.cast::<MUX::Media::Brush>()?;
        let stroke = resources.Lookup(&stroke_key)?.cast::<MUX::Media::Brush>()?;
        border.SetBackground(&fill)?;
        border.SetBorderBrush(&stroke)?;
        border.SetBorderThickness(MUX::Thickness {
            Left: 1.0,
            Top: 1.0,
            Right: 1.0,
            Bottom: 1.0,
        })?;
        border.SetCornerRadius(CornerRadius {
            TopLeft: 8.0,
            TopRight: 8.0,
            BottomRight: 8.0,
            BottomLeft: 8.0,
        })?;
        let canvas = parent.as_winui();
        canvas.Children()?.Append(&border)?;
        Ok(Self {
            element: border,
            canvas: canvas.downgrade()?,
        })
    }

    fn set_bounds(&self, rect: Rect) -> Result<()> {
        let element = self.element.cast::<MUX::FrameworkElement>()?;
        MUXC::Canvas::SetLeft(&element, rect.origin.x)?;
        MUXC::Canvas::SetTop(&element, rect.origin.y)?;
        element.SetWidth(rect.size.width)?;
        element.SetHeight(rect.size.height)?;
        self.set_visible(true)
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        self.element
            .cast::<MUX::FrameworkElement>()?
            .SetVisibility(if visible {
                MUX::Visibility::Visible
            } else {
                MUX::Visibility::Collapsed
            })?;
        Ok(())
    }
}

impl Drop for NativeBorder {
    fn drop(&mut self) {
        let Some(canvas) = self.canvas.upgrade() else {
            return;
        };
        let Ok(children) = canvas.Children() else {
            return;
        };
        let mut index = 0;
        if children.IndexOf(&self.element, &mut index).is_ok() {
            let _ = children.RemoveAt(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_cards_keep_auxiliary_regions_beside_the_primary_example() {
        let layout = CardLayout::calculate(1064.0, true, true, false, 2_000);
        assert!(layout.wide);
        let output = layout.output.unwrap();
        let options = layout.options.unwrap();
        assert!(output.origin.x > layout.primary.origin.x);
        assert!(options.origin.x == output.origin.x);
        assert!(options.origin.y >= output.max_y());
        assert!(layout.width <= PAGE_MAX_WIDTH);
    }

    #[test]
    fn narrow_cards_stack_auxiliary_regions_and_source_increases_extent() {
        let collapsed = CardLayout::calculate(560.0, true, true, false, 4_000);
        let expanded = CardLayout::calculate(560.0, true, true, true, 4_000);
        assert!(!collapsed.wide);
        assert_eq!(
            collapsed.primary.origin.x,
            collapsed.output.unwrap().origin.x
        );
        assert_eq!(
            collapsed.primary.origin.x,
            collapsed.options.unwrap().origin.x
        );
        assert!(collapsed.output.unwrap().origin.y > collapsed.primary.max_y());
        assert!(collapsed.options.unwrap().origin.y > collapsed.output.unwrap().max_y());
        assert!(expanded.source.unwrap().origin.y > collapsed.source_toggle.max_y());
        assert!(expanded.content_height > collapsed.content_height);
    }
}
