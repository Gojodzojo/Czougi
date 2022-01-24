use crossterm::{
    cursor, queue,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    Result,
};
use std::io::Stdout;

pub(super) fn draw_grid(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
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
