use super::drawing_utils::draw_background;
use super::menu::Menu;
use super::Mode;
use crate::game::input::InputState;
use crate::game::options::Options;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, style::Print, Result};
use std::io::Stdout;
use std::time::Duration;

const OFFLINE_GAME_FRAME_COLOR: Color = Color::Rgb {
    r: 24,
    g: 204,
    b: 36,
};

const ONLINE_GAME_FRAME_COLOR: Color = Color::Rgb {
    r: 240,
    g: 204,
    b: 28,
};

enum Section {
    OfflineGame,
    OnlineGame,
}

pub struct GamePicker {
    current_section: Section,
}

impl Mode for GamePicker {
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
            self.refresh(stdout, horizontal_margin, vertical_margin)?;
        }

        if mouse_state.is_clicked(horizontal_margin + 7, vertical_margin + 2, 15, 3) {
            self.current_section = Section::OfflineGame;
            self.refresh(stdout, horizontal_margin, vertical_margin)?;
        }

        if mouse_state.is_clicked(horizontal_margin + 22, vertical_margin + 2, 15, 3) {
            self.current_section = Section::OnlineGame;
            self.refresh(stdout, horizontal_margin, vertical_margin)?;
        }

        if mouse_state.is_clicked(horizontal_margin, vertical_margin + 5, 6, 3) {
            return Ok(Some(Box::new(Menu::new())));
        }

        Ok(None)
    }
}

impl GamePicker {
    pub fn new() -> Self {
        GamePicker {
            current_section: Section::OfflineGame,
        }
    }

    fn refresh(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        draw_background(stdout, horizontal_margin, vertical_margin)?;
        self.draw_frame(stdout, horizontal_margin, vertical_margin)?;
        self.draw_back_arrow(stdout, horizontal_margin + 2, vertical_margin + 6)?;

        Ok(())
    }

    fn draw_frame(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        match self.current_section {
            Section::OfflineGame => queue!(stdout, SetForegroundColor(OFFLINE_GAME_FRAME_COLOR))?,
            Section::OnlineGame => queue!(stdout, SetForegroundColor(ONLINE_GAME_FRAME_COLOR))?,
        }

        queue!(
            stdout,
            SetBackgroundColor(Color::Black),
            cursor::MoveTo(horizontal_margin + 7, vertical_margin + 2),
            Print("┌──────────────┬─────────────┐"),
            cursor::MoveTo(horizontal_margin + 7, vertical_margin + 3),
            Print("│              │             │"),
            cursor::MoveTo(horizontal_margin + 7, vertical_margin + 4),
            Print("├──────────────┴─────────────┴─────────────────────────────────────────────────────────────────────────────┐"),
        )?;

        for x in [horizontal_margin + 7, horizontal_margin + 114] {
            for y in vertical_margin + 5..vertical_margin + 47 {
                queue!(stdout, cursor::MoveTo(x, y), Print("│"),)?;
            }
        }

        queue!(
            stdout,
            cursor::MoveTo(horizontal_margin + 7, vertical_margin + 47),
            Print("└──────────────────────────────────────────────────────────────────────────────────────────────────────────┘"),
        )?;

        match self.current_section {
            Section::OfflineGame => queue!(
                stdout,
                SetForegroundColor(Color::White),
                SetBackgroundColor(OFFLINE_GAME_FRAME_COLOR),
                cursor::MoveTo(horizontal_margin + 8, vertical_margin + 3),
                Print(" Offline game "),
                SetBackgroundColor(Color::Black),
                cursor::MoveTo(horizontal_margin + 24, vertical_margin + 3),
                Print("Online game"),
            )?,
            Section::OnlineGame => queue!(
                stdout,
                SetForegroundColor(Color::White),
                SetBackgroundColor(ONLINE_GAME_FRAME_COLOR),
                cursor::MoveTo(horizontal_margin + 23, vertical_margin + 3),
                Print(" Online game "),
                SetBackgroundColor(Color::Black),
                cursor::MoveTo(horizontal_margin + 8, vertical_margin + 3),
                Print(" Offline game "),
            )?,
        }

        Ok(())
    }

    fn draw_back_arrow(&self, stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        queue!(
            stdout,
            cursor::MoveTo(x, y),
            SetBackgroundColor(Color::Black),
            SetForegroundColor(Color::Red),
            Print("◄--"),
        )?;

        Ok(())
    }
}
