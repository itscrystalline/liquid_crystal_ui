use crate::ScreenCoordinates;

#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait AsyncLcdBackend<const CHAR_HEIGHT: usize> {
    const CUSTOM_CHARACTER_SLOTS: u8;
    type Error;

    async fn init_driver(&mut self) -> Result<&mut Self, Self::Error>;
    async fn clear(&mut self) -> Result<&mut Self, Self::Error>;
    async fn move_cursor(&mut self, pos: ScreenCoordinates) -> Result<&mut Self, Self::Error>;
    async fn write_byte(&mut self, byte: u8) -> Result<&mut Self, Self::Error>;
    async fn set_custom_character(
        &mut self,
        character: [u8; CHAR_HEIGHT],
    ) -> Result<u8, [u8; CHAR_HEIGHT]>;
}

pub trait LcdBackend<const CHAR_HEIGHT: usize> {
    const CUSTOM_CHARACTER_SLOTS: u8;
    type Error;

    fn init_driver(&mut self) -> Result<&mut Self, Self::Error>;
    fn clear(&mut self) -> Result<&mut Self, Self::Error>;
    fn move_cursor(&mut self, pos: ScreenCoordinates) -> Result<&mut Self, Self::Error>;
    fn write_byte(&mut self, byte: u8) -> Result<&mut Self, Self::Error>;
    fn set_custom_character(
        &mut self,
        character: [u8; CHAR_HEIGHT],
    ) -> Result<u8, [u8; CHAR_HEIGHT]>;
}
