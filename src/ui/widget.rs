//! On-screen widgets.

use alloc::{
    collections::VecDeque,
    string::{String, ToString},
};

use crate::{ScreenCoordinates, ui::transition::Transition};

#[derive(Debug)]
pub(crate) struct ScreenElement {
    pub(crate) content: ScreenContent,
    pub(crate) pos: ScreenCoordinates,
    pub(crate) hidden: bool,
    pub(crate) transitions: VecDeque<Transition>,
    pub(crate) transition_progress: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
/// A reference to a custom character, agnostic of it's actual index in the screen RAM.
pub struct CustomCharacterRef(pub(crate) u32, pub(crate) usize);

#[derive(Debug)]
/// What a widget will display.
pub enum ScreenContent {
    /// ASCII / Extended ASCII string.
    Text(String),
    /// A defined custom character.
    CustomCharacter(CustomCharacterRef),
}

impl ScreenContent {
    /// Shorthand for creating a [`ScreenContent::Text`] from an `&str`.
    #[cfg(feature = "alloc")]
    pub fn text(c: &str) -> Self {
        ScreenContent::Text(c.to_string())
    }
}
