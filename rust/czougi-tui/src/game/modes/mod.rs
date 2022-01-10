use super::input::InputState;
use crossterm::Result;
use std::io::Stdout;
use std::time::Duration;

mod drawing_utils;
pub mod editor;
pub mod menu;
pub trait Mode {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        delta_time: Duration,
        horizontal_margin: u16,
        vertical_margin: u16,
        resized: bool,
        input_state: &InputState,
    ) -> Result<()>;
}
