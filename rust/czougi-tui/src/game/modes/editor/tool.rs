use crate::game::{
    drawing_utils::{
        block::{
            draw_brick_block, draw_concrete_block, draw_full_block, draw_leaves_block,
            draw_water_block, BRICK_BACKGROUND_COLOR, BRICK_FOREGROUND_COLOR,
            CONCRETE_BACKGROUND_COLOR, CONCRETE_FOREGROUND_COLOR, LEAVES_BACKGROUND_COLOR,
            LEAVES_FOREGROUND_COLOR, WATER_BACKGROUND_COLOR, WATER_FOREGROUND_COLOR,
        },
        tank::draw_tank,
    },
    input::ScrollState,
    level::Direction,
};
use crossterm::{
    queue,
    style::{Color, SetBackgroundColor, SetForegroundColor},
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
        first_selection_corner: &Option<(u16, u16)>,
        mouse_x: u16,
        mouse_y: u16,
        mouse_map_x: u16,
        mouse_map_y: u16,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) -> Result<()> {
        if let Tool::Tank(player_number, direction) = &self {
            if mouse_map_x <= 46 && mouse_map_y <= 46 {
                queue!(stdout, SetBackgroundColor(Color::Black))?;
                draw_tank(stdout, mouse_x, mouse_y, *player_number, direction)?;
            }
            return Ok(());
        }

        if mouse_map_x <= 48 && mouse_map_y <= 48 {
            type DrawFunctionPointer = fn(&mut Stdout, u16, u16) -> Result<()>;
            let (foreground, background, draw): (Color, Color, DrawFunctionPointer) = match self {
                Tool::Brick => (
                    BRICK_FOREGROUND_COLOR,
                    BRICK_BACKGROUND_COLOR,
                    draw_brick_block,
                ),
                Tool::Concrete => (
                    CONCRETE_FOREGROUND_COLOR,
                    CONCRETE_BACKGROUND_COLOR,
                    draw_concrete_block,
                ),
                Tool::Water => (
                    WATER_FOREGROUND_COLOR,
                    WATER_BACKGROUND_COLOR,
                    draw_water_block,
                ),
                Tool::Leaves => (
                    LEAVES_FOREGROUND_COLOR,
                    LEAVES_BACKGROUND_COLOR,
                    draw_leaves_block,
                ),
                Tool::Eraser => (Color::Red, Color::Red, draw_full_block),
                _ => unreachable!(),
            };

            let (first_selection_corner_x, first_selection_corner_y) = match first_selection_corner
            {
                Some((x, y)) => (x * 2 + horizontal_margin, y + vertical_margin),
                None => (mouse_x, mouse_y),
            };

            let position_x_iterator = {
                let (first, last) = if first_selection_corner_x < mouse_x {
                    (first_selection_corner_x, mouse_x)
                } else {
                    (mouse_x, first_selection_corner_x)
                };
                (first..=last).step_by(4).chain(last..=last)
            };

            let position_y_iterator = {
                let (first, last) = if first_selection_corner_y < mouse_y {
                    (first_selection_corner_y, mouse_y)
                } else {
                    (mouse_y, first_selection_corner_y)
                };
                (first..=last).step_by(2).chain(last..=last)
            };

            queue!(
                stdout,
                SetBackgroundColor(background),
                SetForegroundColor(foreground)
            )?;

            for x in position_x_iterator {
                for y in position_y_iterator.clone() {
                    draw(stdout, x, y)?;
                }
            }
        }

        Ok(())
    }
}
