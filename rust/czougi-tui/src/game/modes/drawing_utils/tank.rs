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
    player_number: u8,
    direction: &Direction,
) -> Result<()> {
    let color = match player_number {
        1 => Color::Yellow,
        2 => Color::Blue,
        3 => Color::Green,
        4 => Color::Red,
        _ => unreachable!(),
    };

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
