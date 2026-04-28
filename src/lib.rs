//! # liquid_crystal_ui
//! A retained-mode-esque UI library for displaying to small alphanumeric LCD displays, like the
//! ones on cash registers or something

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod backend;
pub mod ui;

/// Blocking (Sync) mode.
pub struct Blocking;
/// Asynchronous mode.
pub struct Async;

/// Screen Coordinates on the display.
#[derive(Clone, Copy, Debug)]
pub struct ScreenCoordinates {
    /// The Y coordinate.
    row: u8,
    /// The X coordinate.
    col: u8,
}

impl ScreenCoordinates {
    /// Helper function for creating a `ScreenCoordinates` to avoid confusion between `row` and `col` and `y` and `x`.
    /// (I get confused a lot, even while writing the above line)
    pub const fn at(x: u8, y: u8) -> Self {
        ScreenCoordinates { row: y, col: x }
    }
}
impl From<(u8, u8)> for ScreenCoordinates {
    fn from(value: (u8, u8)) -> Self {
        ScreenCoordinates::at(value.0, value.1)
    }
}
