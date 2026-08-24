use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex}, mutex::Mutex};
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

pub type SharedDisplay<D> = Mutex<CriticalSectionRawMutex, Display<D>>;

// TODO: will expand more later
pub struct Display<D>
where
    D: DrawTarget<Color = BinaryColor>,
    D::Error: core::fmt::Debug,
{
    display: D
}

impl <D>Display<D>
where
    D: DrawTarget<Color = BinaryColor>,
    D::Error: core::fmt::Debug,
{
    pub fn new(display: D) -> Self {
        Self {
            display
        }
    }

    pub fn get(&self) -> &D {
        &self.display
    }

    pub async fn use_raw<F>(&mut self, display_func: F) 
        where F: AsyncFnOnce(&mut D)
    {
        display_func(&mut self.display).await;
    }
}