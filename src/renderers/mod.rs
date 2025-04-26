use color_eyre::owo_colors::OwoColorize;
use gilrs::GamepadId;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
};
use styles::{
    ERROR_COLOR, MUTED_TEXT_COLOR, PRIMARY_COLOR_LIGHTER, SELECTED_BACKGROUND, SUCCESS_COLOR,
    TEXT_COLOR, WARNING_COLOR, block_style, selected_block_style,
};

use crate::{
    App,
    app::{
        ACTIVE_OPMODE_BLOCK_ID, DEBUG_BLOCK_ID, GAMEPADS_BLOCK_ID, OP_MODES_BLOCK_ID,
        ROBOT_BLOCK_ID,
    },
    ftc_dashboard::robot_status::OpModeStatus,
};

pub mod styles;

impl App {
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    pub async fn render(&mut self, frame: &mut Frame<'_>) {
        let main_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());

        let top_inner_layout = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(main_layout[0]);

        let bottom_inner_layout =
            Layout::horizontal([Constraint::Ratio(3, 4), Constraint::Fill(1)])
                .split(main_layout[1]);

        let mut debug_block = Block::bordered().title("Debug").border_style(block_style());

        let mut op_modes_block = Block::bordered()
            .title("Op modes")
            .border_style(block_style())
            .padding(Padding::new(1, 1, 1, 1));

        let mut robot_block = Block::bordered()
            .title("Robot status")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let mut active_opmode_block = Block::bordered()
            .title("Active opmode")
            .border_style(block_style());

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
            OP_MODES_BLOCK_ID => {
                op_modes_block = op_modes_block
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

        frame.render_widget(&debug_block, top_inner_layout[0]);
        frame.render_widget(&op_modes_block, top_inner_layout[1]);
        frame.render_widget(&robot_block, top_inner_layout[2]);

        frame.render_widget(&active_opmode_block, bottom_inner_layout[0]);
        frame.render_widget(&gamepads_block, bottom_inner_layout[1]);

        frame.render_widget(
            self.create_gamepads_paragraph().await,
            gamepads_block.inner(bottom_inner_layout[1]),
        );

        frame.render_widget(
            self.create_robot_paragraph().await,
            robot_block.inner(top_inner_layout[2]),
        );

        frame.render_widget(
            self.create_opmode_list_paragraph().await,
            robot_block.inner(top_inner_layout[1]),
        );
    }

    /// Creates the gamepads debug text
    pub async fn create_gamepads_paragraph(&mut self) -> Paragraph {
        let gamepads = vec![
            self.gamepad_one.read().await.clone(),
            self.gamepad_two.read().await.clone(),
        ];

        let mut gamepads_text: Vec<Line> = Vec::new();

        for i in 0..gamepads.len() {
            let gamepad_option = gamepads[i].clone();

            if let Some(gamepad) = gamepad_option {
                let state = gamepad.last_state.clone();
                let gilrs_gamepad = self.gilrs.gamepad(gamepad.id);

                gamepads_text.push(Line::from(vec![
                    Span::styled(
                        gilrs_gamepad.name().to_string(),
                        Style::new().fg(SUCCESS_COLOR),
                    ),
                    Span::styled(":", Style::new().fg(MUTED_TEXT_COLOR)),
                ]));

                if state.left_stick_button {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  left stick: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!(
                                "{:.1}, {:.1} (pressed)",
                                state.left_stick_x, state.left_stick_y
                            ),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                } else {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  left stick: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!("{:.1}, {:.1}", state.left_stick_x, state.left_stick_y),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                }

                if state.right_stick_button {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  right stick: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!(
                                "{:.1}, {:.1} (pressed)",
                                state.right_stick_x, state.right_stick_y
                            ),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                } else {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  right stick: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!("{:.1}, {:.1}", state.right_stick_x, state.right_stick_y),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                }

                if state.left_bumper {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  left trigger: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!("{:.1} + bumper", state.left_trigger),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                } else {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  left trigger: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!("{:.1}", state.left_trigger),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                }

                if state.right_bumper {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  right trigger: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!("{:.1} + bumper", state.right_trigger),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                } else {
                    gamepads_text.push(Line::from(vec![
                        Span::styled("  right trigger: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(
                            format!("{:.1}", state.right_trigger),
                            Style::new().fg(TEXT_COLOR),
                        ),
                    ]));
                }

                let mut dpad_buttons_pressed = Vec::new();

                if state.dpad_up {
                    dpad_buttons_pressed.push("up")
                };
                if state.dpad_left {
                    dpad_buttons_pressed.push("left")
                };
                if state.dpad_right {
                    dpad_buttons_pressed.push("right")
                };
                if state.dpad_down {
                    dpad_buttons_pressed.push("down")
                };

                if !dpad_buttons_pressed.is_empty() {
                    let mut dpad_text = String::new();

                    for text in dpad_buttons_pressed {
                        dpad_text.push(' ');
                        dpad_text.push_str(&text);
                    }

                    gamepads_text.push(Line::from(vec![
                        Span::styled("  dpad:", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(dpad_text, Style::new().fg(TEXT_COLOR)),
                    ]));
                }

                let mut general_buttons_pressed = Vec::new();

                if state.a {
                    general_buttons_pressed.push("a")
                };
                if state.b {
                    general_buttons_pressed.push("b")
                };
                if state.x {
                    general_buttons_pressed.push("x")
                };
                if state.y {
                    general_buttons_pressed.push("y")
                };
                if state.guide {
                    general_buttons_pressed.push("guide")
                };
                if state.start {
                    general_buttons_pressed.push("start")
                };
                if state.back {
                    general_buttons_pressed.push("back")
                };

                if !general_buttons_pressed.is_empty() {
                    let mut buttons_text = String::new();

                    for text in general_buttons_pressed {
                        buttons_text.push(' ');
                        buttons_text.push_str(&text);
                    }

                    gamepads_text.push(Line::from(vec![
                        Span::styled("  buttons:", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled(buttons_text, Style::new().fg(TEXT_COLOR)),
                    ]));
                }

                if state.touchpad {
                    gamepads_text.push(Line::from(Span::styled(
                        "  touchpad",
                        Style::new().fg(TEXT_COLOR),
                    )));
                }

                gamepads_text.push(Line::from(""));
            } else {
                gamepads_text.push(Line::from(vec![
                    Span::styled(format!("Gamepad {}", i + 1), Style::new().fg(TEXT_COLOR)),
                    Span::styled(" - not connected", Style::new().fg(MUTED_TEXT_COLOR)),
                ]));
            }
        }

        Paragraph::new(gamepads_text).wrap(Wrap { trim: false })
    }

    /// Creates the robot text
    pub async fn create_robot_paragraph(&mut self) -> Paragraph {
        let mut robot_text: Vec<Line> = Vec::new();

        let Some(robot) = &*self.robot.read().await else {
            robot_text.push(Line::from(vec![Span::styled(
                "Not connected to robot yet..",
                Style::new().fg(MUTED_TEXT_COLOR),
            )]));

            return Paragraph::new(robot_text).wrap(Wrap { trim: false });
        };

        let Some(status) = &robot.status else {
            robot_text.push(Line::from(vec![Span::styled(
                "Waiting to receive status..",
                Style::new().fg(MUTED_TEXT_COLOR),
            )]));

            return Paragraph::new(robot_text).wrap(Wrap { trim: false });
        };

        robot_text.push(Line::from(Span::styled(
            format!(
                "Last update was {} second(s) ago",
                (std::time::Instant::now() - robot.last_status_update).as_secs()
            ),
            Style::new().fg(MUTED_TEXT_COLOR),
        )));

        let voltage_color = if status.battery_voltage <= 7.5 {
            ERROR_COLOR
        } else if status.battery_voltage < 10.0 {
            WARNING_COLOR
        } else {
            SUCCESS_COLOR
        };

        robot_text.push(Line::from(vec![
            Span::styled("Battery voltage: ", Style::new().fg(MUTED_TEXT_COLOR)),
            Span::styled(
                format!("{:.1} V", status.battery_voltage),
                Style::new().fg(voltage_color),
            ),
        ]));

        if !status.enabled {
            robot_text.push(Line::from(Span::styled(
                "Not enabled",
                Style::new().fg(WARNING_COLOR),
            )));
        }

        if !status.available {
            robot_text.push(Line::from(Span::styled(
                "Not available",
                Style::new().fg(WARNING_COLOR),
            )));
        }

			if status.active_op_mode == "$Stop$Robot$" {
            robot_text.push(Line::from(""));

            robot_text.push(Line::from(vec![
                Span::styled("Robot is stopped.", Style::new().fg(TEXT_COLOR)),
            ]));
        }

        else if !status.active_op_mode.is_empty() {
            robot_text.push(Line::from(""));

            robot_text.push(Line::from(vec![
                Span::styled("Active OpMode: ", Style::new().fg(MUTED_TEXT_COLOR)),
                Span::styled(status.active_op_mode.clone(), Style::new().fg(TEXT_COLOR)),
            ]));

            match status.active_op_mode_status {
                OpModeStatus::Init => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Initialized", Style::new().fg(PRIMARY_COLOR_LIGHTER)),
                    ]));
                }
                OpModeStatus::Running => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Running", Style::new().fg(SUCCESS_COLOR)),
                    ]));
                }
                OpModeStatus::Stopped => {
                    robot_text.push(Line::from(vec![
                        Span::styled("OpMode status: ", Style::new().fg(MUTED_TEXT_COLOR)),
                        Span::styled("Stopped", Style::new().fg(WARNING_COLOR)),
                    ]));
                }
            }
        }

        robot_text.push(Line::from(""));

        if !status.warning_message.is_empty() {
            robot_text.push(Line::from(Span::styled(
                status.warning_message.clone(),
                Style::new().fg(WARNING_COLOR),
            )));
        }

        if !status.error_message.is_empty() {
            robot_text.push(Line::from(Span::styled(
                status.error_message.clone(),
                Style::new().fg(ERROR_COLOR),
            )));
        }

        Paragraph::new(robot_text).wrap(Wrap { trim: false })
    }

    /// Creates the opmode list
    pub async fn create_opmode_list_paragraph(&mut self) -> Paragraph {
        let mut text: Vec<Line> = Vec::new();

        let Some(robot) = &*self.robot.read().await else {
            text.push(Line::from(vec![Span::styled(
                "Not connected to robot yet..",
                Style::new().fg(MUTED_TEXT_COLOR),
            )]));

            return Paragraph::new(text).wrap(Wrap { trim: false });
        };

        let Some(opmode_list) = &robot.opmode_list else {
            text.push(Line::from(vec![Span::styled(
                "Waiting to receive opmodes..",
                Style::new().fg(MUTED_TEXT_COLOR),
            )]));

            return Paragraph::new(text).wrap(Wrap { trim: false });
        };

        for i in 0..opmode_list.len() {
            let opmode_name = opmode_list[i].clone();

            let selected =
                self.opmode_list_selected_index == i && self.selected_block == OP_MODES_BLOCK_ID;

            let mut style = if selected {
                Style::new().fg(TEXT_COLOR).bg(SELECTED_BACKGROUND)
            } else {
                Style::new().fg(TEXT_COLOR)
            };

            if let Some(status) = &robot.status {
                if status.active_op_mode == opmode_name {
                    match status.active_op_mode_status {
                        OpModeStatus::Init => style = style.fg(PRIMARY_COLOR_LIGHTER),
                        OpModeStatus::Running => style = style.fg(SUCCESS_COLOR),
                        OpModeStatus::Stopped => style = style.fg(WARNING_COLOR),
                    }
                }
            }

            text.push(Line::from(Span::styled(opmode_name, style)));
        }

        Paragraph::new(text).wrap(Wrap { trim: false })
    }
}
