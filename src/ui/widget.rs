//! On-screen widgets.

use crate::{
    ScreenCoordinates,
    storage::{QueueContainer, TextContainer},
    ui::transition::Transition,
};

#[derive(Debug)]
pub(crate) struct ScreenElement<S: TextContainer, Q: QueueContainer<Transition<S>>> {
    pub(crate) content: ScreenContent<S>,
    pub(crate) pos: ScreenCoordinates,
    pub(crate) hidden: bool,
    pub(crate) transitions: Q,
    pub(crate) transition_progress: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
/// A reference to a custom character, agnostic of it's actual index in the screen RAM.
pub struct CustomCharacterRef(pub(crate) u32, pub(crate) usize);

#[derive(Debug)]
/// What a widget will display.
pub enum ScreenContent<S: TextContainer> {
    /// ASCII / Extended ASCII string.
    Text(S),
    /// A defined custom character.
    CustomCharacter(CustomCharacterRef),
}

impl<S: TextContainer> ScreenContent<S> {
    /// Shorthand for creating a [`ScreenContent::Text`] from an `&str`.
    pub fn text(c: &str) -> Result<Self, S::Error> {
        Ok(ScreenContent::Text(S::from_str(c)?))
    }
}
