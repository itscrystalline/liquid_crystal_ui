#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::rng::Rng;
use esp_hal::time::Rate;
use liquid_crystal::{BusBits, LCD20X4, LiquidCrystal};
use liquid_crystal_ui::ScreenCoordinates;
use liquid_crystal_ui::storage::HeaplessStorage;
use liquid_crystal_ui::ui::LcdScreen;
use liquid_crystal_ui::ui::transition::Transition;
use liquid_crystal_ui::ui::widget::WidgetContent;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

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
const CLOCK: [u8; 8] = liquid_crystal_ui::bitmap!(
    (. . . . .),
    (. . . . .),
    (. # . # .),
    (# . # . #),
    (# . # . #),
    (# . . . #),
    (. # # # .),
    (. . . . .),
);
const BAT_1: [u8; 8] = liquid_crystal_ui::bitmap!(
    (. . . . .),
    (. # # # #),
    (. # . . .),
    (. # . # #),
    (. # . # #),
    (. # . . .),
    (. # # # #),
    (. . . . .),
);
const BAT_2: [u8; 8] = liquid_crystal_ui::bitmap!(
    (. . . . .),
    (# # # # .),
    (. . . # .),
    (# # . # #),
    (# # . # #),
    (. . . # .),
    (# # # # .),
    (. . . . .),
);

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();

    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO22)
    .with_sda(peripherals.GPIO23);
    let mut interface = liquid_crystal::I2C::new(i2c_bus, 0x27);
    let lcd = LiquidCrystal::new(&mut interface, BusBits::Bus4Bits, LCD20X4);

    let mut screen: LcdScreen<_, _, HeaplessStorage<32>, _, _> =
        LcdScreen::new(lcd, delay).unwrap();

    let smiley_ref = screen.register_custom_char(SMILEY).unwrap();
    let clock_ref = screen.register_custom_char(CLOCK).unwrap();
    let bat_1_ref = screen.register_custom_char(BAT_1).unwrap();
    let bat_2_ref = screen.register_custom_char(BAT_2).unwrap();

    const FPS: u64 = 30;
    fn random_spot() -> ScreenCoordinates {
        let rng = Rng::new();
        let rnd = rng.random();
        ScreenCoordinates::at((rnd % 19) as u8, (rnd % 3) as u8)
    }

    let hello_text = screen
        .new_elem(WidgetContent::text("Hello!").unwrap(), (0, 0), false)
        .unwrap();
    let smiley = screen
        .new_elem(WidgetContent::CustomCharacter(smiley_ref), (8, 2), false)
        .unwrap();

    let _uptime_text = screen
        .new_elem(WidgetContent::CustomCharacter(clock_ref), (0, 3), false)
        .unwrap();
    let uptime_widget = screen
        .new_elem(WidgetContent::text("0s").unwrap(), (1, 3), false)
        .unwrap();

    let _bat_text = screen
        .new_elem(WidgetContent::text("Bat").unwrap(), (15, 0), false)
        .unwrap();
    let _bat_1 = screen
        .new_elem(WidgetContent::CustomCharacter(bat_1_ref), (18, 0), false)
        .unwrap();
    let _bat_2 = screen
        .new_elem(WidgetContent::CustomCharacter(bat_2_ref), (19, 0), false)
        .unwrap();

    let mut frame_counter = 0u64;
    const TRANSITION_TIME: u64 = 4 * FPS;

    loop {
        if frame_counter.is_multiple_of(TRANSITION_TIME) {
            screen
                .queue_transition(
                    smiley,
                    Transition::move_to(random_spot(), TRANSITION_TIME as u8),
                )
                .unwrap();
            screen
                .queue_transition(
                    hello_text,
                    Transition::move_to(random_spot(), TRANSITION_TIME as u8),
                )
                .unwrap();
        }
        if frame_counter.is_multiple_of(FPS) {
            let mut buf = itoa::Buffer::new();
            let uptime_str = buf.format(frame_counter / FPS);
            screen
                .queue_transition(
                    uptime_widget,
                    Transition::ChangeTo(WidgetContent::text(uptime_str).unwrap()),
                )
                .unwrap();
        }
        delay.delay_millis(1000 / FPS as u32);
        screen.draw().unwrap();
        frame_counter = frame_counter.wrapping_add(1);
    }
}
