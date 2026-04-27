//! Widget transitions.
// holy shit this crate is woke transgnder?????? uninstalling

use crate::{ScreenCoordinates, ui::widget::ScreenContent};

#[derive(Debug)]
/// Different transitions a widget can take to change it's state.
/// Transitions that do not have a `duration` field complete in 1 frame.
pub enum Transition<const STR_LEN: usize> {
    /// Moves the element from it's current position to a new position.
    MoveTo {
        /// New position.
        new: ScreenCoordinates,
        /// How long (in frames) the transition will take.
        duration: u8,
    },
    /// Moves the element from it's one position to another position.
    MoveToExt {
        /// Old position.
        old: ScreenCoordinates,
        /// New position.
        new: ScreenCoordinates,
        /// How long (in frames) the transition will take.
        duration: u8,
    },
    /// Idles for `duration` frames.
    Wait {
        /// How many frames to idle for.
        duration: u8,
    },
    /// Changes to another [`crate::ui::widget::ScreenContent`].
    ChangeTo(ScreenContent<STR_LEN>),
    /// Hides the widget.
    Hide,
    /// Shows the widget.
    Show,
    /// Destroys the widget.
    Delete,
}
