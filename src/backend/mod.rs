//! ## LCD Backends. You must implement `crate::backend::LcdBackend` or
//! `crate::backend::AsyncLcdBackend` on your LCD drivers (or wrap them in structs, then implement)
//! to use them with this library.
//!
//! ### Usage
//! TBA

use crate::ScreenCoordinates;

#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait AsyncLcdBackend<const CHAR_HEIGHT: usize> {
    const CUSTOM_CHARACTER_SLOTS: u8;
    type Error;

    async fn init_driver(&mut self, delay: &mut impl ADelay) -> Result<&mut Self, Self::Error>;
    async fn clear(&mut self, delay: &mut impl ADelay) -> Result<&mut Self, Self::Error>;
    async fn move_cursor(
        &mut self,
        delay: &mut impl ADelay,
        pos: ScreenCoordinates,
    ) -> Result<&mut Self, Self::Error>;
    async fn write_byte(
        &mut self,
        delay: &mut impl ADelay,
        byte: u8,
    ) -> Result<&mut Self, Self::Error>;
    async fn write_custom_character(
        &mut self,
        delay: &mut impl ADelay,
        char_idx: u8,
    ) -> Result<&mut Self, Self::Error>;
    async fn set_custom_character_at(
        &mut self,
        delay: &mut impl ADelay,
        at: u8,
        character: [u8; CHAR_HEIGHT],
    ) -> Result<&mut Self, Self::Error>;
}

pub trait LcdBackend<const CHAR_HEIGHT: usize> {
    const CUSTOM_CHARACTER_SLOTS: u8;
    type Error;

    fn init_driver(&mut self, delay: &mut impl Delay) -> Result<&mut Self, Self::Error>;
    fn clear(&mut self, delay: &mut impl Delay) -> Result<&mut Self, Self::Error>;
    fn move_cursor(
        &mut self,
        delay: &mut impl Delay,
        pos: ScreenCoordinates,
    ) -> Result<&mut Self, Self::Error>;
    fn write_byte(&mut self, delay: &mut impl Delay, byte: u8) -> Result<&mut Self, Self::Error>;
    fn write_custom_character(
        &mut self,
        delay: &mut impl Delay,
        char_idx: u8,
    ) -> Result<&mut Self, Self::Error>;
    fn set_custom_character_at(
        &mut self,
        delay: &mut impl Delay,
        at: u8,
        character: [u8; CHAR_HEIGHT],
    ) -> Result<&mut Self, Self::Error>;
}
