use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding, Paragraph, Wrap},
};
use styles::{
    ERROR_COLOR, MUTED_TEXT_COLOR, PRIMARY_COLOR_LIGHTER, SELECTED_BACKGROUND, SUCCESS_COLOR,
    TEXT_COLOR, WARNING_COLOR, block_style, selected_block_style,
};

use crate::{
    App,
    app::{
        ACTIVE_OPMODE_BLOCK_ID, AUTO_BLOCK_ID, DEBUG_BLOCK_ID, GAMEPADS_BLOCK_ID, ROBOT_BLOCK_ID,
        TELEOP_BLOCK_ID,
    },
    ftc_proto::{gamepad_packet::ButtonFlags, time_packet::RobotOpmodeState},
    network::NetworkStatus,
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

        frame.render_widget(&debug_block, top_inner_layout[0]);

        // These two blocks are rendered in the list render function
        //frame.render_widget(&teleop_block, top_inner_layout[1]);
        //frame.render_widget(&auto_block, top_inner_layout[2]);

        frame.render_widget(&robot_block, top_inner_layout[3]);

        frame.render_widget(&active_opmode_block, bottom_inner_layout[0]);
        frame.render_widget(&gamepads_block, bottom_inner_layout[1]);

        frame.render_widget(
            self.create_debug_paragraph().await,
            debug_block.inner(top_inner_layout[0]),
        );

        self.render_teleop_list(frame, teleop_block, top_inner_layout[1])
            .await;
        self.render_auto_list(frame, auto_block, top_inner_layout[2])
            .await;

        frame.render_widget(
            self.create_robot_paragraph().await,
            robot_block.inner(top_inner_layout[3]),
        );

        frame.render_widget(
            self.create_active_opmode_paragraph().await,
            active_opmode_block.inner(bottom_inner_layout[0]),
        );

        frame.render_widget(
            self.create_gamepads_paragraph().await,
            gamepads_block.inner(bottom_inner_layout[1]),
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

                let button_flags = ButtonFlags::from_bits(state.button_flags).unwrap();

                if button_flags.contains(ButtonFlags::LEFT_STICK_BUTTON) {
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

                if button_flags.contains(ButtonFlags::RIGHT_STICK_BUTTON) {
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

                if button_flags.contains(ButtonFlags::LEFT_BUMPER) {
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

                if button_flags.contains(ButtonFlags::RIGHT_BUMPER) {
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

                if button_flags.contains(ButtonFlags::DPAD_UP) {
                    dpad_buttons_pressed.push("up")
                };
                if button_flags.contains(ButtonFlags::DPAD_LEFT) {
                    dpad_buttons_pressed.push("left")
                };
                if button_flags.contains(ButtonFlags::DPAD_RIGHT) {
                    dpad_buttons_pressed.push("right")
                };
                if button_flags.contains(ButtonFlags::DPAD_DOWN) {
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

                if button_flags.contains(ButtonFlags::A) {
                    general_buttons_pressed.push("a")
                };
                if button_flags.contains(ButtonFlags::B) {
                    general_buttons_pressed.push("b")
                };
                if button_flags.contains(ButtonFlags::X) {
                    general_buttons_pressed.push("x")
                };
                if button_flags.contains(ButtonFlags::Y) {
                    general_buttons_pressed.push("y")
                };
                if button_flags.contains(ButtonFlags::GUIDE) {
                    general_buttons_pressed.push("guide")
                };
                if button_flags.contains(ButtonFlags::START) {
                    general_buttons_pressed.push("start")
                };
                if button_flags.contains(ButtonFlags::BACK) {
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

        let robot = self.robot.read().await;

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
            let voltage_color = if battery_voltage <= 7.5 {
                ERROR_COLOR
            } else if battery_voltage < 10.0 {
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

    /// Creates the debug text
    pub async fn create_debug_paragraph(&mut self) -> Paragraph {
        let mut debug_text: Vec<Line> = Vec::new();

        let shared_network_read = self.shared_network_data.read().await;

        let mut network_state_line = Vec::new();

        network_state_line.push(Span::styled(
            "Network status: ".to_string(),
            Style::new().fg(MUTED_TEXT_COLOR),
        ));

        match shared_network_read.state {
            NetworkStatus::Establishing => network_state_line.push(Span::styled(
                "Establishing..".to_string(),
                Style::new().fg(WARNING_COLOR),
            )),
            NetworkStatus::Disconnected => network_state_line.push(Span::styled(
                "Disconnected.".to_string(),
                Style::new().fg(ERROR_COLOR),
            )),
            NetworkStatus::Connected => network_state_line.push(Span::styled(
                "Connected!".to_string(),
                Style::new().fg(SUCCESS_COLOR),
            )),
        }

        debug_text.push(Line::from(network_state_line));

        let mut last_packet_line = Vec::new();

        last_packet_line.push(Span::styled(
            "Last packet was ".to_string(),
            Style::new().fg(MUTED_TEXT_COLOR),
        ));

        if let Some(last_packet) = shared_network_read.last_received {
            last_packet_line.push(Span::styled(
                format!("{:.1?}", last_packet.elapsed()),
                Style::new().fg(TEXT_COLOR),
            ));
            last_packet_line.push(Span::styled(
                " ago".to_string(),
                Style::new().fg(MUTED_TEXT_COLOR),
            ));
        } else {
            last_packet_line.push(Span::styled(
                "never".to_string(),
                Style::new().fg(TEXT_COLOR),
            ));
        }

        debug_text.push(Line::from(last_packet_line));

        Paragraph::new(debug_text).wrap(Wrap { trim: false })
    }

    /// Creates the teleop opmode list
    pub async fn render_teleop_list(
        &mut self,
        frame: &mut Frame<'_>,
        block: Block<'_>,
        rect: Rect,
    ) {
        let mut items: Vec<ListItem> = Vec::new();

        let opmode_list = self.get_teleop_opmodes().await;

        let robot = self.robot.read().await;

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
    pub async fn render_auto_list(&mut self, frame: &mut Frame<'_>, block: Block<'_>, rect: Rect) {
        let mut items: Vec<ListItem> = Vec::new();

        let opmode_list = self.get_auto_opmodes().await;

        let robot = self.robot.read().await;

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
    pub async fn create_active_opmode_paragraph(&mut self) -> Paragraph {
        let mut text: Vec<Line> = Vec::new();

        let robot = self.robot.read().await;

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
