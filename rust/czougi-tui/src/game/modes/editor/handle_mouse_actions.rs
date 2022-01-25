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
                        let mut action: Box<dyn FnMut(u16, u16) -> ()> = match self.tool {
                            Tool::Brick => {
                                Box::new(|x: u16, y: u16| self.level.bricks.push(Block { x, y }))
                            }
                            Tool::Concrete => {
                                Box::new(|x: u16, y: u16| self.level.concretes.push(Block { x, y }))
                            }
                            Tool::Water => {
                                Box::new(|x: u16, y: u16| self.level.waters.push(Block { x, y }))
                            }
                            Tool::Leaves => {
                                Box::new(|x: u16, y: u16| self.level.leaves.push(Block { x, y }))
                            }
                            Tool::Eraser => {
                                Box::new(|x: u16, y: u16| self.level.erase_element(x, y))
                            }
                            _ => unreachable!(),
                        };

                        let position_x_iterator = {
                            let (first, last) = if first_selection_corner_x < mouse_map_x {
                                (first_selection_corner_x, mouse_map_x)
                            } else {
                                (mouse_map_x, first_selection_corner_x)
                            };
                            (first..=last).step_by(2).chain(last..=last)
                        };

                        let position_y_iterator = {
                            let (first, last) = if first_selection_corner_y < mouse_map_y {
                                (first_selection_corner_y, mouse_map_y)
                            } else {
                                (mouse_map_y, first_selection_corner_y)
                            };
                            (first..=last).step_by(2).chain(last..=last)
                        };

                        for x in position_x_iterator {
                            for y in position_y_iterator.clone() {
                                action(x, y);
                            }
                        }

                        self.first_selection_corner = None;
                    }
                }
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
