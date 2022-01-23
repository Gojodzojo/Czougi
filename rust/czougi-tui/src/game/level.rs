pub struct Level {
    pub bricks: Vec<Block>,
    pub concretes: Vec<Block>,
    pub waters: Vec<Block>,
    pub leaves: Vec<Block>,
    pub tanks: [Option<Tank>; 4],
}

impl Level {
    pub fn new() -> Self {
        Level {
            bricks: vec![],
            concretes: vec![],
            waters: vec![],
            leaves: vec![],
            tanks: [None, None, None, None],
        }
    }
}

pub struct Block {
    pub x: u16,
    pub y: u16,
}

pub struct Tank {
    pub x: u16,
    pub y: u16,
    pub direction: Direction,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
