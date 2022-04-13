pub mod block;
pub mod tank;

use self::{block::Block, tank::Tank};
use crossterm::Result;
use std::io::Stdout;
pub struct Level {
    pub blocks: Vec<Block>,
    pub tanks: [Option<Tank>; 4],
}

impl Level {
    pub fn new() -> Self {
        Level {
            blocks: vec![],
            tanks: [None, None, None, None],
        }
    }

    pub fn draw_blocks(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        for block in self.blocks.iter() {
            block.draw(stdout, horizontal_margin, vertical_margin)?;
        }

        Ok(())
    }

    pub fn draw_tanks(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        for (tank, player_number) in self.tanks.iter().zip(0..=3 as u8) {
            if let Some(tank) = tank {
                tank.draw(stdout, horizontal_margin, vertical_margin, player_number)?;
            }
        }

        Ok(())
    }

    // pub fn erase_element(&mut self, x: u16, y: u16) {
    //     let filter = |block: &Block| {
    //         !((x == block.x + 1 || x == block.x) && (y == block.y + 1 || y == block.y))
    //     };

    //     self.bricks.retain(filter);
    //     self.concretes.retain(filter);
    //     self.waters.retain(filter);
    //     self.leaves.retain(filter);

    //     self.tanks.iter_mut().for_each(|t| {
    //         if let Some(tank) = t {
    //             if x >= tank.x && x <= tank.x + 3 && y >= tank.y && y <= tank.y + 3 {
    //                 *t = None;
    //             }
    //         }
    //     });
    // }
}
