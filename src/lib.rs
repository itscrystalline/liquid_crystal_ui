//! # liquid_crystal_ui
//! A retained-mode-esque UI library for displaying to small alphanumeric LCD displays, like the
//! ones on cash registers or something

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod backend;
pub mod ui;

pub struct ScreenCoordinates {
    row: u8,
    col: u8,
}

impl ScreenCoordinates {
    pub fn at(x: u8, y: u8) -> Self {
        ScreenCoordinates { row: x, col: y }
    }
}
impl From<(u8, u8)> for ScreenCoordinates {
    fn from(value: (u8, u8)) -> Self {
        ScreenCoordinates::at(value.0, value.1)
    }
}
