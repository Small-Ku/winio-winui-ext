use crate::{
    chrome::{CardId, GalleryExample, GalleryPage},
    sample::{GallerySampleEvent, GallerySampleInit, GallerySampleMessage},
};
use windows::{
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
    core::Interface,
};
use winio::prelude::*;
use winio_winui_ext::{BitmapImage, Image, ImageEvent, ToggleButtonEvent};
use winio_winui3::Microsoft::UI::Xaml::Media::Stretch;

const EMBEDDED_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x08, 0x06, 0x00, 0x00, 0x00, 0x7f, 0xa8, 0x7d,
    0x63, 0x00, 0x00, 0x00, 0x18, 0x49, 0x44, 0x41, 0x54, 0x78, 0xda, 0x63, 0xf8, 0xcf, 0xc0, 0xf0,
    0x1f, 0x0c, 0x19, 0xfe, 0x03, 0x01, 0x82, 0x06, 0x03, 0x06, 0x20, 0x0f, 0x00, 0x3f, 0xf0, 0x13,
    0xed, 0xd9, 0x3a, 0x52, 0x11, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60,
    0x82,
];

pub struct ImageSample {
    page: GalleryPage,
    card: GalleryExample,
    image: Child<Image>,
    bitmap: BitmapImage,
    _stream: InMemoryRandomAccessStream,
    viewport: Rect,
    source_expanded: bool,
    source_attached: bool,
    generation: u64,
}

impl Component for ImageSample {
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
            "Image",
            "A native Image using an offline embedded PNG and an explicit ImageSource escape hatch.",
            "apps/gallery/src/samples/image.rs · winio_winui_ext::Image",
            include_str!("image.rs"),
            true,
            false,
        )
        .await?;
        card.set_output("loading")?;

        let mut image = Child::<Image>::init(card.as_container()).await?;
        image.set_stretch(Stretch::Uniform)?;

        let stream = InMemoryRandomAccessStream::new()?;
        let writer = DataWriter::CreateDataWriter(&stream)?;
        writer.WriteBytes(EMBEDDED_PNG)?;
        let stored = writer.StoreAsync()?.await?;
        if stored != EMBEDDED_PNG.len() as u32 {
            return Err(windows::core::Error::new(
                windows::core::HRESULT(0x80004005u32 as i32),
                "embedded PNG stream was only partially written",
            ));
        }
        writer.DetachStream()?;
        stream.Seek(0)?;

        let bitmap = BitmapImage::new()?;

        Ok(Self {
            page,
            card,
            image,
            bitmap,
            _stream: stream,
            viewport: Rect::zero(),
            source_expanded: false,
            source_attached: false,
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
            self.image => {
                ImageEvent::Opened => GallerySampleMessage::Emit("opened".to_owned()),
                ImageEvent::Failed { message } =>
                    GallerySampleMessage::Emit(format!("failed:{message}")),
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
                if !self.source_attached {
                    self.image.set_bitmap_image(&self.bitmap)?;
                    let _roundtrip_bitmap = self.image.source()?.cast::<BitmapImage>()?;
                    self.bitmap.SetSource(&self._stream)?;
                    self.source_attached = true;
                }
            }
            GallerySampleMessage::SourceExpanded {
                card: CardId::MAIN,
                expanded,
            } => {
                self.source_expanded = expanded;
                self.reflow()?;
            }
            GallerySampleMessage::Emit(value) if value == "opened" => {
                let value = format!(
                    "opened · {}×{}",
                    self.bitmap.PixelWidth()?,
                    self.bitmap.PixelHeight()?
                );
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "image",
                    value,
                });
            }
            GallerySampleMessage::Emit(value) if !value.is_empty() => {
                self.card.set_output(&value)?;
                sender.output(GallerySampleEvent::Output {
                    generation: self.generation,
                    sample_id: "image",
                    value,
                });
            }
            _ => {}
        }
        Ok(false)
    }

    async fn update_children(&mut self) -> Result<bool> {
        self.page.update_children().await?;
        self.card.update_children().await?;
        self.image.update().await
    }

    fn render_children(&mut self) -> Result<()> {
        self.page.render()?;
        self.card.render()?;
        self.image.render()
    }
}

impl ImageSample {
    fn reflow(&mut self) -> Result<()> {
        let layout = self
            .card
            .calculate_layout(self.viewport.size.width, self.source_expanded);
        self.card.set_layout(0.0, layout)?;
        self.page.set_layout(self.viewport, layout.content_height)?;

        self.image.set_loc(Point::new(
            layout.primary.origin.x + 16.0,
            layout.primary.origin.y + 12.0,
        ))?;
        self.image.set_size(Size::new(
            (layout.primary.size.width - 32.0).max(0.0),
            (layout.primary.size.height - 24.0).max(0.0),
        ))?;
        Ok(())
    }
}
