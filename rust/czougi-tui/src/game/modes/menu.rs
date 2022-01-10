use super::Mode;
use crate::game::input::InputState;
use crossterm::{cursor, queue, style::Print, terminal, Result};
use std::io::{Stdout, Write};
use std::time::Duration;

pub struct Menu {
    column: u16,
    row: u16,
}

impl Menu {
    pub fn new() -> Self {
        Menu { column: 1, row: 1 }
    }
}

impl Mode for Menu {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        delta_time: Duration,
        horizontal_margin: u16,
        vertical_margin: u16,
        resized: bool,
        input_state: &InputState,
    ) -> Result<()> {
        let InputState {
            mouse_state,
            window_state,
            players_keys_states,
            ..
        } = input_state;

        if players_keys_states[0].up && self.row >= 1 {
            self.row -= 1;
        }
        if players_keys_states[0].down && self.row < window_state.height {
            self.row += 1;
        }
        if players_keys_states[0].left && self.column >= 1 {
            self.column -= 1;
        }
        if players_keys_states[0].right && self.column < window_state.width {
            self.column += 1;
        }

        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            Print(format!("Delta time: {}ms", delta_time.as_millis())),
            cursor::MoveTo(0, 1),
            Print(format!(
                "Width: {}, Height: {}",
                window_state.width, window_state.height
            )),
            cursor::MoveTo(self.column, self.row),
            Print(format!("Cursor x: {}, Cursor y: {}", self.column, self.row)),
            cursor::MoveTo(mouse_state.column, mouse_state.row),
            Print(format!(
                "Mouse x: {}, Mouse y: {}",
                mouse_state.column, mouse_state.row
            )),
        )?;

        stdout.flush()?;

        Ok(())
    }
}
