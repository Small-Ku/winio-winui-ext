use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use winio::prelude::*;
use winio_winui_ext::{ColorPicker, ColorPickerEvent, ColorValue, ToggleButtonEvent};

pub struct ColorPickerSample {
    page: GalleryPage,
    card: GalleryExample,
    picker: Child<ColorPicker>,
    viewport: Rect,
    source_expanded: bool,
    skip_initial_color: bool,
    generation: u64,
}

impl Component for ColorPickerSample {
    type Error = Error;
    type Event = GallerySampleEvent;
    type Init<'a> = GallerySampleInit<'a>;
    type Message = GallerySampleMessage;

    async fn init(init: Self::Init<'_>, _: &ComponentSender<Self>) -> Result<Self> {
        let GallerySampleInit { parent, generation } = init;
        let page = GalleryPage::new(parent).await?;
        let mut card = GalleryExample::new(
            page.as_container(),
            CardId::MAIN,
            "ColorPicker",
            "A compact native ColorPicker exposing color input and typed RGBA changes.",
            "apps/gallery/src/samples/color_picker.rs · winio_winui_ext::ColorPicker",
            include_str!("color_picker.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("#FF3366CC")?;

        let mut picker = Child::<ColorPicker>::init(card.as_container()).await?;
        picker.set_is_alpha_enabled(false)?;
        picker.set_is_color_spectrum_visible(false)?;
        picker.set_is_color_slider_visible(false)?;
        picker.set_is_color_channel_text_input_visible(false)?;
        picker.set_is_hex_input_visible(true)?;
        picker.set_color(ColorValue::from_argb(255, 0x33, 0x66, 0xCC))?;

        Ok(Self {
            page,
            card,
            picker,
            viewport: Rect::zero(),
            source_expanded: false,
            skip_initial_color: true,
            generation,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        let card_id = self.card.id();
        start! {
            sender, default: GallerySampleMessage::Emit(String::new()),
            self.card.source_toggle => {
                ToggleButtonEvent::StateChanged(state) =>
                    GallerySampleMessage::SourceExpanded {
                        card: card_id,
                        expanded: state == Some(true),
                    },
            },
            self.picker => {
                ColorPickerEvent::ColorChanged { new, .. } =>
                    GallerySampleMessage::Emit(format_color(new)),
            },
        }
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        match message {
            GallerySampleMessage::SetBounds(viewport) => {
                self.viewport = viewport;
                self.reflow()?;
            }
            GallerySampleMessage::SourceExpanded {
                card: CardId::MAIN,
                expanded,
            } => {
                self.source_expanded = expanded;
                self.reflow()?;
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                let initial = self.skip_initial_color && value == "#FF3366CC";
                self.skip_initial_color = false;
                if !initial {
                    self.card.set_output(&value)?;
                    sender.output(GallerySampleEvent::Output {
                        generation: self.generation,
                        sample_id: "color-picker",
                        value,
                    });
                }
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.picker.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.picker.render()
    }
}

impl ColorPickerSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.picker.set_loc(Point::new(
            layout.primary.origin.x + 16.0,
            layout.primary.origin.y + 8.0,
        ))?;
        self.picker.set_size(Size::new(
            (layout.primary.size.width - 32.0).max(0.0),
            (layout.primary.size.height - 16.0).max(0.0),
        ))?;
        Ok(())
    }
}

fn format_color(color: ColorValue) -> String {
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        color.a, color.r, color.g, color.b
    )
}
