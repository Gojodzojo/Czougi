use super::options::PlayerKeybindings;
use crossterm::{
    event::{read, Event, MouseButton, MouseEvent, MouseEventKind},
    terminal::size,
    Result,
};
use device_query::Keycode;
use device_query::{DeviceQuery, DeviceState};
use std::sync::{mpsc, Arc, Mutex};

pub struct Input {
    mouse_state: Arc<Mutex<MouseState>>,
    window_state: Arc<Mutex<WindowState>>,
    device_state: DeviceState,
    stoppper_tx: mpsc::Sender<()>,
}

impl Input {
    pub fn new() -> Result<Self> {
        let (stoppper_tx, stoppper_rx) = mpsc::channel();

        let input = Input {
            mouse_state: Arc::new(Mutex::new(MouseState::new())),
            window_state: Arc::new(Mutex::new(WindowState::new()?)),
            device_state: DeviceState::new(),
            stoppper_tx,
        };

        let mouse_state = input.mouse_state.clone();
        let window_state = input.window_state.clone();

        // Thread updating mouse and window state
        std::thread::spawn(move || loop {
            if let Ok(()) = stoppper_rx.try_recv() {
                break;
            }

            match read().unwrap() {
                Event::Mouse(mouse_event) => {
                    let mut mouse_state = mouse_state.lock().unwrap();
                    mouse_state.update(mouse_event);
                }
                Event::Resize(width, height) => {
                    let mut window_state = window_state.lock().unwrap();
                    window_state.update(width, height);
                }
                _ => {}
            }
        });

        Ok(input)
    }

    pub fn get_state(&mut self, keybindings: &[PlayerKeybindings; 4]) -> Result<InputState> {
        Ok(InputState {
            mouse_state: self.mouse_state.lock().unwrap().get_state(),
            window_state: self.window_state.lock().unwrap().get_state(),
            players_keys_states: self.get_players_keys_state(&keybindings),
        })
    }

    fn get_players_keys_state(&self, keybindings: &[PlayerKeybindings; 4]) -> [PlayerKeysState; 4] {
        let keys = self.device_state.get_keys();
        [
            PlayerKeysState::new(&keys, &keybindings[0]),
            PlayerKeysState::new(&keys, &keybindings[1]),
            PlayerKeysState::new(&keys, &keybindings[2]),
            PlayerKeysState::new(&keys, &keybindings[3]),
        ]
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        self.stoppper_tx.send(()).unwrap();
    }
}

pub struct InputState {
    pub mouse_state: MouseState,
    pub players_keys_states: [PlayerKeysState; 4],
    pub window_state: WindowState,
}

pub struct PlayerKeysState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub shoot: bool,
}

impl PlayerKeysState {
    pub fn new(keys: &Vec<Keycode>, keybindings: &PlayerKeybindings) -> Self {
        PlayerKeysState {
            up: keys.contains(&keybindings.up),
            down: keys.contains(&keybindings.down),
            left: keys.contains(&keybindings.left),
            right: keys.contains(&keybindings.right),
            shoot: keys.contains(&keybindings.shoot),
        }
    }
}

#[derive(Clone)]
pub struct MouseState {
    pub column: u16,
    pub row: u16,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            column: 1,
            row: 1,
            left_button: false,
            right_button: false,
            middle_button: false,
        }
    }

    pub fn update(&mut self, mouse_event: MouseEvent) {
        self.column = mouse_event.column;
        self.row = mouse_event.row;

        match mouse_event.kind {
            MouseEventKind::Up(button) => match button {
                MouseButton::Left => self.left_button = false,
                MouseButton::Right => self.right_button = false,
                MouseButton::Middle => self.middle_button = false,
            },
            MouseEventKind::Down(button) => match button {
                MouseButton::Left => self.left_button = true,
                MouseButton::Right => self.right_button = true,
                MouseButton::Middle => self.middle_button = true,
            },
            _ => {}
        }
    }

    pub fn get_state(&self) -> MouseState {
        self.clone()
    }
}

#[derive(Clone)]
pub struct WindowState {
    pub width: u16,
    pub height: u16,
}

impl WindowState {
    pub fn new() -> Result<Self> {
        let (width, height) = size()?;
        Ok(WindowState { width, height })
    }

    pub fn update(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn get_state(&self) -> WindowState {
        self.clone()
    }
}
