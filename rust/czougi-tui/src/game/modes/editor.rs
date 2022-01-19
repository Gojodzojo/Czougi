use super::drawing_utils::block::{
    draw_brick_block, draw_concrete_block, draw_leaves_block, draw_water_block,
};
use super::drawing_utils::draw_multi_line_text;
use super::drawing_utils::tank::{draw_tank, Direction};
use super::Mode;
use crate::game::input::{InputState, MouseState, ScrollState};
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

impl Tool {
    fn change_tank_direction(&mut self, scroll: &ScrollState) {
        if let Tool::Tank(_, direction) = self {
            match scroll {
                ScrollState::Up => {
                    *direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Up,
                    };
                }
                ScrollState::Down => {
                    *direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                }
                _ => {}
            }
        }
    }
}
pub struct Editor {
    tool: Tool,
}

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
        self.tool.change_tank_direction(&mouse_state.scroll);
        self.draw_tool(stdout, mouse_state, horizontal_margin, vertical_margin)?;

        if mouse_state.is_clicked(horizontal_margin + 102, vertical_margin + 10, 8, 4) {
            self.tool = Tool::Brick;
        } else if mouse_state.is_clicked(horizontal_margin + 112, vertical_margin + 10, 8, 4) {
            self.tool = Tool::Concrete;
        } else if mouse_state.is_clicked(horizontal_margin + 102, vertical_margin + 15, 8, 4) {
            self.tool = Tool::Water;
        } else if mouse_state.is_clicked(horizontal_margin + 112, vertical_margin + 15, 8, 4) {
            self.tool = Tool::Leaves;
        } else if mouse_state.is_clicked(horizontal_margin + 102, vertical_margin + 20, 8, 4) {
            self.tool = Tool::Tank(1, Direction::Up);
        } else if mouse_state.is_clicked(horizontal_margin + 112, vertical_margin + 20, 8, 4) {
            self.tool = Tool::Tank(2, Direction::Up);
        } else if mouse_state.is_clicked(horizontal_margin + 102, vertical_margin + 25, 8, 4) {
            self.tool = Tool::Tank(3, Direction::Up);
        } else if mouse_state.is_clicked(horizontal_margin + 112, vertical_margin + 25, 8, 4) {
            self.tool = Tool::Tank(4, Direction::Up);
        } else if mouse_state.is_clicked(horizontal_margin + 107, vertical_margin + 30, 8, 4) {
            self.tool = Tool::Eraser;
        }

        Ok(None)
    }
}

impl Editor {
    pub fn new() -> Self {
        Editor { tool: Tool::Brick }
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
        draw_tank(stdout, x + 2, y + 20, 1, &Direction::Up)?;
        draw_tank(stdout, x + 12, y + 20, 2, &Direction::Up)?;
        draw_tank(stdout, x + 2, y + 25, 3, &Direction::Up)?;
        draw_tank(stdout, x + 12, y + 25, 4, &Direction::Up)?;

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

    fn draw_tool(
        &self,
        stdout: &mut Stdout,
        mouse_state: &MouseState,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        if !mouse_state.is_hovered(horizontal_margin, vertical_margin, 96, 48) {
            return Ok(());
        }

        let MouseState { column, row, .. } = mouse_state;
        let column = *column - (*column - horizontal_margin) % 2;

        if let Tool::Tank(player_number, direction) = &self.tool {
            if mouse_state.is_hovered(horizontal_margin, vertical_margin, 92, 46) {
                draw_tank(stdout, column, *row, *player_number, direction)?;
                return Ok(());
            }
        }

        match self.tool {
            Tool::Brick => {
                queue!(
                    stdout,
                    SetForegroundColor(BRICK_FOREGROUND_COLOR),
                    SetBackgroundColor(BRICK_BACKGROUND_COLOR),
                )?;
                draw_brick_block(stdout, column, *row)?;
            }
            Tool::Concrete => {
                queue!(
                    stdout,
                    SetForegroundColor(CONCRETE_FOREGROUND_COLOR),
                    SetBackgroundColor(CONCRETE_BACKGROUND_COLOR),
                )?;
                draw_concrete_block(stdout, column, *row)?;
            }
            Tool::Water => {
                queue!(
                    stdout,
                    SetForegroundColor(WATER_FOREGROUND_COLOR),
                    SetBackgroundColor(WATER_BACKGROUND_COLOR),
                )?;
                draw_water_block(stdout, column, *row)?;
            }
            Tool::Leaves => {
                queue!(
                    stdout,
                    SetForegroundColor(LEAVES_FOREGROUND_COLOR),
                    SetBackgroundColor(LEAVES_BACKGROUND_COLOR),
                )?;
                draw_leaves_block(stdout, column, *row)?;
            }
            _ => {}
        }

        Ok(())
    }
}
