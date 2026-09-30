use windows::core::{HSTRING, Interface};
use winio::prelude::*;
use winio_winui_bindings::Microsoft::UI::Xaml::{Controls::Border, CornerRadius};
use winio_winui_ext::ToggleButton;
use winio_winui3::Microsoft::UI::Xaml::{self as MUX, Controls as MUXC};

const PAGE_MAX_WIDTH: f64 = 1064.0;
const CARD_PADDING: f64 = 12.0;
const CARD_GAP: f64 = 16.0;
const NARROW_BREAKPOINT: f64 = 820.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardId(&'static str);

impl CardId {
    pub const MAIN: Self = Self::new("main");
    pub const BASIC: Self = Self::new("basic");

    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }
}

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
            (
                Rect::new(
                    Point::new(CARD_PADDING, top),
                    Size::new(main_width, primary_height),
                ),
                (top + primary_height).max(side_y),
            )
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
            let lines = source_chars
                .div_ceil(chars_per_line)
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

pub fn page_width(viewport_width: f64) -> f64 {
    viewport_width.clamp(0.0, PAGE_MAX_WIDTH)
}

pub fn stack_cards(layouts: &[CardLayout]) -> (Vec<f64>, f64) {
    let mut tops = Vec::with_capacity(layouts.len());
    let mut y = 0.0;
    for (index, layout) in layouts.iter().enumerate() {
        if index > 0 {
            y += CARD_GAP;
        }
        tops.push(y);
        y += layout.content_height;
    }
    (tops, y)
}

/// Owns the single vertical scroll viewport for a routed control page.
pub struct GalleryPage {
    scroll: Child<ScrollView>,
    extent: Child<Label>,
}

impl GalleryPage {
    pub async fn new(parent: BorrowedContainer<'_>) -> Result<Self> {
        init! { scroll: ScrollView = (parent), extent: Label = (&scroll) => { text: "" } }
        scroll.set_hscroll(false)?;
        Ok(Self { scroll, extent })
    }

    pub fn as_container(&self) -> BorrowedContainer<'_> {
        self.scroll.as_container()
    }

    pub fn set_layout(&mut self, viewport: Rect, content_height: f64) -> Result<()> {
        self.scroll.set_loc(viewport.origin)?;
        self.scroll.set_size(viewport.size)?;
        self.extent.set_loc(Point::new(0.0, content_height))?;
        self.extent
            .set_size(Size::new(page_width(viewport.size.width).max(1.0), 1.0))?;
        self.scroll.set_size(viewport.size)?;
        Ok(())
    }

    pub async fn update_children(&mut self) -> Result<bool> {
        update_children!(self.scroll, self.extent)
    }
    pub fn render(&mut self) -> Result<()> {
        self.scroll.render()?;
        self.extent.render()
    }
}

impl AsContainer for GalleryPage {
    fn as_container(&self) -> BorrowedContainer<'_> {
        self.scroll.as_container()
    }
}

/// Card-local canvas, presentation chrome, source disclosure, and output state.
/// Controls are constructed with this card as their container from the outset.
pub struct GalleryExample {
    root: Child<View>,
    title: Child<Label>,
    description: Child<Label>,
    output_heading: Child<Label>,
    output_value: Child<Label>,
    options_heading: Child<Label>,
    source_path: Child<Label>,
    pub source_toggle: Child<ToggleButton>,
    card_background: NativeBorder,
    primary_background: NativeBorder,
    output_background: NativeBorder,
    options_background: NativeBorder,
    source_background: NativeBorder,
    source_panel: NativeText,
    id: CardId,
    has_output: bool,
    has_options: bool,
    source_chars: usize,
}

impl GalleryExample {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        parent: BorrowedContainer<'_>,
        id: CardId,
        title_text: &'static str,
        description_text: &'static str,
        source_path_text: &'static str,
        source: &'static str,
        has_output: bool,
        has_options: bool,
    ) -> Result<Self> {
        let root = Child::<View>::init(parent).await?;
        let card_background = NativeBorder::new(root.as_container())?;
        let primary_background = NativeBorder::new(root.as_container())?;
        let output_background = NativeBorder::new(root.as_container())?;
        let options_background = NativeBorder::new(root.as_container())?;
        let source_background = NativeBorder::new(root.as_container())?;
        init! {
            title: Label = (&root) => { text: title_text },
            description: Label = (&root) => { text: description_text },
            output_heading: Label = (&root) => { text: "Output" },
            output_value: Label = (&root) => { text: "Ready" },
            options_heading: Label = (&root) => { text: "Options" },
            source_path: Label = (&root) => { text: source_path_text },
            source_toggle: ToggleButton = (&root) => { text: format!("Source code — {title_text}") },
        }
        source_toggle.set_three_state(false)?;
        source_toggle.set_checked(Some(false))?;
        let source_panel = NativeText::new(root.as_container(), source)?;
        source_panel.set_visible(false)?;
        Ok(Self {
            root,
            title,
            description,
            output_heading,
            output_value,
            options_heading,
            source_path,
            source_toggle,
            card_background,
            primary_background,
            output_background,
            options_background,
            source_background,
            source_panel,
            id,
            has_output,
            has_options,
            source_chars: source.len(),
        })
    }

    pub fn id(&self) -> CardId {
        self.id
    }

    pub fn calculate_layout(&self, width: f64, source_expanded: bool) -> CardLayout {
        CardLayout::calculate(
            width,
            self.has_output,
            self.has_options,
            source_expanded,
            self.source_chars,
        )
    }

    pub fn set_layout(&mut self, top: f64, layout: CardLayout) -> Result<()> {
        self.set_layout_at(Point::new(0.0, top), layout)
    }

    pub fn set_layout_at(&mut self, origin: Point, layout: CardLayout) -> Result<()> {
        self.root.set_loc(origin)?;
        self.root
            .set_size(Size::new(layout.width, layout.content_height))?;
        self.card_background.set_bounds(Rect::new(
            Point::zero(),
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
            self.source_panel.set_visible(true)?;
            self.source_panel.set_loc(rect.origin)?;
            self.source_panel.set_size(rect.size)?;
        } else {
            self.source_panel.set_visible(false)?;
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
        Ok(())
    }

    pub fn set_output(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.output_value.set_text(value)
    }
    pub async fn update_children(&mut self) -> Result<bool> {
        update_children!(
            self.root,
            self.title,
            self.description,
            self.output_heading,
            self.output_value,
            self.options_heading,
            self.source_path,
            self.source_toggle
        )
    }
    pub fn render(&mut self) -> Result<()> {
        self.root.render()?;
        self.title.render()?;
        self.description.render()?;
        self.output_heading.render()?;
        self.output_value.render()?;
        self.options_heading.render()?;
        self.source_path.render()?;
        self.source_toggle.render()
    }
}

impl AsContainer for GalleryExample {
    fn as_container(&self) -> BorrowedContainer<'_> {
        self.root.as_container()
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
        border.SetBackground(&resources.Lookup(&fill_key)?.cast::<MUX::Media::Brush>()?)?;
        border.SetBorderBrush(&resources.Lookup(&stroke_key)?.cast::<MUX::Media::Brush>()?)?;
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
    fn wide_and_narrow_card_layouts_are_responsive() {
        let wide = CardLayout::calculate(1064.0, true, true, false, 2000);
        let narrow = CardLayout::calculate(560.0, true, true, false, 2000);
        assert!(wide.wide);
        assert!(!narrow.wide);
        assert!(wide.output.unwrap().origin.x > wide.primary.origin.x);
        assert_eq!(narrow.primary.origin.x, narrow.output.unwrap().origin.x);
        assert!(narrow.options.unwrap().origin.y > narrow.output.unwrap().max_y());
    }
    #[test]
    fn page_caps_width_and_stacks_card_extents() {
        assert_eq!(page_width(1400.0), PAGE_MAX_WIDTH);
        let cards = [
            CardLayout::calculate(900.0, true, true, false, 300),
            CardLayout::calculate(900.0, true, false, false, 300),
        ];
        let (tops, extent) = stack_cards(&cards);
        assert_eq!(tops[0], 0.0);
        assert_eq!(tops[1], cards[0].content_height + CARD_GAP);
        assert_eq!(extent, tops[1] + cards[1].content_height);
    }
    #[test]
    fn expanded_source_pushes_following_card_and_page_extent() {
        let first = CardLayout::calculate(900.0, true, true, false, 6000);
        let expanded = CardLayout::calculate(900.0, true, true, true, 6000);
        let second = CardLayout::calculate(900.0, true, false, false, 200);
        let (before, before_extent) = stack_cards(&[first, second]);
        let (after, after_extent) = stack_cards(&[expanded, second]);
        assert!(after[1] > before[1]);
        assert!(after_extent > before_extent);
    }
}
