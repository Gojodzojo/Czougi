use super::drawing_utils::draw_multi_line_text;
use super::Mode;
use crate::game::input::InputState;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, terminal, Result};
use std::io::{Stdout, Write};
use std::time::Duration;

// Text generated using this tool: https://patorjk.com/software/taag/#p=display&f=Big%20Money-ne&t=CZOUGI

const TITLE: [&str; 8] = [
    "  /$$$$$$  /$$$$$$$$  /$$$$$$  /$$   /$$  /$$$$$$  /$$$$$$",
    " /$$__  $$|_____ $$  /$$__  $$| $$  | $$ /$$__  $$|_  $$_/",
    "| $$  \\__/     /$$/ | $$  \\ $$| $$  | $$| $$  \\__/  | $$",
    "| $$          /$$/  | $$  | $$| $$  | $$| $$ /$$$$  | $$",
    "| $$         /$$/   | $$  | $$| $$  | $$| $$|_  $$  | $$",
    "| $$    $$  /$$/    | $$  | $$| $$  | $$| $$  \\ $$  | $$",
    "|  $$$$$$/ /$$$$$$$$|  $$$$$$/|  $$$$$$/|  $$$$$$/ /$$$$$$",
    " \\______/ |________/ \\______/  \\______/  \\______/ |______/ ",
];

const PLAY_BUTTON_TEXT: [&str; 5] = [
    "████████████████████████████████████████████████",
    "█████████████  _ ▐▌  ▐██▌  _  ▐▌ │ ▐████████████",
    "█████████████    ▐▌  ▐██▌     ▐▌   ▐████████████",
    "█████████████  ███▌    ▐▌  |  ▐█▌ ▐█████████████",
    "████████████████████████████████████████████████",
];

const OPTIONS_BUTTON_TEXT: [&str; 5] = [
    "████████████████████████████████████████████████",
    "█     ▐▌  _ ▐▌     ▐█     █▌     ▐▌   │ ▐█   __█",
    "█  │  ▐▌    ▐██▌ ▐███─   ─█▌  │  ▐▌     ▐█__   █",
    "█     ▐▌  █████▌ ▐███     █▌     ▐▌ │   ▐█     █",
    "████████████████████████████████████████████████",
];

const BUTTON_FRAME: [&str; 7] = [
    "┌──────────────────────────────────────────────────┐",
    "│                                                  │",
    "│                                                  │",
    "│                                                  │",
    "│                                                  │",
    "│                                                  │",
    "└──────────────────────────────────────────────────┘",
];

const BUTTON_WIDTH: u16 = 50;
const BUTTON_HEIGHT: u16 = 5;
pub struct Menu;

impl Mode for Menu {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        delta_time: Duration,
        horizontal_margin: u16,
        vertical_margin: u16,
        resized: bool,
        input_state: &InputState,
    ) -> Result<()> {
        let InputState { mouse_state, .. } = input_state;

        if resized {
            self.draw_background(stdout, horizontal_margin, vertical_margin)?;

            // Draw title
            queue!(stdout, SetForegroundColor(Color::Red))?;
            draw_multi_line_text(
                stdout,
                TITLE.iter(),
                horizontal_margin + 32,
                vertical_margin + 5,
            )?;

            // Draw buttons' frames
            queue!(
                stdout,
                SetBackgroundColor(Color::Black),
                SetForegroundColor(Color::White)
            )?;
            draw_multi_line_text(
                stdout,
                BUTTON_FRAME.iter(),
                horizontal_margin + 35,
                vertical_margin + 20,
            )?;
            draw_multi_line_text(
                stdout,
                BUTTON_FRAME.iter(),
                horizontal_margin + 35,
                vertical_margin + 30,
            )?;

            queue!(
                stdout,
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::Black),
                cursor::MoveTo(horizontal_margin + 105, vertical_margin + 49),
                Print("Mateusz Goik 2022")
            )?;
        }

        let is_play_button_hovered = mouse_state.is_hovered(
            horizontal_margin + 36,
            vertical_margin + 21,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        );

        let is_options_button_hovered = mouse_state.is_hovered(
            horizontal_margin + 36,
            vertical_margin + 31,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        );

        self.draw_button(
            stdout,
            PLAY_BUTTON_TEXT,
            horizontal_margin + 37,
            vertical_margin + 21,
            is_play_button_hovered,
        )?;

        self.draw_button(
            stdout,
            OPTIONS_BUTTON_TEXT,
            horizontal_margin + 37,
            vertical_margin + 31,
            is_options_button_hovered,
        )?;

        Ok(())
    }
}

impl Menu {
    pub fn new() -> Self {
        Menu
    }

    fn draw_background(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        queue!(stdout, SetBackgroundColor(Color::Black))?;

        for row in y..50 + y {
            queue!(
                stdout,
                cursor::MoveTo(x, row),
                Print("                                                                                                                          "),
            )?;
        }

        Ok(())
    }

    fn draw_button(
        &self,
        stdout: &mut Stdout,
        button_text: [&str; 5],
        x: u16,
        y: u16,
        hovered: bool,
    ) -> Result<()> {
        if hovered {
            queue!(
                stdout,
                SetBackgroundColor(Color::Black),
                SetForegroundColor(Color::White)
            )?;
        } else {
            queue!(
                stdout,
                SetBackgroundColor(Color::White),
                SetForegroundColor(Color::Black)
            )?;
        }

        draw_multi_line_text(stdout, button_text.iter(), x, y)?;

        Ok(())
    }
}
