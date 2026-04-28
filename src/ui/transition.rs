//! Widget transitions.
// holy shit this crate is woke transgnder?????? uninstalling
// (/j /j /j /j /j /s pls dont cancel me pls :pray: :ppray:)

use crate::{ScreenCoordinates, storage::TextContainer, ui::widget::ScreenContent};
#[derive(Debug)]
/// Different transitions a widget can take to change it's state.
/// Transitions that do not have a `duration` field complete in 1 frame.
pub enum Transition<S: TextContainer> {
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
    ChangeTo(ScreenContent<S>),
    /// Hides the widget.
    Hide,
    /// Shows the widget.
    Show,
    /// Destroys the widget.
    Delete,
}
