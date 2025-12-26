use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
};

use crate::{
    app::{AUTO_BLOCK_ID, App, TELEOP_BLOCK_ID},
    ftc_proto::time_packet::RobotOpmodeState,
    renderers::styles::{
        MUTED_TEXT_COLOR, PRIMARY_COLOR_LIGHTER, SELECTED_BACKGROUND, SUCCESS_COLOR, TEXT_COLOR,
        WARNING_COLOR,
    },
};

impl App {
    /// Creates the teleop opmode list
    pub fn render_teleop_list(&mut self, frame: &mut Frame<'_>, block: Block<'_>, rect: Rect) {
        let mut items: Vec<ListItem> = Vec::new();

        let opmode_list = futures::executor::block_on(self.get_teleop_opmodes());

        let robot = futures::executor::block_on(self.robot.read());

        let active_opmode = robot.active_opmode.clone();
        let opmode_status = robot.active_opmode_state.clone();

        drop(robot);

        for i in 0..opmode_list.len() {
            let selected_opmode = opmode_list[i].clone();

            let selected = self.teleop_list_state.selected().unwrap_or_default() == i
                && self.selected_block == TELEOP_BLOCK_ID;

            let mut style = if selected {
                Style::new().fg(TEXT_COLOR).bg(SELECTED_BACKGROUND)
            } else {
                Style::new().fg(TEXT_COLOR)
            };

            if let Some(status) = opmode_status {
                if active_opmode == selected_opmode.name {
                    match status {
                        RobotOpmodeState::Initialized | RobotOpmodeState::NotStarted => {
                            style = style.fg(PRIMARY_COLOR_LIGHTER)
                        }
                        RobotOpmodeState::Running => style = style.fg(SUCCESS_COLOR),
                        RobotOpmodeState::Stopped | RobotOpmodeState::EmergencyStopped => {
                            style = style.fg(WARNING_COLOR)
                        }
                        RobotOpmodeState::Unknown => style = style.fg(TEXT_COLOR),
                    }
                }
            }

            items.push(ListItem::new(Span::styled(selected_opmode.name, style)));
        }

        frame.render_stateful_widget(
            List::new(items).block(block.clone()),
            rect,
            &mut self.teleop_list_state,
        );
    }

    /// Creates the auto opmode list
    pub fn render_auto_list(&mut self, frame: &mut Frame<'_>, block: Block<'_>, rect: Rect) {
        let mut items: Vec<ListItem> = Vec::new();

        let opmode_list = futures::executor::block_on(self.get_auto_opmodes());

        let robot = futures::executor::block_on(self.robot.read());

        let active_opmode = robot.active_opmode.clone();
        let opmode_status = robot.active_opmode_state.clone();

        drop(robot);

        for i in 0..opmode_list.len() {
            let selected_opmode = opmode_list[i].clone();

            let selected = self.auto_list_state.selected().unwrap_or_default() == i
                && self.selected_block == AUTO_BLOCK_ID;

            let mut style = if selected {
                Style::new().fg(TEXT_COLOR).bg(SELECTED_BACKGROUND)
            } else {
                Style::new().fg(TEXT_COLOR)
            };

            if let Some(status) = opmode_status {
                if active_opmode == selected_opmode.name {
                    match status {
                        RobotOpmodeState::Initialized | RobotOpmodeState::NotStarted => {
                            style = style.fg(PRIMARY_COLOR_LIGHTER)
                        }
                        RobotOpmodeState::Running => style = style.fg(SUCCESS_COLOR),
                        RobotOpmodeState::Stopped | RobotOpmodeState::EmergencyStopped => {
                            style = style.fg(WARNING_COLOR)
                        }
                        RobotOpmodeState::Unknown => style = style.fg(TEXT_COLOR),
                    }
                }
            }

            items.push(ListItem::new(Span::styled(selected_opmode.name, style)));
        }

        frame.render_stateful_widget(
            List::new(items).block(block.clone()),
            rect,
            &mut self.auto_list_state,
        );
    }

    /// Creates the active opmode / telemetry paragraph
    pub fn create_active_opmode_paragraph(&mut self) -> Paragraph {
        let mut text: Vec<Line> = Vec::new();

        let robot = futures::executor::block_on(self.robot.read());

        for line in robot.telemetry_list.clone() {
            let line = line;

            if line.contains(" : ") {
                let split = line.split(" : ").collect::<Vec<&str>>();

                let key = split[0].trim();
                let value = split[1].trim();

                text.push(Line::from(vec![
                    Span::styled(format!("{key} : "), Style::new().fg(MUTED_TEXT_COLOR)),
                    Span::styled(value.to_string(), Style::new().fg(TEXT_COLOR)),
                ]));
            } else {
                text.push(Line::from(Span::styled(line, Style::new().fg(TEXT_COLOR))));
            }
        }

        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .scroll((self.telemetry_display_scroll, 0))
    }
}
