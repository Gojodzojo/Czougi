use super::input::InputState;
use crossterm::Result;
use std::io::Stdout;
use std::time::Duration;

pub mod menu;
pub trait Mode {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        delta_time: Duration,
        input_state: &InputState,
    ) -> Result<()>;
}
