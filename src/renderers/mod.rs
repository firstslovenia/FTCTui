use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Clear, Padding, Paragraph},
};
use styles::{TEXT_COLOR, block_style, selected_block_style};

use crate::{
    App,
    app::{
        ACTIVE_OPMODE_BLOCK_ID, AUTO_BLOCK_ID, AppMode, DEBUG_BLOCK_ID, GAMEPADS_BLOCK_ID,
        ROBOT_BLOCK_ID, TELEOP_BLOCK_ID,
    },
};

pub mod debug;
pub mod gamepads;
pub mod opmode;
pub mod popup;
pub mod robot;
pub mod styles;

impl App {
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    ///
    /// Should NOT be async
    pub fn render(&mut self, frame: &mut Frame<'_>) {
        let main_layout = match self.mode {
            AppMode::Normal => {
                Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(frame.area())
            }
            AppMode::InsertCommand => Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                Constraint::Length(1),
            ])
            .split(frame.area()),
        };

        let top_inner_layout = Layout::horizontal([
            Constraint::Ratio(2, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(2, 6),
        ])
        .split(main_layout[0]);

        let bottom_inner_layout =
            Layout::horizontal([Constraint::Ratio(3, 4), Constraint::Fill(1)])
                .split(main_layout[1]);

        let mut debug_block = Block::bordered()
            .title("Debug")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let mut teleop_block = Block::bordered()
            .title("Teleop opmodes")
            .border_style(block_style())
            .padding(Padding::new(1, 1, 1, 1));

        let mut auto_block = Block::bordered()
            .title("Auto opmodes")
            .border_style(block_style())
            .padding(Padding::new(1, 1, 1, 1));

        let mut robot_block = Block::bordered()
            .title("Robot status")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let mut active_opmode_block = Block::bordered()
            .title("Active opmode")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let mut gamepads_block = Block::bordered()
            .title("Gamepads")
            .border_style(block_style())
            .padding(Padding::new(1, 1, 1, 1));

        match self.selected_block {
            DEBUG_BLOCK_ID => {
                debug_block = debug_block
                    .border_style(selected_block_style())
                    .border_type(ratatui::widgets::BorderType::Thick)
            }
            TELEOP_BLOCK_ID => {
                teleop_block = teleop_block
                    .border_style(selected_block_style())
                    .border_type(ratatui::widgets::BorderType::Thick)
            }
            AUTO_BLOCK_ID => {
                auto_block = auto_block
                    .border_style(selected_block_style())
                    .border_type(ratatui::widgets::BorderType::Thick)
            }
            ROBOT_BLOCK_ID => {
                robot_block = robot_block
                    .border_style(selected_block_style())
                    .border_type(ratatui::widgets::BorderType::Thick)
            }
            ACTIVE_OPMODE_BLOCK_ID => {
                active_opmode_block = active_opmode_block
                    .border_style(selected_block_style())
                    .border_type(ratatui::widgets::BorderType::Thick)
            }
            GAMEPADS_BLOCK_ID => {
                gamepads_block = gamepads_block
                    .border_style(selected_block_style())
                    .border_type(ratatui::widgets::BorderType::Thick)
            }
            _ => {}
        }

        frame.render_widget(Clear, frame.area());
        frame.render_widget(&debug_block, top_inner_layout[0]);

        frame.render_widget(&robot_block, top_inner_layout[3]);

        frame.render_widget(&active_opmode_block, bottom_inner_layout[0]);
        frame.render_widget(&gamepads_block, bottom_inner_layout[1]);

        frame.render_widget(
            self.create_debug_paragraph(),
            debug_block.inner(top_inner_layout[0]),
        );

        self.render_teleop_list(frame, teleop_block, top_inner_layout[1]);
        self.render_auto_list(frame, auto_block, top_inner_layout[2]);

        frame.render_widget(
            self.create_robot_paragraph(),
            robot_block.inner(top_inner_layout[3]),
        );

        frame.render_widget(
            self.create_active_opmode_paragraph(),
            active_opmode_block.inner(bottom_inner_layout[0]),
        );

        frame.render_widget(
            self.create_gamepads_paragraph(),
            gamepads_block.inner(bottom_inner_layout[1]),
        );

        self.render_popup_if_any(frame);
        self.render_quickmenu_if_open(frame);

        // Render the vim-like command thingy
        if self.mode == AppMode::InsertCommand {
            // Slightly left-padded
            let command_paragraph =
                Paragraph::new(format!(":{}█", self.current_command)).fg(TEXT_COLOR);

            frame.render_widget(command_paragraph, main_layout[2]);
        }
    }
}
