mod input;
mod modes;
mod options;

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};
use input::Input;
use modes::{menu::Menu, Mode};
use options::Options;
use std::{io::Stdout, thread::sleep, time::Instant};

pub struct Game {
    stdout: Stdout,
    mode: Box<dyn Mode>,
    options: Options,
    input: Input,
}

impl Game {
    pub fn new(stdout: Stdout) -> Result<Self> {
        Ok(Game {
            stdout,
            mode: Box::new(Menu::new()),
            options: Options::new(),
            input: Input::new()?,
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

            let input_state = self.input.get_state(&self.options.keybindings);

            if input_state.ctrl_c {
                break;
            }

            self.mode.draw(&mut self.stdout, delta_time, &input_state)?;

            let desired_time = current_time + self.options.interval;
            let now = Instant::now();
            if desired_time > now {
                sleep(desired_time - now);
            }
        }

        Ok(())
    }
}
