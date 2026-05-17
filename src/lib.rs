//! # liquid_crystal_ui
//! A retained mode UI library for displaying to small alphanumeric LCD displays, like the
//! ones on cash registers or something

#![no_std]
#![warn(missing_docs)]
#![feature(doc_cfg)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod backend;
pub mod ui;

pub mod error;
pub mod storage;

/// Screen Coordinates on the display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// The X coordinate (column).
    pub const fn x(&self) -> u8 {
        self.col
    }
    /// The Y coordinate (row).
    pub const fn y(&self) -> u8 {
        self.row
    }
    /// The column (X coordinate).
    pub const fn col(&self) -> u8 {
        self.col
    }
    /// The row (Y coordinate).
    pub const fn row(&self) -> u8 {
        self.row
    }
}
impl From<(u8, u8)> for ScreenCoordinates {
    fn from(value: (u8, u8)) -> Self {
        ScreenCoordinates::at(value.0, value.1)
    }
}
