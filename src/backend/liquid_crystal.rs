//! Provides a sync and async implementation of [`crate::backend::LcdBackend`]
//! and [`crate::backend::AsyncLcdBackend`] for the [`liquid_crystal::LiquidCrystal`]
//! screen driver.
//!
//! You can also use this module as a reference for implementing the backend traits on your own drivers.

use core::convert::Infallible;

pub use embedded_hal::delay::DelayNs as Delay;
#[cfg(feature = "async")]
pub use embedded_hal_async::delay::DelayNs as ADelay;

use liquid_crystal::{Command, Commands::Clear, LiquidCrystal, SendType::CustomChar};

#[cfg(feature = "async")]
use crate::backend::AsyncLcdBackend;
use crate::{backend::LcdBackend, storage::TextContainer};

impl<T: liquid_crystal::Interface, S: TextContainer, const COLS: u8, const LINES: usize>
    LcdBackend<8, 8, S> for LiquidCrystal<'_, T, COLS, LINES, liquid_crystal::Blocking>
{
    /// The driver doesn't return errors, so no errors can happen in the driver
    type Error = Infallible;

    fn prepare_screen(&mut self, delay: &mut impl Delay) -> Result<&mut Self, Self::Error> {
        self.begin(delay);
        self.disable_autoscroll()
            .disable_cursor()
            .update_config(delay);
        Ok(self)
    }

    fn clear(&mut self, delay: &mut impl Delay) -> Result<&mut Self, Self::Error> {
        self.write(delay, Command(Clear));
        Ok(self)
    }

    fn move_cursor(
        &mut self,
        delay: &mut impl Delay,
        pos: crate::ScreenCoordinates,
    ) -> Result<&mut Self, Self::Error> {
        let y = (pos.row as usize).clamp(0, LINES);
        let x = pos.col.clamp(0, COLS);
        #[cfg(feature = "log")]
        {
            if y != pos.row as usize {
                log::warn!("Y coordinate {y} is larger than the screen size of {LINES}!");
            }
            if x != pos.col {
                log::warn!("X coordinate {x} is larger than the screen size of {COLS}!");
            }
        }
        self.set_cursor(delay, y, x);
        Ok(self)
    }

    fn write_byte(&mut self, delay: &mut impl Delay, byte: u8) -> Result<&mut Self, Self::Error> {
        self.send(delay, byte, 0x01);
        Ok(self)
    }

    fn set_custom_character_at(
        &mut self,
        delay: &mut impl Delay,
        at: u8,
        character: [u8; 8],
    ) -> Result<&mut Self, Self::Error> {
        #[cfg(feature = "log")]
        if at >= 8 {
            log::error!(
                "character index {at} is more than the supported character count of this display!"
            );
        }

        if at < 8 {
            self.custom_char(delay, &character, at);
        }
        Ok(self)
    }

    fn write_custom_character(
        &mut self,
        delay: &mut impl Delay,
        char_idx: u8,
    ) -> Result<&mut Self, Self::Error> {
        self.write(delay, CustomChar(char_idx));
        Ok(self)
    }
}

#[cfg(feature = "async")]
impl<T: liquid_crystal::Interface, S: TextContainer, const COLS: u8, const LINES: usize>
    AsyncLcdBackend<8, 8, S> for LiquidCrystal<'_, T, COLS, LINES, liquid_crystal::Async>
{
    /// The driver doesn't return errors, so no errors can happen in the driver
    type Error = Infallible;

    async fn prepare_screen(&mut self, delay: &mut impl ADelay) -> Result<&mut Self, Self::Error> {
        self.begin(delay).await;
        self.disable_autoscroll()
            .disable_cursor()
            .update_config(delay)
            .await;
        Ok(self)
    }

    async fn clear(&mut self, delay: &mut impl ADelay) -> Result<&mut Self, Self::Error> {
        self.write(delay, Command(Clear)).await;
        Ok(self)
    }

    async fn move_cursor(
        &mut self,
        delay: &mut impl ADelay,
        pos: crate::ScreenCoordinates,
    ) -> Result<&mut Self, Self::Error> {
        let y = (pos.row as usize).clamp(0, LINES);
        let x = pos.col.clamp(0, COLS);
        #[cfg(feature = "log")]
        {
            if y != pos.row as usize {
                log::warn!("Y coordinate {y} is larger than the screen size of {LINES}!");
            }
            if x != pos.col {
                log::warn!("X coordinate {x} is larger than the screen size of {COLS}!");
            }
        }
        self.set_cursor(delay, y, x).await;
        Ok(self)
    }

    async fn write_byte(
        &mut self,
        delay: &mut impl ADelay,
        byte: u8,
    ) -> Result<&mut Self, Self::Error> {
        self.send(delay, byte, 0x01).await;
        Ok(self)
    }

    async fn set_custom_character_at(
        &mut self,
        delay: &mut impl ADelay,
        at: u8,
        character: [u8; 8],
    ) -> Result<&mut Self, Self::Error> {
        #[cfg(feature = "log")]
        if at >= 8 {
            log::error!(
                "character index {at} is more than the supported character count of this display!"
            );
        }

        if at < 8 {
            self.custom_char(delay, &character, at).await;
        }
        Ok(self)
    }

    async fn write_custom_character(
        &mut self,
        delay: &mut impl ADelay,
        char_idx: u8,
    ) -> Result<&mut Self, Self::Error> {
        self.write(delay, CustomChar(char_idx)).await;
        Ok(self)
    }
}
