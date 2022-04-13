use super::{tool::Tool, Editor};
use crate::game::{
    input::{ButtonState, MouseState},
    level::{
        block::{Block, BlockType, BlockVariant},
        tank::{Direction, Tank},
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
                if !matches!(self.tool, Tool::Tank(_, _)) && mouse_map_x <= 48 && mouse_map_y <= 48
                {
                    self.first_selection_corner = Some((mouse_map_x, mouse_map_y));
                }
            }
            ButtonState::GettingReleased => {
                if let Tool::Tank(player_number, direction) = self.tool {
                    if mouse_map_x <= 46 && mouse_map_y <= 46 {
                        self.level.tanks[player_number as usize] = Some(Tank {
                            x: mouse_map_x,
                            y: mouse_map_y,
                            direction,
                        });
                    }
                } else if let Some((first_selection_corner_x, first_selection_corner_y)) =
                    self.first_selection_corner
                {
                    if mouse_map_x <= 48 && mouse_map_y <= 48 {
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
                                        self.level.blocks.push(Block {
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
                                    for y in left_top_y..=right_bottom_y {
                                        let is_left = (x - left_top_x) % 2 == 0;
                                        let is_top = (y - left_top_y) % 2 == 0;

                                        let block_variant = match (is_left, is_top) {
                                            (true, true) => BlockVariant::LeftTop,
                                            (true, false) => BlockVariant::LeftBottom,
                                            (false, true) => BlockVariant::RightTop,
                                            (false, false) => BlockVariant::RightBottom,
                                        };

                                        self.level.blocks.push(Block {
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
                                        if tank.x >= left_top_x
                                            && tank.x + 3 <= right_bottom_x
                                            && tank.y >= left_top_y
                                            && tank.y + 3 <= right_bottom_y
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
            if mouse_state.is_hovered(horizontal_margin + 102, vertical_margin + 10, 8, 4) {
                self.tool = Tool::FullBlock(BlockType::Brick);
            } else if mouse_state.is_hovered(horizontal_margin + 112, vertical_margin + 10, 8, 4) {
                self.tool = Tool::FullBlock(BlockType::Concrete);
            } else if mouse_state.is_hovered(horizontal_margin + 102, vertical_margin + 15, 8, 4) {
                self.tool = Tool::FullBlock(BlockType::Water);
            } else if mouse_state.is_hovered(horizontal_margin + 112, vertical_margin + 15, 8, 4) {
                self.tool = Tool::FullBlock(BlockType::Leaves);
            } else if mouse_state.is_hovered(horizontal_margin + 102, vertical_margin + 20, 8, 4) {
                self.tool = Tool::Tank(0, Direction::Up);
            } else if mouse_state.is_hovered(horizontal_margin + 112, vertical_margin + 20, 8, 4) {
                self.tool = Tool::Tank(1, Direction::Up);
            } else if mouse_state.is_hovered(horizontal_margin + 102, vertical_margin + 25, 8, 4) {
                self.tool = Tool::Tank(2, Direction::Up);
            } else if mouse_state.is_hovered(horizontal_margin + 112, vertical_margin + 25, 8, 4) {
                self.tool = Tool::Tank(3, Direction::Up);
            } else if mouse_state.is_hovered(horizontal_margin + 107, vertical_margin + 30, 8, 4) {
                self.tool = Tool::Eraser;
            }
        }
    }
}
