//! On-screen widgets.

use crate::{
    ScreenCoordinates,
    error::StorageError,
    storage::{Storage, TextContainer},
    ui::transition::Transition,
};

/// An On-screen widget.
#[derive(Debug)]
pub struct Widget<S: Storage> {
    pub(crate) content: WidgetContent<S::Text>,
    pub(crate) pos: ScreenCoordinates,
    pub(crate) hidden: bool,
    pub(crate) transitions: S::Queue<Transition<S::Text>>,
    pub(crate) transition_progress: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
/// A reference to a custom character, agnostic of its actual index in the screen RAM.
pub struct CustomCharacterRef(pub(crate) u32);

#[derive(Debug)]
/// What a widget will display.
pub enum WidgetContent<S: TextContainer> {
    /// ASCII / Extended ASCII string.
    Text(S),
    /// ASCII / Extended ASCII string that scrolls in place.
    ScrollingText {
        /// The String.
        string: S,
        /// The actual length to show on screen.
        len: usize,
        /// How many characters to scroll per tick.
        speed: usize,
        /// How the text scrolls.
        behaviour: ScrollBehaviour,
    },
    /// A defined custom character.
    CustomCharacter(CustomCharacterRef),
}
#[derive(Debug)]
/// Describes how does a [`WidgetContent::ScrollingText`] scroll. The parameters in `Reset` and `Bounce`
/// dictate how long to stop at the end before continuing.
pub enum ScrollBehaviour {
    /// Snaps back to the start after reaching the end.
    Reset(u8),
    /// Transparently puts the front in after reaching the end.
    Loop,
    /// Bounces back the other way after reaching the end.
    Bounce(u8),
}

impl<S: TextContainer> WidgetContent<S> {
    /// Shorthand for creating a [`WidgetContent::Text`] from an `&str`.
    pub fn text(c: &str) -> Result<Self, StorageError> {
        Ok(WidgetContent::Text(S::from_str(c)?))
    }
    /// Shorthand for creating a [`WidgetContent::ScrollingText`].
    pub fn scroll_text(
        c: &str,
        len: usize,
        behaviour: ScrollBehaviour,
    ) -> Result<Self, StorageError> {
        Ok(WidgetContent::ScrollingText {
            string: S::from_str(c)?,
            len,
            behaviour,
        })
    }
}
