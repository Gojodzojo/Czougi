use super::drawing_utils::tank::{draw_tank, Direction};
use super::Mode;
use crate::game::input::InputState;
use crate::game::modes::drawing_utils::block::{
    draw_block, BRICK_COLOR, CONCRETE_COLOR, GRASS_COLOR, WATER_COLOR,
};
use crossterm::style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, Result};
use std::io::Stdout;
use std::time::Duration;

pub struct Editor {}

impl Mode for Editor {
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

        if resized {
            self.draw_sidebar(stdout, horizontal_margin + 100, vertical_margin)?;
        }

        self.draw_map(stdout, horizontal_margin, vertical_margin)?;

        Ok(())
    }
}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    fn draw_sidebar(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        queue!(stdout, SetBackgroundColor(Color::White))?;

        for row in y..50 + y {
            queue!(
                stdout,
                cursor::MoveTo(x, row),
                Print("                      "),
            )?;
        }

        queue!(stdout, SetForegroundColor(BRICK_COLOR))?;
        draw_block(stdout, x + 2, y + 15)?;
        draw_block(stdout, x + 6, y + 15)?;
        draw_block(stdout, x + 2, y + 17)?;
        draw_block(stdout, x + 6, y + 17)?;

        queue!(stdout, SetForegroundColor(CONCRETE_COLOR))?;
        draw_block(stdout, x + 12, y + 15)?;
        draw_block(stdout, x + 16, y + 15)?;
        draw_block(stdout, x + 12, y + 17)?;
        draw_block(stdout, x + 16, y + 17)?;

        queue!(stdout, SetForegroundColor(WATER_COLOR))?;
        draw_block(stdout, x + 2, y + 20)?;
        draw_block(stdout, x + 6, y + 20)?;
        draw_block(stdout, x + 2, y + 22)?;
        draw_block(stdout, x + 6, y + 22)?;

        queue!(stdout, SetForegroundColor(GRASS_COLOR))?;
        draw_block(stdout, x + 12, y + 20)?;
        draw_block(stdout, x + 16, y + 20)?;
        draw_block(stdout, x + 12, y + 22)?;
        draw_block(stdout, x + 16, y + 22)?;

        draw_tank(stdout, x + 2, y + 25, Color::Yellow, Direction::Up)?;
        draw_tank(stdout, x + 12, y + 25, Color::Blue, Direction::Down)?;
        draw_tank(stdout, x + 2, y + 30, Color::Green, Direction::Left)?;
        draw_tank(stdout, x + 12, y + 30, Color::Red, Direction::Right)?;

        Ok(())
    }

    fn draw_map(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        let horizontal_lines = "   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │    ";
        queue!(stdout, SetBackgroundColor(Color::Black))?;

        for row in (y..48 + y).step_by(2) {
            queue!(
                stdout,
                cursor::MoveTo(x, row),
                Print(horizontal_lines),
                cursor::MoveTo(x, row + 1),
                SetAttribute(Attribute::Underlined),
                Print(horizontal_lines),
                SetAttribute(Attribute::Reset),
            )?;
        }
        queue!(
            stdout,
            cursor::MoveTo(x, y + 48),
            Print(horizontal_lines),
            cursor::MoveTo(x, y + 49),
            Print(horizontal_lines),
        )?;

        Ok(())
    }
}
