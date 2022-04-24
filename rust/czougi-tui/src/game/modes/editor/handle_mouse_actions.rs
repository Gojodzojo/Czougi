use std::io::Stdout;

use crossterm::Result;

use super::{tool::Tool, Editor};
use crate::game::{
    input::{ButtonState, MouseState},
    level::{
        block::{Block, BlockType, BlockVariant},
        tank::{Direction, Tank, TANK_SIZE},
        LEVEL_MAP_WIDTH, LEVEL_SIZE,
    },
};

impl Editor {
    pub(super) fn handle_map_mouse_actions(
        &mut self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
        mouse_state: &MouseState,
        mouse_map_x: u16,
        mouse_map_y: u16,
    ) -> Result<()> {
        match mouse_state.left_button {
            ButtonState::GettingPressed => {
                if !matches!(self.tool, Tool::Tank(_, _))
                    && mouse_map_x < LEVEL_SIZE
                    && mouse_map_y < LEVEL_SIZE
                {
                    self.first_selection_corner = Some((mouse_map_x, mouse_map_y));
                }
            }
            ButtonState::GettingReleased => {
                if let Tool::Tank(player_number, direction) = self.tool {
                    self.place_tank(
                        stdout,
                        horizontal_margin,
                        vertical_margin,
                        mouse_map_x,
                        mouse_map_y,
                        player_number,
                        direction,
                    )?;
                } else if let Some((first_selection_corner_x, first_selection_corner_y)) =
                    self.first_selection_corner
                {
                    if mouse_map_x < LEVEL_SIZE && mouse_map_y < LEVEL_SIZE {
                        let (left_top_x, right_bottom_x) = if first_selection_corner_x < mouse_map_x
                        {
                            (first_selection_corner_x, mouse_map_x)
                        } else {
                            (mouse_map_x, first_selection_corner_x)
                        };

                        let (left_top_y, right_bottom_y) = if first_selection_corner_y < mouse_map_y
                        {
                            (first_selection_corner_y, mouse_map_y)
                        } else {
                            (mouse_map_y, first_selection_corner_y)
                        };

                        match self.tool {
                            Tool::SmallBlock(block_type, block_variant) => self.place_block(
                                left_top_x,
                                left_top_y,
                                right_bottom_x,
                                right_bottom_y,
                                block_type,
                                |_, _| block_variant,
                            ),
                            Tool::FullBlock(block_type) => self.place_block(
                                left_top_x,
                                left_top_y,
                                right_bottom_x,
                                right_bottom_y,
                                block_type,
                                |x, y| {
                                    let is_left = (x - left_top_x) % 2 == 0;
                                    let is_top = (y - left_top_y) % 2 == 0;

                                    match (is_left, is_top) {
                                        (true, true) => BlockVariant::LeftTop,
                                        (true, false) => BlockVariant::LeftBottom,
                                        (false, true) => BlockVariant::RightTop,
                                        (false, false) => BlockVariant::RightBottom,
                                    }
                                },
                            ),
                            Tool::Eraser => self.erase(
                                stdout,
                                horizontal_margin,
                                vertical_margin,
                                mouse_map_x,
                                mouse_map_y,
                                first_selection_corner_x,
                                first_selection_corner_y,
                            )?,
                            _ => unreachable!(),
                        };

                        self.level.draw(
                            stdout,
                            horizontal_margin,
                            vertical_margin,
                            left_top_x,
                            left_top_y,
                            right_bottom_x - left_top_x + 1,
                            right_bottom_y - left_top_y + 1,
                        )?;
                    }
                }

                self.first_selection_corner = None;
            }
            _ => {}
        }

        Ok(())
    }

    fn place_tank(
        &mut self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
        mouse_map_x: u16,
        mouse_map_y: u16,
        player_number: u8,
        direction: Direction,
    ) -> Result<()> {
        let mut is_overlapping = false;

        for block in self.level.blocks.iter() {
            if block.x >= mouse_map_x
                && block.x < mouse_map_x + TANK_SIZE
                && block.y >= mouse_map_y
                && block.y < mouse_map_y + TANK_SIZE
            {
                is_overlapping = true;
                break;
            }
        }

        if !is_overlapping {
            for (t, p) in self.level.tanks.iter().zip(0..4) {
                if p != player_number {
                    if let Some(tank) = t {
                        if tank.x + 3 >= mouse_map_x
                            && tank.x <= mouse_map_x + 3
                            && tank.y + 3 >= mouse_map_y
                            && tank.y <= mouse_map_y + 3
                        {
                            is_overlapping = true;
                            break;
                        }
                    }
                }
            }
        }

        if !is_overlapping
            && mouse_map_x <= LEVEL_SIZE - TANK_SIZE
            && mouse_map_y <= LEVEL_SIZE - TANK_SIZE
        {
            let previous_position = if let Some(tank) = &self.level.tanks[player_number as usize] {
                Some((tank.x, tank.y))
            } else {
                None
            };

            self.level.tanks[player_number as usize] = Some(Tank {
                x: mouse_map_x,
                y: mouse_map_y,
                direction,
            });

            // Remove previous tank graphics
            if let Some((previous_x, previous_y)) = previous_position {
                self.level.draw(
                    stdout,
                    horizontal_margin,
                    vertical_margin,
                    previous_x,
                    previous_y,
                    TANK_SIZE,
                    TANK_SIZE,
                )?;
            }

            // Draw new tank graphics
            self.level.draw(
                stdout,
                horizontal_margin,
                vertical_margin,
                mouse_map_x,
                mouse_map_y,
                TANK_SIZE,
                TANK_SIZE,
            )?;
        }

        Ok(())
    }

    fn place_block(
        &mut self,
        left_top_x: u16,
        left_top_y: u16,
        right_bottom_x: u16,
        right_bottom_y: u16,
        block_type: BlockType,
        block_variant_getter: impl Fn(u16, u16) -> BlockVariant,
    ) {
        for x in left_top_x..right_bottom_x + 1{
            'label: for y in left_top_y..right_bottom_y + 1 {
                for tank in self.level.tanks.iter().flatten() {
                    if x >= tank.x && x <= tank.x + 3 && y >= tank.y && y <= tank.y + 3 {
                        continue 'label;
                    }
                }

                let block_variant = block_variant_getter(x, y);

                self.level.blocks.insert(Block {
                    x,
                    y,
                    block_type,
                    block_variant,
                });
            }
        }
    }

    fn erase(
        &mut self,
        stdout: &mut Stdout,
        horizontal_margin: u16,
        vertical_margin: u16,
        mouse_map_x: u16,
        mouse_map_y: u16,
        first_selection_corner_x: u16,
        first_selection_corner_y: u16,
    ) -> Result<()> {
        let (left_top_x, right_bottom_x) = {
            if first_selection_corner_x < mouse_map_x {
                (first_selection_corner_x, mouse_map_x)
            } else {
                (mouse_map_x, first_selection_corner_x)
            }
        };

        let (left_top_y, right_bottom_y) = {
            if first_selection_corner_y < mouse_map_y {
                (first_selection_corner_y, mouse_map_y)
            } else {
                (mouse_map_y, first_selection_corner_y)
            }
        };

        self.level.blocks.retain(|block| {
            !(block.x >= left_top_x
                && block.x <= right_bottom_x
                && block.y >= left_top_y
                && block.y <= right_bottom_y)
        });

        for i in 0..self.level.tanks.len() {
            if let Some(tank) = &self.level.tanks[i] {
                if !(tank.x + 3 < left_top_x
                    || tank.x > right_bottom_x
                    || tank.y + 3 < left_top_y
                    || tank.y > right_bottom_y)
                {
                    let Tank { x, y, .. } = *tank;
                    self.level.tanks[i] = None;
                    self.level.draw(
                        stdout,
                        horizontal_margin,
                        vertical_margin,
                        x,
                        y,
                        TANK_SIZE,
                        TANK_SIZE,
                    )?;
                }
            }
        }

        Ok(())
    }

    pub(super) fn handle_sidebar_mouse_actions(
        &mut self,
        mouse_state: &MouseState,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) {
        if !matches!(self.first_selection_corner, None) {
            self.first_selection_corner = None;
        }

        if matches!(mouse_state.left_button, ButtonState::GettingReleased) {
            if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 3,
                vertical_margin + 10,
                8,
                4,
            ) {
                self.tool = Tool::FullBlock(BlockType::Brick);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 14,
                vertical_margin + 10,
                8,
                4,
            ) {
                self.tool = Tool::FullBlock(BlockType::Concrete);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 3,
                vertical_margin + 15,
                8,
                4,
            ) {
                self.tool = Tool::FullBlock(BlockType::Water);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 14,
                vertical_margin + 15,
                8,
                4,
            ) {
                self.tool = Tool::FullBlock(BlockType::Leaves);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 3,
                vertical_margin + 20,
                8,
                4,
            ) {
                self.tool = Tool::Tank(0, Direction::Up);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 14,
                vertical_margin + 20,
                8,
                4,
            ) {
                self.tool = Tool::Tank(1, Direction::Up);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 3,
                vertical_margin + 25,
                8,
                4,
            ) {
                self.tool = Tool::Tank(2, Direction::Up);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 13,
                vertical_margin + 25,
                8,
                4,
            ) {
                self.tool = Tool::Tank(3, Direction::Up);
            } else if mouse_state.is_hovered(
                horizontal_margin + LEVEL_MAP_WIDTH + 9,
                vertical_margin + 30,
                8,
                4,
            ) {
                self.tool = Tool::Eraser;
            }
        }
    }
}
