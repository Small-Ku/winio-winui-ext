use crate::{catalog::GallerySampleKind, chrome::CardId, samples};
use winio::prelude::*;

#[derive(Debug)]
pub enum GallerySampleMessage {
    SetBounds(Rect),
    Emit(String),
    ExampleChanged { card: CardId, value: String },
    OpenContentDialog,
    OpenTeachingTip,
    SourceExpanded { card: CardId, expanded: bool },
    ToggleThreeState(bool),
    NumberStepOption(bool),
}

pub struct GallerySampleInit<'a> {
    pub parent: BorrowedContainer<'a>,
    pub generation: u64,
}

#[derive(Debug)]
pub enum GallerySampleEvent {
    Output {
        generation: u64,
        sample_id: &'static str,
        value: String,
    },
}

pub type GallerySampleChild = Child<BoxComponent<GallerySampleMessage, GallerySampleEvent, Error>>;

pub async fn create_sample(
    kind: GallerySampleKind,
    parent: BorrowedContainer<'_>,
    generation: u64,
) -> Result<GallerySampleChild> {
    let init = GallerySampleInit { parent, generation };
    Ok(match kind {
        GallerySampleKind::Overview => Child::<samples::overview::OverviewSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::InfoBadge => Child::<samples::info_badge::InfoBadgeSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::RadioButtons => {
            Child::<samples::radio_buttons::RadioButtonsSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::SelectorBar => {
            Child::<samples::selector_bar::SelectorBarSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::ToggleButton => {
            Child::<samples::toggle_button::ToggleButtonSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::ToggleSwitch => {
            Child::<samples::toggle_switch::ToggleSwitchSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::CalendarDatePicker => {
            Child::<samples::calendar_date_picker::CalendarDatePickerSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::ColorPicker => {
            Child::<samples::color_picker::ColorPickerSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::CommandBar => {
            Child::<samples::command_bar::CommandBarSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::ContentDialog => {
            Child::<samples::content_dialog::ContentDialogSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::DatePicker => {
            Child::<samples::date_picker::DatePickerSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::DropDownButton => {
            Child::<samples::drop_down_button::DropDownButtonSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::Expander => Child::<samples::expander::ExpanderSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::FlipView => Child::<samples::flip_view::FlipViewSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::InfoBar => Child::<samples::info_bar::InfoBarSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::MenuBar => Child::<samples::menu_bar::MenuBarSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::NumberBox => Child::<samples::number_box::NumberBoxSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::PasswordBox => {
            Child::<samples::password_box::PasswordBoxSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::PersonPicture => {
            Child::<samples::person_picture::PersonPictureSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::PipsPager => Child::<samples::pips_pager::PipsPagerSample>::init(init)
            .await?
            .into_boxed(),
        GallerySampleKind::ProgressBar => {
            Child::<samples::progress_bar::ProgressBarSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::ProgressRing => {
            Child::<samples::progress_ring::ProgressRingSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::RatingControl => {
            Child::<samples::rating_control::RatingControlSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::SplitButton => {
            Child::<samples::split_button::SplitButtonSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::TimePicker => {
            Child::<samples::time_picker::TimePickerSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::TeachingTip => {
            Child::<samples::teaching_tip::TeachingTipSample>::init(init)
                .await?
                .into_boxed()
        }
        GallerySampleKind::TreeView => Child::<samples::tree_view::TreeViewSample>::init(init)
            .await?
            .into_boxed(),
    })
}
