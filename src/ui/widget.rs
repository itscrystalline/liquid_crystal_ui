//! On-screen widgets.

use crate::{
    ScreenCoordinates,
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
/// A reference to a custom character, agnostic of it's actual index in the screen RAM.
pub struct CustomCharacterRef(pub(crate) u32, pub(crate) usize);

#[derive(Debug)]
/// What a widget will display.
pub enum WidgetContent<S: TextContainer> {
    /// ASCII / Extended ASCII string.
    Text(S),
    /// A defined custom character.
    CustomCharacter(CustomCharacterRef),
}

impl<S: TextContainer> WidgetContent<S> {
    /// Shorthand for creating a [`ScreenContent::Text`] from an `&str`.
    pub fn text(c: &str) -> Result<Self, S::Error> {
        Ok(WidgetContent::Text(S::from_str(c)?))
    }
}
