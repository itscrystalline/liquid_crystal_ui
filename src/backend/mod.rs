//! LCD Backends. You must implement [`LcdBackend`] or
//! [`AsyncLcdBackend`] on your LCD drivers (or wrap them in structs, then implement)
//! to use them with this library.
//!
//! Usage
//! TBA

use crate::ScreenCoordinates;
use crate::storage::TextContainer;

#[cfg(feature = "liquid_crystal_driver")]
pub mod liquid_crystal;

pub use embedded_hal::delay::DelayNs as Delay;
#[cfg(feature = "async")]
pub use embedded_hal_async::delay::DelayNs as ADelay;

#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
/// The Asynchronous Backend trait for `async` drivers.
pub trait AsyncLcdBackend<
    const CHAR_HEIGHT: usize,
    const CUSTOM_CHARACTER_SLOTS: usize,
    T: TextContainer,
>
{
    /// The error type that is emitted by the driver. If the driver doesn't fail, you can use
    /// [`core::convert::Infallible`] or the `!` (never) type.
    type Error;

    /// Initializes the screen after initializing the driver. This function should disable the
    /// cursor and disable autoscroll.
    async fn prepare_screen(&mut self, delay: &mut impl ADelay) -> Result<&mut Self, Self::Error>;
    /// Clears the screen.
    async fn clear(&mut self, delay: &mut impl ADelay) -> Result<&mut Self, Self::Error>;
    /// Move the internal cursor to start writing at this position.
    async fn move_cursor(
        &mut self,
        delay: &mut impl ADelay,
        pos: ScreenCoordinates,
    ) -> Result<&mut Self, Self::Error>;
    /// Prints the byte at the cursor's location.
    async fn write_byte(
        &mut self,
        delay: &mut impl ADelay,
        byte: u8,
    ) -> Result<&mut Self, Self::Error>;
    /// Prints the custom character at the cursor's location, assuming the display supports custom
    /// characters.
    async fn write_custom_character(
        &mut self,
        delay: &mut impl ADelay,
        char_idx: u8,
    ) -> Result<&mut Self, Self::Error>;
    /// Assigns a character slot at index `at` to be the passed in bitmap.
    async fn set_custom_character_at(
        &mut self,
        delay: &mut impl ADelay,
        at: u8,
        character: [u8; CHAR_HEIGHT],
    ) -> Result<&mut Self, Self::Error>;

    /// Writes a string of ASCII characters to the screen.
    async fn write_str(
        &mut self,
        delay: &mut impl ADelay,
        s: &T,
    ) -> Result<&mut Self, Self::Error> {
        for ch in s.chars() {
            self.write_byte(delay, ch).await?;
        }
        Ok(self)
    }
}

/// The Synchronous Backend trait for blocking drivers.
pub trait LcdBackend<
    const CHAR_HEIGHT: usize,
    const CUSTOM_CHARACTER_SLOTS: usize,
    T: TextContainer,
>
{
    /// The error type that is emitted by the driver. If the driver doesn't fail, you can use
    /// [`core::convert::Infallible`] or the `!` (never) type.
    type Error;

    /// Initializes the screen after initializing the driver. This function should disable the
    /// cursor and disable autoscroll.
    fn prepare_screen(&mut self, delay: &mut impl Delay) -> Result<&mut Self, Self::Error>;
    /// Clears the screen.
    fn clear(&mut self, delay: &mut impl Delay) -> Result<&mut Self, Self::Error>;
    /// Move the internal cursor to start writing at this position.
    fn move_cursor(
        &mut self,
        delay: &mut impl Delay,
        pos: ScreenCoordinates,
    ) -> Result<&mut Self, Self::Error>;
    /// Prints the byte at the cursor's location.
    fn write_byte(&mut self, delay: &mut impl Delay, byte: u8) -> Result<&mut Self, Self::Error>;
    /// Prints the custom character at the cursor's location, assuming the display supports custom
    /// characters.
    fn write_custom_character(
        &mut self,
        delay: &mut impl Delay,
        char_idx: u8,
    ) -> Result<&mut Self, Self::Error>;
    /// Assigns a character slot at index `at` to be the passed in bitmap.
    fn set_custom_character_at(
        &mut self,
        delay: &mut impl Delay,
        at: u8,
        character: [u8; CHAR_HEIGHT],
    ) -> Result<&mut Self, Self::Error>;

    /// Writes a string of ASCII characters to the screen.
    fn write_str(&mut self, delay: &mut impl Delay, s: &T) -> Result<&mut Self, Self::Error> {
        for ch in s.chars() {
            self.write_byte(delay, ch)?;
        }
        Ok(self)
    }
}
