use ratatui::{
	style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{app::App, ftc_proto::gamepad_packet::ButtonFlags, renderers::styles::{MUTED_TEXT_COLOR, SUCCESS_COLOR, TEXT_COLOR}};

impl App {
    /// Creates the gamepads debug text
    pub fn create_gamepads_paragraph(&mut self) -> Paragraph<'_> {
        let gamepads = vec![
            futures::executor::block_on(self.gamepad_one.read()).clone(),
            futures::executor::block_on(self.gamepad_two.read()).clone(),
        ];

        let mut gamepads_text: Vec<Line> = Vec::new();

        for i in 0..gamepads.len() {
            let gamepad_option = gamepads[i].clone();

            if let Some(gamepad) = gamepad_option {
                let state = gamepad.last_state.clone();
                let gilrs_gamepad = self.gilrs.0.gamepad(gamepad.id);

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
                    Span::styled(" - not bound", Style::new().fg(MUTED_TEXT_COLOR)),
                ]));
            }
        }

        Paragraph::new(gamepads_text).wrap(Wrap { trim: false })
    }
}
