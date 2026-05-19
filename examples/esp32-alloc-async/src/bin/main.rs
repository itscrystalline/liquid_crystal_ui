#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::num::NonZeroUsize;
use core::sync::atomic::{AtomicU32, Ordering};

use alloc::format;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::rng::Rng;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use liquid_crystal::{BusBits, LCD20X4, LiquidCrystal};
use liquid_crystal_ui::ScreenCoordinates;
use liquid_crystal_ui::ui::AsyncLcdScreen;
use liquid_crystal_ui::ui::transition::Transition;
use liquid_crystal_ui::ui::widget::{ScrollBehaviour, ScrollSpeed, WidgetContent};
use log::info;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static UPTIME: AtomicU32 = AtomicU32::new(0);
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
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO22)
    .with_sda(peripherals.GPIO23)
    .into_async();
    let mut interface = liquid_crystal::I2C::new(i2c_bus, 0x27);
    let lcd = LiquidCrystal::new(&mut interface, BusBits::Bus4Bits, LCD20X4).asynch();
    let mut screen = AsyncLcdScreen::<_, _, liquid_crystal_ui::storage::AllocStorage, _, _>::new(
        lcd,
        embassy_time::Delay,
    )
    .await
    .unwrap();

    let smiley_ref = screen.register_custom_char(SMILEY).unwrap();
    let clock_ref = screen.register_custom_char(CLOCK).unwrap();
    let bat_1_ref = screen.register_custom_char(BAT_1).unwrap();
    let bat_2_ref = screen.register_custom_char(BAT_2).unwrap();

    const FPS: u64 = 30;
    fn random_spot() -> liquid_crystal_ui::ScreenCoordinates {
        let rng = Rng::new();
        let rnd = rng.random();
        ScreenCoordinates::at((rnd % 19) as u8, (rnd % 2) as u8 + 1)
    }
    macro_rules! nz {
        ($x: expr) => {
            NonZeroUsize::new($x).unwrap()
        };
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

    let _scroll_demo_1 = screen
        .new_elem(
            WidgetContent::scroll_text(
                "boioingnggg boioingnggg",
                nz!(10),
                ScrollSpeed::TPC(nz!(10)),
                ScrollBehaviour::Bounce(5),
            )
            .unwrap(),
            (0, 0),
            false,
        )
        .unwrap();
    let _scroll_demo_2 = screen
        .new_elem(
            WidgetContent::scroll_text(
                "im lowk the most boring scrolling text of all",
                nz!(10),
                ScrollSpeed::TPC(nz!(20)),
                ScrollBehaviour::Reset(5),
            )
            .unwrap(),
            (9, 3),
            false,
        )
        .unwrap();

    let _bat_text = screen
        .new_elem(
            WidgetContent::scroll_text(
                "Bat (Not real)  ",
                nz!(5),
                ScrollSpeed::TPC(nz!(15)),
                ScrollBehaviour::Loop,
            )
            .unwrap(),
            (13, 0),
            false,
        )
        .unwrap();
    let _bat_1 = screen
        .new_elem(WidgetContent::CustomCharacter(bat_1_ref), (18, 0), false)
        .unwrap();
    let _bat_2 = screen
        .new_elem(WidgetContent::CustomCharacter(bat_2_ref), (19, 0), false)
        .unwrap();

    let mut frame_counter = 0;
    let mut last_uptime = 0u32;
    let mut last_usage = 0usize;

    spawner.spawn(ticker()).unwrap();
    const TRANSITION_TIME: u8 = 4 * FPS as u8;

    loop {
        let uptime = UPTIME.load(Ordering::Relaxed);
        if uptime != last_uptime {
            last_uptime = uptime;
            screen
                .queue_transition(
                    uptime_widget,
                    Transition::ChangeTo(WidgetContent::text(&format!("{uptime}s")).unwrap()),
                )
                .unwrap();
            let stats = esp_alloc::HEAP.stats();
            if last_usage != stats.current_usage {
                log::info!("Uptime: {uptime}s");
                log::info!("{stats}");
                last_usage = stats.current_usage;
            }
        }
        if frame_counter % TRANSITION_TIME == 0 {
            screen
                .queue_transition(smiley, Transition::move_to(random_spot(), TRANSITION_TIME))
                .unwrap();
            screen
                .queue_transition(
                    hello_text,
                    Transition::move_to(random_spot(), TRANSITION_TIME),
                )
                .unwrap();
        }
        Timer::after(Duration::from_millis(1000 / FPS)).await;
        screen.draw().await.unwrap();
        frame_counter = frame_counter.wrapping_add(1);
    }
}

#[embassy_executor::task]
async fn ticker() {
    loop {
        Timer::after_secs(1).await;
        UPTIME.fetch_add(1, Ordering::SeqCst);
    }
}
