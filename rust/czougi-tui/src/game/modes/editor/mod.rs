mod draw_sidebar;
mod handle_mouse_actions;
mod tool;

use self::tool::Tool;
use super::Mode;
use crate::game::level::block::BlockType;
use crate::game::level::{Level, LEVEL_MAP_WIDTH, LEVEL_SIZE};
use crate::game::{input::InputState, options::Options};
use crossterm::Result;
use draw_sidebar::draw_sidebar;
use std::io::Stdout;
use std::time::Duration;

pub struct Editor {
    tool: Tool,
    level: Level,
    first_selection_corner: Option<(u16, u16)>,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            tool: Tool::FullBlock(BlockType::Brick),
            level: Level::new(),
            first_selection_corner: None,
        }
    }
}

impl Mode for Editor {
    fn draw(
        &mut self,
        stdout: &mut Stdout,
        _delta_time: Duration,
        horizontal_margin: u16,
        vertical_margin: u16,
        refresh: bool,
        input_state: &InputState,
        _options: &Options,
    ) -> Result<Option<Box<dyn Mode>>> {
        let InputState { mouse_state, .. } = input_state;

        if refresh {
            draw_sidebar(stdout, horizontal_margin + LEVEL_MAP_WIDTH, vertical_margin)?;
        }

        self.level.draw(
            stdout,
            horizontal_margin,
            vertical_margin,
            0,
            0,
            LEVEL_SIZE,
            LEVEL_SIZE,
        )?;

        // Mouse is over the map
        if mouse_state.is_hovered(
            horizontal_margin,
            vertical_margin,
            LEVEL_MAP_WIDTH,
            LEVEL_SIZE,
        ) {
            let mouse_level_x = mouse_state.column - (mouse_state.column - horizontal_margin) % 2;
            let mouse_level_y = mouse_state.row;

            let mouse_map_x = (mouse_level_x - horizontal_margin) / 2;
            let mouse_map_y = mouse_level_y - vertical_margin;

            self.tool.handle_scroll(&mouse_state.scroll);

            // self.tool.draw_tool(
            //     stdout,
            //     &self.first_selection_corner,
            //     mouse_x,
            //     mouse_y,
            //     mouse_map_x,
            //     mouse_map_y,
            //     horizontal_margin,
            //     vertical_margin,
            // )?;

            self.handle_map_mouse_actions(mouse_state, mouse_map_x, mouse_map_y);
        }
        // Mouse is over the sidebar
        else {
            self.handle_sidebar_mouse_actions(mouse_state, horizontal_margin, vertical_margin);
        }

        Ok(None)
    }
}
