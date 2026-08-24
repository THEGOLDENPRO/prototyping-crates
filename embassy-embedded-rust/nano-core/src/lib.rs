#![no_std]
use embassy_executor::Spawner;
use embedded_graphics::{Drawable, draw_target::DrawTarget, geometry::Point, mono_font::{MonoTextStyle, ascii::FONT_10X20}, pixelcolor::BinaryColor, text::{Text, renderer::TextRenderer}};

use crate::display::SharedDisplay;

pub mod display;

pub async fn run<D>(spawner: Spawner, display: &SharedDisplay<D>)
where
    D: DrawTarget<Color = BinaryColor>,
    D::Error: core::fmt::Debug,
{
    let text_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    display.lock().await.use_raw(async |display| {
        let _ = display.clear(BinaryColor::Off);

        let display_center = display.bounding_box().center();

        let _ = Text::new(
            "NANO OS",
            // TODO: we'll need a way to check text width to 
            // draw it directly in the center of the screen 
            Point::new(
                30,
                (display_center.y as f32 + text_style.line_height() as f32 / 4.0) as i32
            ),
            text_style
        ).draw(display);
    }).await;
}