//! On-screen widgets.

#[cfg(feature = "alloc")]
use alloc::{
    collections::VecDeque,
    string::{String, ToString},
};

#[cfg(not(feature = "alloc"))]
use heapless::{deque::Deque as VecDeque, string::String};

use crate::{ScreenCoordinates, ui::transition::Transition};

#[derive(Debug)]
struct ScreenElement<const MAX_TRANSITIONS: usize, const STR_LEN: usize> {
    content: ScreenContent<STR_LEN>,
    pos: ScreenCoordinates,
    hidden: bool,
    transitions: VecDeque<Transition<STR_LEN>, MAX_TRANSITIONS>,
    transition_progress: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
/// A reference to a custom character, agnostic of it's actual index in the screen RAM.
pub struct CustomCharacterRef(u32, usize);
#[derive(Debug)]
/// What a widget will display.
pub enum ScreenContent<const STR_LEN: usize> {
    /// ASCII / Extended ASCII string.
    Text(String<STR_LEN>),
    /// A defined custom character.
    CustomCharacter(CustomCharacterRef),
}

impl<const STR_LEN: usize> ScreenContent<STR_LEN> {
    /// Shorthand for creating a [`ScreenContent::Text`] from an `&str`.
    /// Returns an `Err` if the string is longer than the specified `STR_LEN`.
    #[cfg(not(feature = "alloc"))]
    fn text(c: &str) -> Result<Self, heapless::CapacityError> {
        Ok(ScreenContent::Text(String::try_from(c)?))
    }
    /// Shorthand for creating a [`ScreenContent::Text`] from an `&str`.
    #[cfg(feature = "alloc")]
    fn text(c: &str) -> Self {
        ScreenContent::Text(c.to_string())
    }
}
