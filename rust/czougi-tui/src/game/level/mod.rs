pub mod block;
pub mod tank;

use self::{block::Block, tank::Tank};
use crossterm::{
    cursor, queue,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    Result,
};
use std::{collections::HashSet, io::Stdout};
pub struct Level {
    pub blocks: HashSet<Block>,
    pub tanks: [Option<Tank>; 4],
}

pub const LEVEL_SIZE: u16 = 50;
pub const LEVEL_MAP_WIDTH: u16 = 100;

impl Level {
    pub fn new() -> Self {
        Level {
            blocks: HashSet::new(),
            tanks: [None, None, None, None],
        }
    }

    pub fn draw(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<()> {
        queue!(
            stdout,
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::Black)
        )?;

        for x in x..x + width {
            for y in y..y + height {
                let graphics = if x % 2 == 1 { " │" } else { "  " };

                let horizontal_line = y % 2 == 1;

                if horizontal_line {
                    queue!(stdout, SetAttribute(Attribute::Underlined))?;
                }

                queue!(
                    stdout,
                    cursor::MoveTo(x * 2 + horizontal_margin, y + vertical_margin),
                    Print(graphics)
                )?;

                if horizontal_line {
                    queue!(stdout, SetAttribute(Attribute::Reset))?;
                }
            }
        }

        for (tank, player_number) in self.tanks.iter().zip(0..4 as u8) {
            if let Some(tank) = tank {
                if tank.x >= x && tank.x < x + width && tank.y >= y && tank.y < y + height {
                    tank.draw(stdout, horizontal_margin, vertical_margin, player_number)?;
                }
            }
        }

        let filtered_blocks = self.blocks.iter().filter(|block| {
            block.x >= x && block.x < x + width && block.y >= y && block.y < y + height
        });

        for block in filtered_blocks {
            block.draw(stdout, horizontal_margin, vertical_margin)?;
        }

        Ok(())
    }
}
