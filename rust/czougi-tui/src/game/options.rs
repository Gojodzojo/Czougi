use device_query::Keycode;
use std::time::Duration;
pub struct PlayerKeybindings {
    pub up: Keycode,
    pub down: Keycode,
    pub left: Keycode,
    pub right: Keycode,
    pub shoot: Keycode,
}

pub struct Options {
    pub keybindings: [PlayerKeybindings; 4],
    pub interval: Duration,
}

impl Options {
    pub fn new() -> Self {
        Options {
            interval: Duration::from_millis(1000 / 60),
            keybindings: [
                PlayerKeybindings {
                    up: Keycode::W,
                    down: Keycode::S,
                    left: Keycode::A,
                    right: Keycode::D,
                    shoot: Keycode::E,
                },
                PlayerKeybindings {
                    up: Keycode::W,
                    down: Keycode::S,
                    left: Keycode::A,
                    right: Keycode::D,
                    shoot: Keycode::E,
                },
                PlayerKeybindings {
                    up: Keycode::W,
                    down: Keycode::S,
                    left: Keycode::A,
                    right: Keycode::D,
                    shoot: Keycode::E,
                },
                PlayerKeybindings {
                    up: Keycode::W,
                    down: Keycode::S,
                    left: Keycode::A,
                    right: Keycode::D,
                    shoot: Keycode::E,
                },
            ],
        }
    }
}
