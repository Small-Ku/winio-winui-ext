use windows::core::{HSTRING, Interface};
use winio::elm::{Component, ComponentSender};
use winio::handle::{AsWidget, BorrowedContainer, BorrowedWidget};
use winio::primitive::{Enable, Failable, Layoutable, Point, Size, Visible};
use winio::{Error, Result};
use winio_winui_bindings::Microsoft::UI::Xaml::Controls::PersonPicture as NativePersonPicture;

use crate::native::NativeElement;

/// A native WinUI PersonPicture hosted as a first-class Winio component.
#[derive(Debug)]
pub struct PersonPicture {
    native: NativePersonPicture,
    element: NativeElement,
}

impl Failable for PersonPicture {
    type Error = Error;
}

impl PersonPicture {
    pub fn display_name(&self) -> Result<String> {
        Ok(self.native.DisplayName()?.to_string_lossy())
    }

    pub fn set_display_name(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetDisplayName(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn initials(&self) -> Result<String> {
        Ok(self.native.Initials()?.to_string_lossy())
    }

    pub fn set_initials(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetInitials(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn badge_number(&self) -> Result<i32> {
        self.native.BadgeNumber()
    }

    pub fn set_badge_number(&mut self, value: i32) -> Result<()> {
        self.native.SetBadgeNumber(value)?;
        Ok(())
    }

    pub fn badge_text(&self) -> Result<String> {
        Ok(self.native.BadgeText()?.to_string_lossy())
    }

    pub fn set_badge_text(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetBadgeText(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn badge_glyph(&self) -> Result<String> {
        Ok(self.native.BadgeGlyph()?.to_string_lossy())
    }

    pub fn set_badge_glyph(&mut self, value: impl AsRef<str>) -> Result<()> {
        self.native.SetBadgeGlyph(&HSTRING::from(value.as_ref()))?;
        Ok(())
    }

    pub fn is_group(&self) -> Result<bool> {
        self.native.IsGroup()
    }

    pub fn set_is_group(&mut self, value: bool) -> Result<()> {
        self.native.SetIsGroup(value)?;
        Ok(())
    }

    pub fn prefer_small_image(&self) -> Result<bool> {
        self.native.PreferSmallImage()
    }

    pub fn set_prefer_small_image(&mut self, value: bool) -> Result<()> {
        self.native.SetPreferSmallImage(value)?;
        Ok(())
    }

    pub fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    pub fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

impl Visible for PersonPicture {
    fn is_visible(&self) -> Result<bool> {
        self.element.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.element.set_visible(visible)
    }
}

impl Enable for PersonPicture {
    fn is_enabled(&self) -> Result<bool> {
        self.element.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.element.set_enabled(enabled)
    }
}

impl Layoutable for PersonPicture {
    fn loc(&self) -> Result<Point> {
        self.element.loc()
    }

    fn set_loc(&mut self, point: Point) -> Result<()> {
        self.element.set_loc(point)
    }

    fn size(&self) -> Result<Size> {
        self.element.size()
    }

    fn set_size(&mut self, size: Size) -> Result<()> {
        self.element.set_size(size)
    }

    fn preferred_size(&self) -> Result<Size> {
        self.element.preferred_size()
    }

    fn min_size(&self) -> Result<Size> {
        self.element.min_size()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PersonPictureMessage {}

impl Component for PersonPicture {
    type Error = Error;
    type Event = ();
    type Init<'a> = BorrowedContainer<'a>;
    type Message = PersonPictureMessage;

    async fn init(parent: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        let native = NativePersonPicture::new()?;
        let framework_element =
            native.cast::<winio_winui3::Microsoft::UI::Xaml::FrameworkElement>()?;
        let element = NativeElement::new(parent, framework_element)?;
        Ok(Self { native, element })
    }
}

impl AsWidget for PersonPicture {
    fn as_widget(&self) -> BorrowedWidget<'_> {
        self.element.as_widget()
    }
}
