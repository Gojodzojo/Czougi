use std::io::Stdout;

use crossterm::{
    cursor, queue,
    style::{Color, Print},
    Result,
};

pub const BRICK_COLOR: Color = Color::Rgb {
    r: 156,
    g: 76,
    b: 0,
};

pub const CONCRETE_COLOR: Color = Color::Rgb {
    r: 196,
    g: 196,
    b: 196,
};

pub const WATER_COLOR: Color = Color::Rgb {
    r: 68,
    g: 68,
    b: 252,
};

pub const GRASS_COLOR: Color = Color::Rgb {
    r: 98,
    g: 173,
    b: 4,
};

pub fn draw_block(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print("████"),
        cursor::MoveTo(x, y + 1),
        Print("████"),
    )?;
    Ok(())
}
