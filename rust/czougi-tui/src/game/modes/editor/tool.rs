use crate::game::{
    input::ScrollState,
    level::{
        block::{BlockType, BlockVariant},
        tank::Direction,
    },
};

pub(super) enum Tool {
    SmallBlock(BlockType, BlockVariant),
    FullBlock(BlockType),
    Tank(u8, Direction), // Player number, direction of tank
    Eraser,
}

impl Tool {
    pub(super) fn handle_scroll(&mut self, scroll: &ScrollState) {
        if matches!(scroll, ScrollState::None) {
            return;
        }

        match self {
            Tool::SmallBlock(block_type, block_variant) => match scroll {
                ScrollState::Up => match block_variant {
                    BlockVariant::LeftTop => *self = Tool::FullBlock(*block_type),
                    BlockVariant::RightTop => *block_variant = BlockVariant::RightBottom,
                    BlockVariant::LeftBottom => *block_variant = BlockVariant::LeftTop,
                    BlockVariant::RightBottom => *block_variant = BlockVariant::RightTop,
                },
                ScrollState::Down => match block_variant {
                    BlockVariant::LeftTop => *block_variant = BlockVariant::RightTop,
                    BlockVariant::RightTop => *block_variant = BlockVariant::LeftBottom,
                    BlockVariant::LeftBottom => *block_variant = BlockVariant::RightBottom,
                    BlockVariant::RightBottom => *self = Tool::FullBlock(*block_type),
                },
                _ => {}
            },
            Tool::FullBlock(block_type) => match scroll {
                ScrollState::Up => *self = Tool::SmallBlock(*block_type, BlockVariant::RightBottom),
                ScrollState::Down => *self = Tool::SmallBlock(*block_type, BlockVariant::LeftTop),
                _ => {}
            },
            Tool::Tank(_, direction) => {
                *direction = match scroll {
                    ScrollState::Up => match direction {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Up,
                    },
                    ScrollState::Down => match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    },
                    _ => unreachable!(),
                }
            }
            Tool::Eraser => {}
        }
    }

    // pub(super) fn draw_tool(
    //     &self,
    //     stdout: &mut Stdout,
    //     first_selection_corner: &Option<(u16, u16)>,
    //     mouse_x: u16,
    //     mouse_y: u16,
    //     mouse_map_x: u16,
    //     mouse_map_y: u16,
    //     horizontal_margin: u16,
    //     vertical_margin: u16,
    // ) -> Result<()> {
    //     if let Tool::Tank(player_number, direction) = &self {
    //         if mouse_map_x <= 46 && mouse_map_y <= 46 {
    //             queue!(stdout, SetBackgroundColor(Color::Black))?;
    //             draw_tank(stdout, mouse_x, mouse_y, *player_number, direction)?;
    //         }
    //         return Ok(());
    //     }

    //     if mouse_map_x <= 48 && mouse_map_y <= 48 {
    //         type DrawFunctionPointer = fn(&mut Stdout, u16, u16) -> Result<()>;
    //         let (foreground, background, draw): (Color, Color, DrawFunctionPointer) = match self {
    //             Tool::Brick => (
    //                 BRICK_FOREGROUND_COLOR,
    //                 BRICK_BACKGROUND_COLOR,
    //                 draw_brick_block,
    //             ),
    //             Tool::Concrete => (
    //                 CONCRETE_FOREGROUND_COLOR,
    //                 CONCRETE_BACKGROUND_COLOR,
    //                 draw_concrete_block,
    //             ),
    //             Tool::Water => (
    //                 WATER_FOREGROUND_COLOR,
    //                 WATER_BACKGROUND_COLOR,
    //                 draw_water_block,
    //             ),
    //             Tool::Leaves => (
    //                 LEAVES_FOREGROUND_COLOR,
    //                 LEAVES_BACKGROUND_COLOR,
    //                 draw_leaves_block,
    //             ),
    //             Tool::Eraser => (Color::Red, Color::Red, draw_full_block),
    //             _ => unreachable!(),
    //         };

    //         let (first_selection_corner_x, first_selection_corner_y) = match first_selection_corner
    //         {
    //             Some((x, y)) => (x * 2 + horizontal_margin, y + vertical_margin),
    //             None => (mouse_x, mouse_y),
    //         };

    //         let position_x_iterator = {
    //             let (first, last) = if first_selection_corner_x < mouse_x {
    //                 (first_selection_corner_x, mouse_x)
    //             } else {
    //                 (mouse_x, first_selection_corner_x)
    //             };
    //             (first..=last).step_by(4).chain(last..=last)
    //         };

    //         let position_y_iterator = {
    //             let (first, last) = if first_selection_corner_y < mouse_y {
    //                 (first_selection_corner_y, mouse_y)
    //             } else {
    //                 (mouse_y, first_selection_corner_y)
    //             };
    //             (first..=last).step_by(2).chain(last..=last)
    //         };

    //         queue!(
    //             stdout,
    //             SetBackgroundColor(background),
    //             SetForegroundColor(foreground)
    //         )?;

    //         for x in position_x_iterator {
    //             for y in position_y_iterator.clone() {
    //                 draw(stdout, x, y)?;
    //             }
    //         }
    //     }

    //     Ok(())
    // }
}
