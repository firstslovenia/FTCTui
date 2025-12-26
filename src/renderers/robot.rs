use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{
    app::App,
    ftc_proto::time_packet::RobotOpmodeState,
    renderers::styles::{
        ERROR_COLOR, MUTED_TEXT_COLOR, PRIMARY_COLOR_LIGHTER, SUCCESS_COLOR, TEXT_COLOR,
        WARNING_COLOR,
    },
};

impl App {
    /// Creates the robot text
    pub fn create_robot_paragraph(&mut self) -> Paragraph {
        let mut robot_text: Vec<Line> = Vec::new();

        let robot = futures::executor::block_on(self.robot.read());

        let Some(status) = &robot.active_opmode_state else {
            robot_text.push(Line::from(vec![Span::styled(
                "Waiting to receive status..",
                Style::new().fg(MUTED_TEXT_COLOR),
            )]));

            return Paragraph::new(robot_text).wrap(Wrap { trim: false });
        };

        robot_text.push(Line::from(Span::styled(
            format!(
                "Last update was {} second(s) ago",
                (std::time::Instant::now() - robot.last_battery_update).as_secs()
            ),
            Style::new().fg(MUTED_TEXT_COLOR),
        )));

        if let Some(battery_voltage) = robot.battery_voltage {
            let voltage_color = if battery_voltage <= 11.0 {
                ERROR_COLOR
            } else if battery_voltage < 13.0 {
                WARNING_COLOR
            } else {
                SUCCESS_COLOR
            };

            robot_text.push(Line::from(vec![
                Span::styled("Battery voltage: ", Style::new().fg(MUTED_TEXT_COLOR)),
                Span::styled(
                    format!("{:.2} V", battery_voltage),
                    Style::new().fg(voltage_color),
                ),
            ]));
        } else {
            robot_text.push(Line::from(vec![
                Span::styled("Battery voltage: ", Style::new().fg(MUTED_TEXT_COLOR)),
                Span::styled("Unknown".to_string(), Style::new().fg(MUTED_TEXT_COLOR)),
            ]));
        }

        if let Some(active_config) = &robot.active_configuration {
            robot_text.push(Line::from(vec![
                Span::styled("Active configuration: ", Style::new().fg(MUTED_TEXT_COLOR)),
                Span::styled(active_config.name.clone(), Style::new().fg(TEXT_COLOR)),
            ]));
        } else {
            robot_text.push(Line::from(vec![
                Span::styled("Active configuration: ", Style::new().fg(MUTED_TEXT_COLOR)),
                Span::styled("Unknown".to_string(), Style::new().fg(MUTED_TEXT_COLOR)),
            ]));
        }

        if robot.active_opmode == "$Stop$Robot$" {
            robot_text.push(Line::from(""));

            robot_text.push(Line::from(vec![Span::styled(
                "Robot is stopped.",
                Style::new().fg(TEXT_COLOR),
            )]));
        } else if !robot.active_opmode.is_empty() {
            robot_text.push(Line::from(""));

            robot_text.push(Line::from(vec![
                Span::styled("Active OpMode: ", Style::new().fg(MUTED_TEXT_COLOR)),
                Span::styled(robot.active_opmode.clone(), Style::new().fg(TEXT_COLOR)),
            ]));

            match status {
                RobotOpmodeState::Initialized | RobotOpmodeState::NotStarted => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Initialized", Style::new().fg(PRIMARY_COLOR_LIGHTER)),
                    ]));
                }
                RobotOpmodeState::Running => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Running", Style::new().fg(SUCCESS_COLOR)),
                    ]));
                }
                RobotOpmodeState::Stopped | RobotOpmodeState::EmergencyStopped => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Stopped", Style::new().fg(WARNING_COLOR)),
                    ]));
                }
                RobotOpmodeState::Unknown => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Unknown", Style::new().fg(MUTED_TEXT_COLOR)),
                    ]));
                }
            }
        }

        robot_text.push(Line::from(""));

        if let Some(warning_message) = &robot.warning_message {
            robot_text.push(Line::from(Span::styled(
                warning_message.clone(),
                Style::new().fg(WARNING_COLOR),
            )));
        }

        if let Some(error_message) = &robot.error_message {
            robot_text.push(Line::from(Span::styled(
                error_message.clone(),
                Style::new().fg(ERROR_COLOR),
            )));
        }

        Paragraph::new(robot_text).wrap(Wrap { trim: false })
    }
}
