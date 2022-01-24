use std::io::Stdout;

use crate::game::drawing_utils::{
    block::{
        draw_brick_block, draw_concrete_block, draw_leaves_block, draw_water_block,
        BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, CONCRETE_BACKGROUND_COLOR,
        CONCRETE_FOREGROUND_COLOR, LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR,
        WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR,
    },
    tank::draw_tank,
};
use crossterm::{
    queue,
    style::{SetBackgroundColor, SetForegroundColor},
    Result,
};
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

    pub fn draw_tanks(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        for (tank, player_number) in self.tanks.iter().zip(0..=3 as u8) {
            if let Some(Tank { x, y, direction }) = tank {
                draw_tank(
                    stdout,
                    *x * 2 + horizontal_margin,
                    *y + vertical_margin,
                    player_number,
                    direction,
                )?;
            }
        }

        Ok(())
    }

    pub fn draw_bricks(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        queue!(
            stdout,
            SetBackgroundColor(BRICK_BACKGROUND_COLOR),
            SetForegroundColor(BRICK_FOREGROUND_COLOR),
        )?;
        for brick in self.bricks.iter() {
            draw_brick_block(
                stdout,
                brick.x * 2 + horizontal_margin,
                brick.y + vertical_margin,
            )?;
        }

        Ok(())
    }

    pub fn draw_concretes(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        queue!(
            stdout,
            SetBackgroundColor(CONCRETE_BACKGROUND_COLOR),
            SetForegroundColor(CONCRETE_FOREGROUND_COLOR),
        )?;
        for concrete in self.concretes.iter() {
            draw_concrete_block(
                stdout,
                concrete.x * 2 + horizontal_margin,
                concrete.y + vertical_margin,
            )?;
        }

        Ok(())
    }

    pub fn draw_waters(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        queue!(
            stdout,
            SetBackgroundColor(WATER_BACKGROUND_COLOR),
            SetForegroundColor(WATER_FOREGROUND_COLOR),
        )?;
        for water in self.waters.iter() {
            draw_water_block(
                stdout,
                water.x * 2 + horizontal_margin,
                water.y + vertical_margin,
            )?;
        }

        Ok(())
    }

    pub fn draw_leaves(
        &self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        queue!(
            stdout,
            SetBackgroundColor(LEAVES_BACKGROUND_COLOR),
            SetForegroundColor(LEAVES_FOREGROUND_COLOR),
        )?;
        for leaf in self.leaves.iter() {
            draw_leaves_block(
                stdout,
                leaf.x * 2 + horizontal_margin,
                leaf.y + vertical_margin,
            )?;
        }

        Ok(())
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
