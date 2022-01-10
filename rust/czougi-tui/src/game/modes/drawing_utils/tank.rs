use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
    Result,
};
use std::io::Stdout;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn draw_tank(
    stdout: &mut Stdout,
    x: u16,
    y: u16,
    color: Color,
    direction: Direction,
) -> Result<()> {
    queue!(stdout, SetForegroundColor(color))?;

    match direction {
        Direction::Up => queue!(
            stdout,
            cursor::MoveTo(x + 3, y),
            Print("▐▌"),
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
            cursor::MoveTo(x + 3, y + 3),
            Print("▐▌"),
        )?,
        Direction::Left => queue!(
            stdout,
            cursor::MoveTo(x + 3, y),
            Print("█████"),
            cursor::MoveTo(x, y + 1),
            Print("▄▄█████"),
            cursor::MoveTo(x, y + 2),
            Print("▀▀█████"),
            cursor::MoveTo(x + 3, y + 3),
            Print("█████"),
        )?,
        Direction::Right => queue!(
            stdout,
            cursor::MoveTo(x, y),
            Print("█████"),
            cursor::MoveTo(x + 1, y + 1),
            Print("█████▄▄"),
            cursor::MoveTo(x + 1, y + 2),
            Print("█████▀▀"),
            cursor::MoveTo(x, y + 3),
            Print("█████"),
        )?,
    }

    Ok(())
}
