use std::sync::OnceLock;

use embassy_time::Timer;
use embassy_executor::Spawner;
use embedded_graphics::{geometry::Size, pixelcolor::BinaryColor};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window};
use nano_core::display::{Display, SharedDisplay};

static DISPLAY_LOCK: OnceLock<SharedDisplay<SimulatorDisplay<BinaryColor>>> = OnceLock::new();

#[embassy_executor::task]
async fn simulator_display_task(display: &'static SharedDisplay<SimulatorDisplay<BinaryColor>>) {
    const FPS: u64 = 60;

    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .build();

    let mut window = Window::new("Nano OS Display Simulator", &output_settings);
    window.set_max_fps(FPS as u32);

    let mut show_display = true;

    while show_display {
        window.update(display.lock().await.get());

        for event in window.events() {
            if event == SimulatorEvent::Quit {
                show_display = false;
                break;
            }
        }

        Timer::after_millis(1000 / FPS).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    let simulator_display = SimulatorDisplay::<BinaryColor>::new(
        Size::new(128, 64)
    );

    let display = DISPLAY_LOCK.get_or_init(
        || SharedDisplay::new(Display::new(simulator_display))
    );

    spawner.spawn(simulator_display_task(display).unwrap());
    // spawner.spawn(draw_on_display(display).unwrap());

    nano_core::run(spawner, display).await;
}