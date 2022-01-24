use crate::game::{
    drawing_utils::{
        block::{
            draw_brick_block, draw_concrete_block, draw_leaves_block, draw_water_block,
            BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR, CONCRETE_BACKGROUND_COLOR,
            CONCRETE_FOREGROUND_COLOR, LEAVES_BACKGROUND_COLOR, LEAVES_FOREGROUND_COLOR,
            WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR,
        },
        tank::draw_tank,
    },
    input::ScrollState,
    level::Direction,
};
use crossterm::{
    queue,
    style::{SetBackgroundColor, SetForegroundColor},
    Result,
};
use std::io::Stdout;

pub(super) enum Tool {
    Brick,
    Concrete,
    Water,
    Leaves,
    Tank(u8, Direction), // Player number, direction of tank
    Eraser,
}

impl Tool {
    pub(super) fn change_tank_direction(&mut self, scroll: &ScrollState) {
        if let Tool::Tank(_, direction) = self {
            match scroll {
                ScrollState::Up => {
                    *direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Up,
                    };
                }
                ScrollState::Down => {
                    *direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                }
                _ => {}
            }
        }
    }

    pub(super) fn draw_tool(
        &self,
        stdout: &mut Stdout,
        mouse_x: u16,
        mouse_y: u16,
        mouse_map_x: u16,
        mouse_map_y: u16,
    ) -> Result<()> {
        if let Tool::Tank(player_number, direction) = &self {
            if mouse_map_x <= 46 && mouse_map_y <= 46 {
                draw_tank(stdout, mouse_x, mouse_y, *player_number, direction)?;
            }
            return Ok(());
        }

        if mouse_map_x <= 48 && mouse_map_y <= 48 {
            match self {
                Tool::Brick => {
                    queue!(
                        stdout,
                        SetForegroundColor(BRICK_FOREGROUND_COLOR),
                        SetBackgroundColor(BRICK_BACKGROUND_COLOR),
                    )?;
                    draw_brick_block(stdout, mouse_x, mouse_y)?;
                }
                Tool::Concrete => {
                    queue!(
                        stdout,
                        SetForegroundColor(CONCRETE_FOREGROUND_COLOR),
                        SetBackgroundColor(CONCRETE_BACKGROUND_COLOR),
                    )?;
                    draw_concrete_block(stdout, mouse_x, mouse_y)?;
                }
                Tool::Water => {
                    queue!(
                        stdout,
                        SetForegroundColor(WATER_FOREGROUND_COLOR),
                        SetBackgroundColor(WATER_BACKGROUND_COLOR),
                    )?;
                    draw_water_block(stdout, mouse_x, mouse_y)?;
                }
                Tool::Leaves => {
                    queue!(
                        stdout,
                        SetForegroundColor(LEAVES_FOREGROUND_COLOR),
                        SetBackgroundColor(LEAVES_BACKGROUND_COLOR),
                    )?;
                    draw_leaves_block(stdout, mouse_x, mouse_y)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
