mod drawing_utils;
mod input;
mod level;
mod modes;
mod options;

use self::{
    input::WindowState,
    level::{LEVEL_MAP_WIDTH, LEVEL_SIZE},
    modes::SIDEBAR_WIDTH,
};
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};
use device_query::Keycode;
use input::Input;
use modes::{menu::Menu, Mode};
use options::Options;
use std::{
    io::{Stdout, Write},
    thread::sleep,
    time::Instant,
};

const MIN_WIDTH: u16 = LEVEL_MAP_WIDTH + SIDEBAR_WIDTH;
const MIN_HEIGHT: u16 = LEVEL_SIZE;

pub struct Game {
    stdout: Stdout,
    mode: Box<dyn Mode>,
    options: Options,
    input: Input,
    horizontal_margin: u16,
    vertical_margin: u16,
    last_window_state: WindowState,
}

impl Game {
    pub fn new(stdout: Stdout) -> Result<Self> {
        Ok(Game {
            stdout,
            mode: Box::new(Menu::new()),
            options: Options::new(),
            input: Input::new()?,
            horizontal_margin: 0,
            vertical_margin: 0,
            last_window_state: WindowState {
                width: 0,
                height: 0,
            },
        })
    }

    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, EnableMouseCapture, cursor::Hide)?;
        Ok(())
    }

    pub fn uninit(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            DisableMouseCapture,
            cursor::Show,
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut current_time = Instant::now();
        let mut previous_time;

        loop {
            previous_time = current_time;
            current_time = Instant::now();
            let delta_time = current_time - previous_time;

            let input_state = self.input.get_state();
            let ctrl_c = input_state.keyboard_state.contains(&Keycode::LControl)
                && input_state.keyboard_state.contains(&Keycode::C);

            if ctrl_c {
                break;
            }

            let window_too_small = input_state.window_state.height < MIN_HEIGHT
                || input_state.window_state.width < MIN_WIDTH;

            if window_too_small {
                self.print_window_size_information(input_state.window_state)?;
            } else {
                let mut refresh = false;
                let window_resized = self.last_window_state.width != input_state.window_state.width
                    || self.last_window_state.height != input_state.window_state.height;

                if window_resized {
                    refresh = true;
                    self.horizontal_margin = (input_state.window_state.width - MIN_WIDTH) / 2;
                    self.vertical_margin = (input_state.window_state.height - MIN_HEIGHT) / 2;
                    self.last_window_state = input_state.window_state.clone();
                    queue!(
                        self.stdout,
                        SetBackgroundColor(Color::DarkBlue),
                        Clear(ClearType::All),
                        cursor::Hide
                    )?;
                }

                let new_mode = self.mode.draw(
                    &mut self.stdout,
                    delta_time,
                    self.horizontal_margin,
                    self.vertical_margin,
                    refresh,
                    &input_state,
                    &self.options,
                )?;

                if let Some(new_mode) = new_mode {
                    self.mode = new_mode;
                    // Trigger a refresh
                    self.last_window_state = WindowState {
                        width: 0,
                        height: 0,
                    };
                }
            }

            self.stdout.flush()?;

            let desired_time = current_time + self.options.interval;
            let now = Instant::now();
            if desired_time > now {
                sleep(desired_time - now);
            }
        }

        Ok(())
    }

    fn print_window_size_information(&mut self, window_state: WindowState) -> Result<()> {
        queue!(
            self.stdout,
            cursor::Show,
            SetBackgroundColor(Color::Black),
            SetForegroundColor(Color::White),
            Clear(ClearType::All),
            cursor::MoveTo(1, 1),
            SetAttribute(Attribute::Encircled),
            Print(format!(
                "Window must be at least {}x{}. Your window is: {}x{}",
                MIN_WIDTH, MIN_HEIGHT, window_state.width, window_state.height
            )),
        )?;
        Ok(())
    }
}
