use std::{io::Stdout, slice::Iter};

use crossterm::{cursor, queue, style::Print, Result};

pub mod block;
pub mod tank;

pub fn draw_multi_line_text(
    stdout: &mut Stdout,
    text_iter: Iter<&str>,
    x: u16,
    y: u16,
) -> Result<()> {
    for (i, line) in text_iter.enumerate() {
        queue!(stdout, cursor::MoveTo(x, y + i as u16), Print(line))?;
    }

    Ok(())
}
