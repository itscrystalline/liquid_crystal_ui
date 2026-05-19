//! On-screen widgets.

use core::{num::NonZeroUsize, ops::Range};

use crate::{
    ScreenCoordinates,
    error::StorageError,
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
/// A reference to a custom character, agnostic of its actual index in the screen RAM.
pub struct CustomCharacterRef(pub(crate) u32);

#[derive(Debug)]
/// What a widget will display.
pub enum WidgetContent<S: TextContainer> {
    /// ASCII / Extended ASCII string.
    Text(S),
    /// ASCII / Extended ASCII string that scrolls in place.
    ScrollingText {
        /// The String.
        string: S,
        /// The actual length to show on screen.
        len: NonZeroUsize,
        /// How fast the text should scroll.
        speed: ScrollSpeed,

        #[allow(private_interfaces)]
        #[allow(missing_docs)]
        state: ScrollingTextState,
    },
    /// A defined custom character.
    CustomCharacter(CustomCharacterRef),
}
#[derive(Debug)]
/// The scroll speed of a [`WidgetContent::ScrollingText`].
pub enum ScrollSpeed {
    /// Characters per tick.
    CPT(NonZeroUsize),
    /// Ticks per character.
    TPC(NonZeroUsize),
}
#[derive(Debug)]
pub(crate) enum ScrollingTextState {
    Reset {
        range: Range<usize>,
        cooldown: u8,
        cd: u8,
    },
    Loop {
        main: Range<usize>,
        wraparound: usize,
    },
    Bounce {
        range: Range<usize>,
        cooldown: u8,
        cd: u8,
        forwards: bool,
    },
}
impl ScrollingTextState {
    pub(crate) fn next(&mut self, text_len: usize, spd: NonZeroUsize) {
        let spd = usize::from(spd);
        match self {
            ScrollingTextState::Reset {
                range,
                cooldown,
                cd,
            } => {
                let disp_len = range.end - range.start;
                if *cd == 0 {
                    let next_range = Range {
                        start: range.start + spd,
                        end: range.end + spd,
                    };

                    if next_range.end <= text_len {
                        *range = next_range;
                    } else {
                        *range = Range {
                            start: text_len - disp_len,
                            end: text_len,
                        };
                        *cd = *cooldown;
                    }
                } else {
                    *cd -= 1;
                }
            }
            ScrollingTextState::Loop { main, wraparound } => {
                //   0   1   2   3   4   5   6   7   8   9   10  11  12    len: 13
                // +---+---+---+---+---+---+---+---+---+---+---+---+---+
                // | H | e | l | l | o | , |   | W | o | r | l | d | ! |
                // +---+---+---+---+---+---+---+---+---+---+---+---+---+
                // \-------------------/  ->
                //             \-------------------/  ->
                //                             ->  \-------------------/   main: 8..13,  wraparound: (0..)0 -> 8..13 -> 9%13..14%13 -> 9..1
                // ----/                                \---------------   main: 9..13,  wraparound: (0..)1 -> 9..1  -> 10%13..2%13 -> 10..2
                // --------/                                \-----------   main: 10..13, wraparound: (0..)2 -> 10..2 -> 11%13..3%13 -> 11..3 -> 12..4
                // ----------------/                               \----   main: 12..13, wraparound: (0..)4 -> 12..4 -> 13%13..5%13 -> 0..5
                // \-------------------/                                   main: 0..5, wraparound: (0..)0   -> 0..5
                let reassembled_range = if *wraparound != 0 && main.end == text_len {
                    main.start..*wraparound
                } else {
                    main.start..main.end
                };

                let new_range = Range {
                    start: (reassembled_range.start + spd) % text_len,
                    end: (reassembled_range.end + spd) % text_len,
                };

                if new_range.start > new_range.end {
                    *wraparound = new_range.end;
                    *main = new_range.start..text_len;
                } else {
                    *main = new_range;
                }
            }
            ScrollingTextState::Bounce {
                range,
                cooldown,
                cd,
                forwards,
            } => {
                let disp_len = range.end - range.start;
                if *cd == 0 {
                    if *forwards {
                        let next_range = Range {
                            start: range.start + spd,
                            end: range.end + spd,
                        };

                        if next_range.end <= text_len {
                            *range = next_range;
                        } else {
                            *range = Range {
                                start: text_len - disp_len,
                                end: text_len,
                            };
                            *cd = *cooldown;
                        }
                    } else {
                        let met_zero = range.start.checked_sub(spd);
                        if let Some(start) = met_zero {
                            *range = Range {
                                start,
                                end: range.end - spd,
                            };
                        } else {
                            *range = Range {
                                start: 0,
                                end: disp_len,
                            };
                            *cd = *cooldown;
                        }
                    }
                } else {
                    *cd -= 1;
                    if *cd == 0 {
                        *forwards = !*forwards;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
/// Describes how does a [`WidgetContent::ScrollingText`] scroll. The parameters in `Reset` and `Bounce`
/// dictate how long to stop at the end before continuing.
pub enum ScrollBehaviour {
    /// Snaps back to the start after reaching the end.
    Reset(u8),
    /// Transparently puts the front in after reaching the end.
    Loop,
    /// Bounces back the other way after reaching the end.
    Bounce(u8),
}

impl<S: TextContainer> WidgetContent<S> {
    /// Shorthand for creating a [`WidgetContent::Text`] from an `&str`.
    pub fn text(c: &str) -> Result<Self, StorageError> {
        Ok(WidgetContent::Text(S::from_str(c)?))
    }

    /// Shorthand for creating a [`WidgetContent::ScrollingText`].
    ///
    /// Note that passing a string thats shorter or equal than `len`, or setting speed == 0 will create a
    /// regular [`WidgetContent::Text`].
    pub fn scroll_text(
        c: &str,
        len: NonZeroUsize,
        speed: ScrollSpeed,
        behaviour: ScrollBehaviour,
    ) -> Result<Self, StorageError> {
        if c.len() <= len.into() {
            Self::text(c)
        } else {
            Ok(WidgetContent::ScrollingText {
                string: S::from_str(c)?,
                len,
                speed,
                state: match behaviour {
                    ScrollBehaviour::Reset(c) => ScrollingTextState::Reset {
                        range: 0..len.into(),
                        cooldown: c,
                        cd: 0,
                    },
                    ScrollBehaviour::Loop => ScrollingTextState::Loop {
                        main: 0..len.into(),
                        wraparound: 0,
                    },
                    ScrollBehaviour::Bounce(c) => ScrollingTextState::Bounce {
                        range: 0..len.into(),
                        cooldown: c,
                        cd: 0,
                        forwards: true,
                    },
                },
            })
        }
    }
}
