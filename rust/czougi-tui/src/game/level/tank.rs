use std::io::Stdout;

use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
    Result,
};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Tank {
    pub x: u16,
    pub y: u16,
    pub direction: Direction,
}

pub const TANK_SIZE: u16 = 4;

impl Tank {
    pub fn draw(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
        player_number: u8,
    ) -> Result<()> {
        draw_tank(
            stdout,
            self.x * 2 + horizontal_margin,
            self.y + vertical_margin,
            player_number,
            self.direction,
        )
    }
}

pub fn draw_tank(
    stdout: &mut Stdout,
    x: u16,
    y: u16,
    player_number: u8,
    direction: Direction,
) -> Result<()> {
    let color = match player_number {
        0 => Color::Yellow,
        1 => Color::Blue,
        2 => Color::Green,
        3 => Color::Red,
        _ => unreachable!(),
    };

    queue!(stdout, SetForegroundColor(color))?;

    match direction {
        Direction::Up => queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("   ▐▌   "),
            cursor::MoveTo(x, y + 1),
            Print("▄▄████▄▄"),
            cursor::MoveTo(x, y + 2),
            Print("████████"),
            cursor::MoveTo(x, y + 3),
            Print("██▀▀▀▀██"),
        )?,
        Direction::Down => queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("██▄▄▄▄██"),
            cursor::MoveTo(x, y + 1),
            Print("████████"),
            cursor::MoveTo(x, y + 2),
            Print("▀▀████▀▀"),
            cursor::MoveTo(x, y + 3),
            Print("   ▐▌   "),
        )?,
        Direction::Left => queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("   █████"),
            cursor::MoveTo(x, y + 1),
            Print("▄▄█████ "),
            cursor::MoveTo(x, y + 2),
            Print("▀▀█████ "),
            cursor::MoveTo(x, y + 3),
            Print("   █████"),
        )?,
        Direction::Right => queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("█████   "),
            cursor::MoveTo(x, y + 1),
            Print(" █████▄▄"),
            cursor::MoveTo(x, y + 2),
            Print(" █████▀▀"),
            cursor::MoveTo(x, y + 3),
            Print("█████   "),
        )?,
    }

    Ok(())
}
