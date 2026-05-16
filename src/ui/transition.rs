//! Widget transitions.
// holy shit this crate is woke transgnder?????? uninstalling
// (/j /j /j /j /j /s pls dont cancel me pls :pray: :ppray:)

use crate::{ScreenCoordinates, storage::TextContainer, ui::widget::WidgetContent};
#[derive(Debug)]
/// Different transitions a widget can take to change it's state.
/// Transitions that do not have a `duration` field complete in 1 frame.
pub enum Transition<T: TextContainer> {
    /// Moves the element from it's current position to a new position.
    MoveTo {
        /// New position.
        new: ScreenCoordinates,
        /// How long (in frames) the transition will take.
        duration: u8,
    },
    /// Moves the element from one position to another position.
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
    /// Changes to another [`WidgetContent`](`crate::ui::widget::WidgetContent`).
    ChangeTo(WidgetContent<T>),
    /// Hides the widget.
    Hide,
    /// Shows the widget.
    Show,
    /// Destroys the widget.
    Delete,
}

impl<T: TextContainer> Transition<T> {
    /// Helper function to create [`Transition::Wait`].
    pub fn wait(frames: u8) -> Self {
        Self::Wait { duration: frames }
    }
    /// Helper function to create [`Transition::MoveTo`].
    pub fn move_to(to: impl Into<ScreenCoordinates>, frames: u8) -> Self {
        Self::MoveTo {
            new: to.into(),
            duration: frames,
        }
    }
    /// Helper function to create [`Transition::MoveToExt`].
    pub fn move_from_to(
        from: impl Into<ScreenCoordinates>,
        to: impl Into<ScreenCoordinates>,
        frames: u8,
    ) -> Self {
        Self::MoveToExt {
            old: from.into(),
            new: to.into(),
            duration: frames,
        }
    }
}
