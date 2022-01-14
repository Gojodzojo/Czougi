use super::drawing_utils::block::{
    draw_brick_block, draw_concrete_block, draw_leaves_block, draw_water_block,
};
use super::drawing_utils::draw_multi_line_text;
use super::drawing_utils::tank::{draw_tank, Direction};
use super::Mode;
use crate::game::input::InputState;
use crate::game::modes::drawing_utils::block::{
    BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, CONCRETE_BACKGROUND_COLOR,
    CONCRETE_FOREGROUND_COLOR, LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR,
    WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR,
};
use crate::game::options::Options;
use crossterm::style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, Result};
use std::io::Stdout;
use std::time::Duration;

const ERASER: [&str; 4] = ["▄▄    ▄▄", " ▀▀▄▄▀▀", " ▄▄▀▀▄▄", "▀▀    ▀▀"];

enum Tool {
    Brick,
    Concrete,
    Water,
    Leaves,
    Tank(u8, Direction), // Player number, direction of tank
    Eraser,
}
pub struct Editor {}

impl Mode for Editor {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        _delta_time: Duration,
        horizontal_margin: u16,
        vertical_margin: u16,
        refresh: bool,
        input_state: &InputState,
        _options: &Options,
    ) -> Result<Option<Box<dyn Mode>>> {
        let InputState { mouse_state, .. } = input_state;

        if refresh {
            self.draw_sidebar(stdout, horizontal_margin + 100, vertical_margin)?;
        }

        self.draw_map(stdout, horizontal_margin, vertical_margin)?;

        Ok(None)
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

        let title = String::from("Place title here");

        queue!(
            stdout,
            cursor::MoveTo(x + (22 - title.len() as u16) / 2, y + 3),
            SetForegroundColor(Color::Black),
            SetAttribute(Attribute::Bold),
            Print(title),
            SetAttribute(Attribute::Reset),
        )?;

        queue!(
            stdout,
            SetForegroundColor(BRICK_FOREGROUND_COLOR),
            SetBackgroundColor(BRICK_BACKGROUND_COLOR),
        )?;
        draw_brick_block(stdout, x + 2, y + 10)?;
        draw_brick_block(stdout, x + 6, y + 10)?;
        draw_brick_block(stdout, x + 2, y + 12)?;
        draw_brick_block(stdout, x + 6, y + 12)?;

        queue!(
            stdout,
            SetForegroundColor(CONCRETE_FOREGROUND_COLOR),
            SetBackgroundColor(CONCRETE_BACKGROUND_COLOR),
        )?;
        draw_concrete_block(stdout, x + 12, y + 10)?;
        draw_concrete_block(stdout, x + 16, y + 10)?;
        draw_concrete_block(stdout, x + 12, y + 12)?;
        draw_concrete_block(stdout, x + 16, y + 12)?;

        queue!(
            stdout,
            SetForegroundColor(WATER_FOREGROUND_COLOR),
            SetBackgroundColor(WATER_BACKGROUND_COLOR),
        )?;
        draw_water_block(stdout, x + 2, y + 15)?;
        draw_water_block(stdout, x + 6, y + 15)?;
        draw_water_block(stdout, x + 2, y + 17)?;
        draw_water_block(stdout, x + 6, y + 17)?;

        queue!(
            stdout,
            SetForegroundColor(LEAVES_FOREGROUND_COLOR),
            SetBackgroundColor(LEAVES_BACKGROUND_COLOR),
        )?;
        draw_leaves_block(stdout, x + 12, y + 15)?;
        draw_leaves_block(stdout, x + 16, y + 15)?;
        draw_leaves_block(stdout, x + 12, y + 17)?;
        draw_leaves_block(stdout, x + 16, y + 17)?;

        queue!(stdout, SetBackgroundColor(Color::White))?;
        draw_tank(stdout, x + 2, y + 20, Color::Yellow, Direction::Up)?;
        draw_tank(stdout, x + 12, y + 20, Color::Blue, Direction::Up)?;
        draw_tank(stdout, x + 2, y + 25, Color::Green, Direction::Up)?;
        draw_tank(stdout, x + 12, y + 25, Color::Red, Direction::Up)?;

        queue!(
            stdout,
            SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }),
            SetAttribute(Attribute::Bold)
        )?;
        draw_multi_line_text(stdout, ERASER.iter(), x + 7, y + 30)?;

        let buttons = [
            (" Play", Color::DarkGreen),
            (" Save", Color::Blue),
            ("Discard", Color::Black),
            ("Delete", Color::Red),
        ];

        for (i, (text, color)) in buttons.iter().enumerate() {
            let i = i as u16;
            let x = x + (i % 2) * 11;
            let y = y + 40 + (i / 2) * 3;
            queue!(
                stdout,
                SetForegroundColor(*color),
                cursor::MoveTo(x, y),
                Print("┌─────────┐"),
                cursor::MoveTo(x, y + 1),
                Print("│         │"),
                cursor::MoveTo(x, y + 2),
                Print("└─────────┘"),
                cursor::MoveTo(x + 2, y + 1),
                Print(text),
            )?;
        }

        Ok(())
    }

    fn draw_map(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        let horizontal_lines = "   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │   │    ";
        queue!(
            stdout,
            SetBackgroundColor(Color::Black),
            SetForegroundColor(Color::White),
        )?;

        for row in (y..48 + y).step_by(2) {
            queue!(
                stdout,
                cursor::MoveTo(x, row),
                Print(horizontal_lines),
                cursor::MoveTo(x, row + 1),
                SetAttribute(Attribute::Reset),
                Print(horizontal_lines),
                SetAttribute(Attribute::OverLined),
            )?;
        }
        queue!(
            stdout,
            cursor::MoveTo(x, y + 48),
            Print(horizontal_lines),
            cursor::MoveTo(x, y + 49),
            SetAttribute(Attribute::Reset),
            Print(horizontal_lines),
        )?;

        Ok(())
    }
}
