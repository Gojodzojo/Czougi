use super::Mode;
use crate::game::input::InputState;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, terminal, Result};
use std::io::{Stdout, Write};
use std::time::Duration;

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
            self.draw_title(stdout, horizontal_margin + 32, vertical_margin + 5)?;
            self.draw_play_button(stdout, horizontal_margin + 40, vertical_margin + 20)?;
            self.draw_options_button(stdout, horizontal_margin + 40, vertical_margin + 29)?;

            queue!(
                stdout,
                SetForegroundColor(Color::White),
                cursor::MoveTo(horizontal_margin + 105, vertical_margin + 49),
                Print("Mateusz Goik 2022")
            )?;
        }

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

    fn draw_title(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        let lines = [
            "  /$$$$$$  /$$$$$$$$  /$$$$$$  /$$   /$$  /$$$$$$  /$$$$$$",
            " /$$__  $$|_____ $$  /$$__  $$| $$  | $$ /$$__  $$|_  $$_/",
            "| $$  \\__/     /$$/ | $$  \\ $$| $$  | $$| $$  \\__/  | $$",
            "| $$          /$$/  | $$  | $$| $$  | $$| $$ /$$$$  | $$",
            "| $$         /$$/   | $$  | $$| $$  | $$| $$|_  $$  | $$",
            "| $$    $$  /$$/    | $$  | $$| $$  | $$| $$  \\ $$  | $$",
            "|  $$$$$$/ /$$$$$$$$|  $$$$$$/|  $$$$$$/|  $$$$$$/ /$$$$$$",
            " \\______/ |________/ \\______/  \\______/  \\______/ |______/ ",
        ];

        queue!(stdout, SetForegroundColor(Color::Red))?;

        for (i, line) in lines.iter().enumerate() {
            queue!(stdout, cursor::MoveTo(x, y + i as u16), Print(line))?;
        }

        Ok(())
    }

    fn draw_play_button(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        let lines = [
            "┌────────────────────────────────────────┐",
            "│         ____  __     __   _  _         │",
            "│        (  _ \\(  )   / _\\ ( \\/ )        │",
            "│         ) __// (_/\\/    \\ )  /         │",
            "│        (__)  \\____/\\_/\\_/(__/          │",
            "│                                        │",
            "└────────────────────────────────────────┘",
        ];

        queue!(stdout, SetForegroundColor(Color::White))?;

        for (i, line) in lines.iter().enumerate() {
            queue!(stdout, cursor::MoveTo(x, y + i as u16), Print(line))?;
        }

        Ok(())
    }

    fn draw_options_button(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        let lines = [
            "┌────────────────────────────────────────┐",
            "│   __  ____  ____  __  __   __ _  ____  │",
            "│  /  \\(  _ \\(_  _)(  )/  \\ (  ( \\/ ___) │",
            "│ (  O )) __/  )(   )((  O )/    /\\___ \\ │",
            "│  \\__/(__)   (__) (__)\\__/ \\_)__)(____/ │",
            "│                                        │",
            "└────────────────────────────────────────┘",
        ];

        queue!(stdout, SetForegroundColor(Color::White))?;

        for (i, line) in lines.iter().enumerate() {
            queue!(stdout, cursor::MoveTo(x, y + i as u16), Print(line))?;
        }

        Ok(())
    }
}
