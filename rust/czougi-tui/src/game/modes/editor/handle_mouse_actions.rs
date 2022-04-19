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
        mouse_state: &MouseState,
        mouse_map_x: u16,
        mouse_map_y: u16,
    ) {
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
                        for (t, p) in self.level.tanks.iter().zip(0..=3) {
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
                        self.level.tanks[player_number as usize] = Some(Tank {
                            x: mouse_map_x,
                            y: mouse_map_y,
                            direction,
                        });
                    }
                } else if let Some((first_selection_corner_x, first_selection_corner_y)) =
                    self.first_selection_corner
                {
                    if mouse_map_x < LEVEL_SIZE && mouse_map_y < LEVEL_SIZE {
                        match self.tool {
                            Tool::SmallBlock(block_type, block_variant) => {
                                let position_x_iterator = {
                                    let (first, last) = if first_selection_corner_x < mouse_map_x {
                                        (first_selection_corner_x, mouse_map_x)
                                    } else {
                                        (mouse_map_x, first_selection_corner_x)
                                    };
                                    first..=last
                                };

                                let position_y_iterator = {
                                    let (first, last) = if first_selection_corner_y < mouse_map_y {
                                        (first_selection_corner_y, mouse_map_y)
                                    } else {
                                        (mouse_map_y, first_selection_corner_y)
                                    };
                                    first..=last
                                };

                                for x in position_x_iterator {
                                    for y in position_y_iterator.clone() {
                                        self.level.blocks.insert(Block {
                                            x,
                                            y,
                                            block_type,
                                            block_variant,
                                        });
                                    }
                                }
                            }
                            Tool::FullBlock(block_type) => {
                                let (left_top_x, right_bottom_x) =
                                    if first_selection_corner_x < mouse_map_x {
                                        (first_selection_corner_x, mouse_map_x)
                                    } else {
                                        (mouse_map_x, first_selection_corner_x)
                                    };

                                let (left_top_y, right_bottom_y) =
                                    if first_selection_corner_y < mouse_map_y {
                                        (first_selection_corner_y, mouse_map_y)
                                    } else {
                                        (mouse_map_y, first_selection_corner_y)
                                    };

                                for x in left_top_x..=right_bottom_x {
                                    'label: for y in left_top_y..=right_bottom_y {
                                        for tank in self.level.tanks.iter().flatten() {
                                            if x >= tank.x
                                                && x <= tank.x + 3
                                                && y >= tank.y
                                                && y <= tank.y + 3
                                            {
                                                continue 'label;
                                            }
                                        }

                                        let is_left = (x - left_top_x) % 2 == 0;
                                        let is_top = (y - left_top_y) % 2 == 0;

                                        let block_variant = match (is_left, is_top) {
                                            (true, true) => BlockVariant::LeftTop,
                                            (true, false) => BlockVariant::LeftBottom,
                                            (false, true) => BlockVariant::RightTop,
                                            (false, false) => BlockVariant::RightBottom,
                                        };

                                        self.level.blocks.insert(Block {
                                            x,
                                            y,
                                            block_type,
                                            block_variant,
                                        });
                                    }
                                }
                            }
                            Tool::Eraser => {
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

                                self.level.tanks.iter_mut().for_each(|t| {
                                    if let Some(tank) = t {
                                        if !(tank.x + 3 < left_top_x
                                            || tank.x > right_bottom_x
                                            || tank.y + 3 < left_top_y
                                            || tank.y > right_bottom_y)
                                        {
                                            *t = None;
                                        }
                                    }
                                });
                            }
                            _ => unreachable!(),
                        };
                    }
                }

                self.first_selection_corner = None;
            }
            _ => {}
        }
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
