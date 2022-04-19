use std::{hash::Hash, io::Stdout};

use crossterm::{
    cursor, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    Result,
};

pub const BRICK_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 119,
    g: 43,
    b: 21,
};

pub const BRICK_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 116,
    g: 91,
    b: 68,
};

pub const CONCRETE_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 196,
    g: 196,
    b: 196,
};

pub const CONCRETE_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 160,
    g: 160,
    b: 160,
};

pub const WATER_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 66,
    g: 66,
    b: 255,
};

pub const WATER_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 160,
    g: 207,
    b: 242,
};

pub const LEAVES_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 140,
    g: 214,
    b: 0,
};

pub const LEAVES_BACKGROUND_COLOR: Color = Color::Rgb { r: 0, g: 82, b: 8 };

#[derive(Copy, Clone)]
pub enum BlockType {
    Brick,
    Concrete,
    Water,
    Leaves,
}

#[derive(Copy, Clone)]
pub enum BlockVariant {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

pub struct Block {
    pub x: u16,
    pub y: u16,
    pub block_type: BlockType,
    pub block_variant: BlockVariant,
}

impl Block {
    pub fn draw(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        draw_block(
            stdout,
            self.block_type,
            self.block_variant,
            horizontal_margin + self.x * 2,
            vertical_margin + self.y,
        )
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Eq for Block {}

impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub fn draw_block(
    stdout: &mut Stdout,
    block_type: BlockType,
    block_variant: BlockVariant,
    x: u16,
    y: u16,
) -> Result<()> {
    let (background_color, foreground_color, graphics) = match (block_type, block_variant) {
        (BlockType::Brick, BlockVariant::LeftTop) => {
            (BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, "▄▄")
        }
        (BlockType::Brick, BlockVariant::RightTop) => {
            (BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, "▀▀")
        }
        (BlockType::Brick, BlockVariant::LeftBottom) => {
            (BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, "▄▄")
        }
        (BlockType::Brick, BlockVariant::RightBottom) => {
            (BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, "▀▀")
        }
        (BlockType::Concrete, BlockVariant::LeftTop) => {
            (CONCRETE_BACKGROUND_COLOR, CONCRETE_FOREGROUND_COLOR, " ▄")
        }
        (BlockType::Concrete, BlockVariant::RightTop) => {
            (CONCRETE_BACKGROUND_COLOR, CONCRETE_FOREGROUND_COLOR, "▄ ")
        }
        (BlockType::Concrete, BlockVariant::LeftBottom) => {
            (CONCRETE_BACKGROUND_COLOR, CONCRETE_FOREGROUND_COLOR, " ▀")
        }
        (BlockType::Concrete, BlockVariant::RightBottom) => {
            (CONCRETE_BACKGROUND_COLOR, CONCRETE_FOREGROUND_COLOR, "▀ ")
        }
        (BlockType::Water, BlockVariant::LeftTop) => {
            (WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR, "█▄")
        }
        (BlockType::Water, BlockVariant::RightTop) => {
            (WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR, "█▀")
        }
        (BlockType::Water, BlockVariant::LeftBottom) => {
            (WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR, "▄▄")
        }
        (BlockType::Water, BlockVariant::RightBottom) => {
            (WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR, "▀█")
        }
        (BlockType::Leaves, BlockVariant::LeftTop) => {
            (LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR, "█▀")
        }
        (BlockType::Leaves, BlockVariant::RightTop) => {
            (LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR, "▄▀")
        }
        (BlockType::Leaves, BlockVariant::LeftBottom) => {
            (LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR, "▄▀")
        }
        (BlockType::Leaves, BlockVariant::RightBottom) => {
            (LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR, "▄█")
        }
    };

    queue!(
        stdout,
        SetBackgroundColor(background_color),
        SetForegroundColor(foreground_color),
        cursor::MoveTo(x, y),
        Print(graphics)
    )?;
    Ok(())
}

pub fn draw_full_block(stdout: &mut Stdout, block_type: BlockType, x: u16, y: u16) -> Result<()> {
    draw_block(stdout, block_type, BlockVariant::LeftTop, x, y)?;

    draw_block(stdout, block_type, BlockVariant::RightTop, x + 2, y)?;

    draw_block(stdout, block_type, BlockVariant::LeftBottom, x, y + 1)?;

    draw_block(stdout, block_type, BlockVariant::RightBottom, x + 2, y + 1)?;
    Ok(())
}
