use std::io::Stdout;

use crossterm::{
    cursor, queue,
    style::{Color, Print},
    Result,
};

pub const BRICK_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 119,
    g: 43,
    b: 21,
};

pub const BRICK_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 116,
    g: 91,
    b: 68,
};

pub const CONCRETE_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 196,
    g: 196,
    b: 196,
};

pub const CONCRETE_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 160,
    g: 160,
    b: 160,
};

pub const WATER_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 66,
    g: 66,
    b: 255,
};

pub const WATER_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 160,
    g: 207,
    b: 242,
};

pub const LEAVES_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 140,
    g: 214,
    b: 0,
};

pub const LEAVES_BACKGROUND_COLOR: Color = Color::Rgb { r: 0, g: 82, b: 8 };

pub fn draw_leaves_block(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print("█▀▄▀"),
        cursor::MoveTo(x, y + 1),
        Print("▄▀▄█"),
    )?;
    Ok(())
}

pub fn draw_brick_block(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print("▄▄▀▀"),
        cursor::MoveTo(x, y + 1),
        Print("▄▄▀▀"),
    )?;
    Ok(())
}

pub fn draw_water_block(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print("█▄█▀"),
        cursor::MoveTo(x, y + 1),
        Print("▄▄▀█"),
    )?;
    Ok(())
}

pub fn draw_concrete_block(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print(" ▄▄ "),
        cursor::MoveTo(x, y + 1),
        Print(" ▀▀ "),
    )?;
    Ok(())
}

pub fn draw_full_block(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x, y),
        Print("████"),
        cursor::MoveTo(x, y + 1),
        Print("████"),
    )?;
    Ok(())
}
