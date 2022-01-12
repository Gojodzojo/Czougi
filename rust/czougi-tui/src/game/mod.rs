mod input;
mod modes;
mod options;

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

use self::input::WindowState;

const MIN_WIDTH: u16 = 122;
const MIN_HEIGHT: u16 = 50;

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

            if input_state.window_state.height < MIN_HEIGHT
                || input_state.window_state.width < MIN_WIDTH
            {
                self.print_window_size_information(input_state.window_state)?;
            } else {
                let mut resized = false;
                if self.last_window_state.width != input_state.window_state.width
                    || self.last_window_state.height != input_state.window_state.height
                {
                    resized = true;
                    self.horizontal_margin = (input_state.window_state.width - 122) / 2;
                    self.vertical_margin = (input_state.window_state.height - 50) / 2;
                    self.last_window_state = input_state.window_state.clone();
                    queue!(
                        self.stdout,
                        SetBackgroundColor(Color::DarkBlue),
                        Clear(ClearType::All),
                        cursor::Hide
                    )?;
                }

                self.mode.draw(
                    &mut self.stdout,
                    delta_time,
                    self.horizontal_margin,
                    self.vertical_margin,
                    resized,
                    &input_state,
                )?;
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
        execute!(
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
