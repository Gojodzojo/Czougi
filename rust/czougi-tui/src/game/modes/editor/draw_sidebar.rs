use crate::game::drawing_utils::{
    block::{
        draw_brick_block, draw_concrete_block, draw_leaves_block, draw_water_block,
        BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, CONCRETE_BACKGROUND_COLOR,
        CONCRETE_FOREGROUND_COLOR, LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR,
        WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR,
    },
    draw_multi_line_text,
    tank::draw_tank,
};
use crate::game::level::Direction;
use crossterm::style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, Result};
use std::io::Stdout;

const ERASER: [&str; 4] = ["▄▄    ▄▄", " ▀▀▄▄▀▀", " ▄▄▀▀▄▄", "▀▀    ▀▀"];

pub(super) fn draw_sidebar(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
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
    draw_tank(stdout, x + 2, y + 20, 0, &Direction::Up)?;
    draw_tank(stdout, x + 12, y + 20, 1, &Direction::Up)?;
    draw_tank(stdout, x + 2, y + 25, 2, &Direction::Up)?;
    draw_tank(stdout, x + 12, y + 25, 3, &Direction::Up)?;

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
