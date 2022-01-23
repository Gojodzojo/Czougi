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

    pub fn get_state(&mut self) -> InputState {
        InputState {
            mouse_state: self.mouse_state.lock().unwrap().get_state(),
            window_state: self.window_state.lock().unwrap().get_state(),
            keyboard_state: self.device_state.get_keys(),
        }
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        self.stoppper_tx.send(()).unwrap();
    }
}

pub struct InputState {
    pub mouse_state: MouseState,
    pub keyboard_state: Vec<Keycode>,
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

    pub fn get_players_keys_state(
        keys: &Vec<Keycode>,
        keybindings: &[PlayerKeybindings; 4],
    ) -> [PlayerKeysState; 4] {
        [
            PlayerKeysState::new(&keys, &keybindings[0]),
            PlayerKeysState::new(&keys, &keybindings[1]),
            PlayerKeysState::new(&keys, &keybindings[2]),
            PlayerKeysState::new(&keys, &keybindings[3]),
        ]
    }
}

#[derive(Clone)]
pub enum ScrollState {
    Up,
    Down,
    None,
}

#[derive(Clone)]
pub enum ButtonState {
    Pressed,
    Released,
    GettingPressed,
    GettingReleased,
}

#[derive(Clone)]
pub struct MouseState {
    pub column: u16,
    pub row: u16,
    pub left_button: ButtonState,
    pub scroll: ScrollState,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            column: 1,
            row: 1,
            left_button: ButtonState::Released,
            scroll: ScrollState::None,
        }
    }

    pub fn update(&mut self, mouse_event: MouseEvent) {
        self.column = mouse_event.column;
        self.row = mouse_event.row;

        match mouse_event.kind {
            MouseEventKind::Up(MouseButton::Left) => {
                match self.left_button {
                    ButtonState::GettingPressed => self.left_button = ButtonState::GettingReleased,
                    ButtonState::GettingReleased => self.left_button = ButtonState::Released,
                    ButtonState::Pressed => self.left_button = ButtonState::GettingReleased,
                    _ => {}
                };
            }
            MouseEventKind::Down(MouseButton::Left) => {
                match self.left_button {
                    ButtonState::GettingPressed => self.left_button = ButtonState::Pressed,
                    ButtonState::GettingReleased => self.left_button = ButtonState::GettingPressed,
                    ButtonState::Released => self.left_button = ButtonState::GettingPressed,
                    _ => {}
                };
            }
            MouseEventKind::ScrollUp => {
                self.scroll = ScrollState::Up;
            }
            MouseEventKind::ScrollDown => {
                self.scroll = ScrollState::Down;
            }
            _ => {}
        }
    }

    pub fn get_state(&mut self) -> MouseState {
        let state = self.clone();
        self.scroll = ScrollState::None;
        match self.left_button {
            ButtonState::GettingPressed => self.left_button = ButtonState::Pressed,
            ButtonState::GettingReleased => self.left_button = ButtonState::Released,
            _ => {}
        };
        state
    }

    pub fn is_hovered(&self, x: u16, y: u16, width: u16, height: u16) -> bool {
        self.column >= x && self.column <= x + width && self.row >= y && self.row <= y + height
    }

    pub fn is_clicked(&self, x: u16, y: u16, width: u16, height: u16) -> bool {
        self.is_hovered(x, y, width, height)
            && matches!(self.left_button, ButtonState::GettingReleased)
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
