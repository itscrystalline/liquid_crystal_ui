# liquid_crystal_ui

A rust embedded UI library for small LCD alphanumeric displays, like the Hitachi
HD44780. Basically, a library that allows you to create UI widgets, move them
around, change them, etc. while displaying to a cash register display.

## Usage

First, you will need to implement either the `LcdBackend` trait or the
`AsyncLcdBackend` trait on your LCD display driver. If you use the
`liquid_crystal` crate to drive your LCD, an implementation is already provided.

For an example, see the docs and the `liquid_crystal` impl at
[liquid_crystal.rs](https://git.iw2tryhard.dev/itscrystalline/liquid_crystal_ui/src/branch/main/src/backend/liquid_crystal.rs).

Next, you want to create a `LcdScreen` or `AsyncLcdScreen` depending if you use
sync/async IO. Here you will also specify which storage backend you want to use
(or specify your own by implementing `Storage`) by providing it in the generics.

```rust
use liquid_crystal_ui::storage::{AllocStorage, HeaplessStorage};

let mut alloc_sync_screen = LcdScreen::<_, _, AllocStorage, _, _>::new(
//                                            ^ Use the `alloc` crate for internal storage
    lcd, // this is your backend driver instance
    esp_hal::delay::Delay::new(), // A delay implementing embedded_hal::delay::DelayNs
)?;
let mut heapless_sync_screen = LcdScreen::<_, _, HeaplessStorage<32>, _, _>::new(
//                                                               ^ the static size of `heapless` containers
//                                               ^ Use the `Heapless` crate for internal storage
    lcd, esp_hal::delay::Delay::new()
)?;

let mut alloc_async_screen = AsyncLcdScreen::<_, _, AllocStorage, _, _>::new(lcd, embassy_time::Delay).await?;
let mut heapless_async_screen = AsyncLcdScreen::<_, _, HeaplessStorage<32>, _, _>::new(lcd, embassy_time::Delay).await?;
```

This will be your primary interface to using the crate's APIs.

To create a widget on screen:

```rust
let hello_text = screen.new_elem(WidgetContent::text("Hello!")?, (0, 0),     false)?;
//                               ^ What to display               ^ Position  ^ Hidden by default?
//                                                                 (X, Y) (col, row)
```

If the display supports custom characters, you can also register them:

```rust
const SMILEY: [u8; 8] = liquid_crystal_ui::bitmap!(
    (. . . . .),
    (. # . # .),
    (. # . # .),
    (. . . . .),
    (# . . . #),
    (# . # . #),
    (. # . # .),
    (. . . . .),
);
let smiley_ref = screen.register_custom_char(SMILEY)?;
let smiley = screen.new_elem(WidgetContent::CustomCharacter(smiley_ref), (8, 1), false)?;
```

Then, you can queue up transitions for the widget to perform an action.

```rust
screen.queue_transition(hello_text,        Transition::move_to((12, 0), 20))?;
//                      ^ Widget reference ^ Transition type            ^ duration (in ticks)
screen.queue_transition(smiley, Transition::ChangeTo(WidgetContent::text(":)"))?))?;
```

> [!NOTE]
> Transitions that do not have a `duration` field take 1 tick to change.

The display will not update itself unless you call `screen.draw()`. Each "tick"
is 1 `.draw()` call. It is reccommended to call `.draw()` in a loop that runs a
certain amount of times a second:

```rust
let fps = 30; // some displays work worse (or even blank out) with high FPS, change the value around to see what works for your display
// for example, an old display may blank out at 20 FPS, but a new display can push 50+ FPS.
loop {
    delay_millis(1000 / fps);
    screen.draw()?;
}
```

## Demonstration

See the `examples/` directory for demo code and videos.
