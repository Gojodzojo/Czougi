use super::{tool::Tool, Editor};
use crate::game::{
    input::{ButtonState, MouseState},
    level::{Block, Direction, Tank},
};

impl Editor {
    pub(super) fn handle_map_mouse_actions(
        &mut self,
        mouse_state: &MouseState,
        mouse_map_x: u16,
        mouse_map_y: u16,
    ) {
        if matches!(mouse_state.left_button, ButtonState::GettingReleased) {
            if let Tool::Tank(player_number, direction) = self.tool {
                if mouse_map_x <= 46 && mouse_map_y <= 46 {
                    self.level.tanks[player_number as usize] = Some(Tank {
                        x: mouse_map_x,
                        y: mouse_map_y,
                        direction,
                    });
                }
            } else if mouse_map_x <= 48 && mouse_map_y <= 48 {
                match self.tool {
                    Tool::Brick => {
                        self.level.bricks.push(Block {
                            x: mouse_map_x,
                            y: mouse_map_y,
                        });
                    }
                    Tool::Concrete => {
                        self.level.concretes.push(Block {
                            x: mouse_map_x,
                            y: mouse_map_y,
                        });
                    }
                    Tool::Water => {
                        self.level.waters.push(Block {
                            x: mouse_map_x,
                            y: mouse_map_y,
                        });
                    }
                    Tool::Leaves => {
                        self.level.leaves.push(Block {
                            x: mouse_map_x,
                            y: mouse_map_y,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    pub(super) fn handle_sidebar_mouse_actions(
        &mut self,
        mouse_state: &MouseState,
        horizontal_margin: u16,
        vertical_margin: u16,
    ) {
        if matches!(mouse_state.left_button, ButtonState::GettingReleased) {
            if mouse_state.is_hovered(horizontal_margin + 102, vertical_margin + 10, 8, 4) {
                self.tool = Tool::Brick;
            } else if mouse_state.is_hovered(horizontal_margin + 112, vertical_margin + 10, 8, 4) {
                self.tool = Tool::Concrete;
            } else if mouse_state.is_hovered(horizontal_margin + 102, vertical_margin + 15, 8, 4) {
                self.tool = Tool::Water;
            } else if mouse_state.is_hovered(horizontal_margin + 112, vertical_margin + 15, 8, 4) {
                self.tool = Tool::Leaves;
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
