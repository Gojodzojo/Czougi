use super::drawing_utils::{draw_background, draw_multi_line_text};
use super::editor::Editor;
use super::game_picker::GamePicker;
use super::Mode;
use crate::game::input::{InputState, MouseState};
use crate::game::options::Options;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, Result};
use std::io::Stdout;
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

const BUTTON_WIDTH: u16 = 51;
const BUTTON_HEIGHT: u16 = 6;
pub struct Menu;

impl Mode for Menu {
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
            draw_background(stdout, horizontal_margin, vertical_margin)?;
            self.draw_title(stdout, horizontal_margin + 32, vertical_margin + 5)?;
            self.draw_buttons_frames(stdout, horizontal_margin, vertical_margin)?;
            self.draw_signature(stdout, horizontal_margin + 105, vertical_margin + 49)?;
        }

        self.draw_play_button(
            stdout,
            mouse_state,
            horizontal_margin + 37,
            vertical_margin + 21,
        )?;

        if mouse_state.is_clicked(
            horizontal_margin + 35,
            vertical_margin + 20,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        ) {
            return Ok(Some(Box::new(GamePicker::new())));
        }

        self.draw_options_button(
            stdout,
            mouse_state,
            horizontal_margin + 37,
            vertical_margin + 31,
        )?;

        if mouse_state.is_clicked(
            horizontal_margin + 35,
            vertical_margin + 30,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        ) {
            return Ok(Some(Box::new(Editor::new())));
        }

        Ok(None)
    }
}

impl Menu {
    pub fn new() -> Self {
        Menu
    }

    fn draw_title(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        queue!(stdout, SetForegroundColor(Color::Red))?;
        draw_multi_line_text(stdout, TITLE.iter(), x, y)?;

        Ok(())
    }

    fn draw_signature(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        queue!(
            stdout,
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::Black),
            cursor::MoveTo(x, y),
            Print("Mateusz Goik 2022")
        )?;

        Ok(())
    }

    fn draw_buttons_frames(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
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

        Ok(())
    }

    fn draw_button_content(
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

    fn draw_play_button(
        &self,
        stdout: &mut Stdout,
        mouse_state: &MouseState,
        x: u16,
        y: u16,
    ) -> Result<()> {
        let is_play_button_hovered =
            mouse_state.is_hovered(x - 2, y - 1, BUTTON_WIDTH, BUTTON_HEIGHT);
        self.draw_button_content(stdout, PLAY_BUTTON_TEXT, x, y, is_play_button_hovered)?;

        Ok(())
    }

    fn draw_options_button(
        &self,
        stdout: &mut Stdout,
        mouse_state: &MouseState,
        x: u16,
        y: u16,
    ) -> Result<()> {
        let is_options_button_hovered =
            mouse_state.is_hovered(x - 2, y - 1, BUTTON_WIDTH, BUTTON_HEIGHT);
        self.draw_button_content(stdout, OPTIONS_BUTTON_TEXT, x, y, is_options_button_hovered)?;

        Ok(())
    }
}
