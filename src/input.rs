use std::time::Duration;

use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    App,
    app::OP_MODES_BLOCK_ID,
    ftc_dashboard::{gamepad_state::GamepadState, message::Message, robot_status::OpModeStatus},
};

use gilrs::{Axis, Button, GamepadId, Gilrs};

impl App {
    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    pub async fn handle_crossterm_events(&mut self) -> Result<()> {
        // Return if there are no events
        if !event::poll(Duration::from_millis(10)).unwrap() {
            return Ok(());
        }

        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key).await,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Quit handler
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit().await,

            // Main action button
            (_, KeyCode::Enter) => match self.selected_block {
                OP_MODES_BLOCK_ID => {
                    if let Some(robot) = &*self.robot.read().await {
                        if let Some(op_modes) = &robot.opmode_list {
                            if let Some(robot_status) = &robot.status {
                                let selected_op_mode =
                                    op_modes[self.opmode_list_selected_index].clone();

                                if robot_status.active_op_mode == selected_op_mode {
                                    match robot_status.active_op_mode_status {
                                        OpModeStatus::Init | OpModeStatus::Stopped => {
                                            self.send_message(Message::StartOpMode).await;
                                        }
                                        OpModeStatus::Running => {
                                            self.send_message(Message::StopOpMode).await;
                                        }
                                    }
                                } else {
                                    self.send_message(Message::InitOpMode(
                                        crate::ftc_dashboard::message::InitOpMode {
                                            op_mode_name: selected_op_mode,
                                        },
                                    ))
                                    .await;
                                }
                            }
                        }
                    }
                }
                _ => {}
            },

            // Stop / start button
            (_, KeyCode::Char(' ')) => {
                if let Some(robot) = &*self.robot.read().await {
                    if let Some(robot_status) = &robot.status {
                        if !robot_status.active_op_mode.is_empty() {
                            match robot_status.active_op_mode_status {
                                OpModeStatus::Init | OpModeStatus::Stopped => {
                                    self.send_message(Message::StartOpMode).await;
                                }
                                OpModeStatus::Running => {
                                    self.send_message(Message::StopOpMode).await;
                                }
                            }
                        }
                    }
                }
            }

            // Move main selection forwards and backwards
            (_, KeyCode::Tab) => {
                if self.selected_block == 4 {
                    self.selected_block = 0;
                } else {
                    self.selected_block = self.selected_block + 1;
                }
            }

            // Move main selection forwards and backwards
            (_, KeyCode::BackTab) => {
                if self.selected_block == 0 {
                    self.selected_block = 4;
                } else {
                    self.selected_block = self.selected_block - 1;
                }
            }

            // Move sub selection up and down
            (_, KeyCode::Up) | (_, KeyCode::Char('k')) => match self.selected_block {
                OP_MODES_BLOCK_ID => {
                    if self.opmode_list_selected_index == 0 {
                        let mut max_index = 0;

                        if let Some(robot) = &*self.robot.read().await {
                            if let Some(op_modes) = &robot.opmode_list {
                                max_index = op_modes.len() - 1;
                            }
                        }

                        self.opmode_list_selected_index = max_index;
                    } else {
                        self.opmode_list_selected_index -= 1;
                    }
                }
                _ => {}
            },

            (_, KeyCode::Down) | (_, KeyCode::Char('j')) => match self.selected_block {
                OP_MODES_BLOCK_ID => {
                    let mut max_index = 0;

                    if let Some(robot) = &*self.robot.read().await {
                        if let Some(op_modes) = &robot.opmode_list {
                            max_index = op_modes.len() - 1;
                        }
                    }

                    if self.opmode_list_selected_index == max_index {
                        self.opmode_list_selected_index = 0;
                    } else {
                        self.opmode_list_selected_index += 1;
                    }
                }
                _ => {}
            },

            _ => {}
        }
    }

    /// Checks if there are any new gamepads / if any have disconnected and updates self if there are
    pub async fn update_gamepads(&mut self) {
        let gamepads: Vec<(GamepadId, gilrs::Gamepad)> = self.gilrs.gamepads().collect();
        let gamepad_ids: Vec<GamepadId> = gamepads.clone().iter().map(|x| x.0).collect();

        let mut gamepad_one = self.gamepad_one.write().await;
        let mut gamepad_two = self.gamepad_two.write().await;

        // Check if any gamepads disconnected
        if let Some(gamepad) = &*gamepad_one {
            if !gamepad_ids.contains(&gamepad.id) {
                *gamepad_one = None;
            }
        }

        if let Some(gamepad) = &*gamepad_two {
            if !gamepad_ids.contains(&gamepad.id) {
                *gamepad_two = None;
            }
        }

        // Check for assigning new gamepads
        for (id, _gamepad) in self.gilrs.gamepads() {
            if let Some(gamepad_one) = &*gamepad_one {
                if gamepad_two.is_none() && gamepad_one.id != id {
                    *gamepad_two = Some(Gamepad {
                        id,
                        last_state: Gamepad::map_to_fsdb_gamepad_state(id, &self.gilrs),
                    });
                    continue;
                }
            } else {
                *gamepad_one = Some(Gamepad {
                    id,
                    last_state: Gamepad::map_to_fsdb_gamepad_state(id, &self.gilrs),
                })
            }
        }

        // TODO: update when we have an input!
        while let Some(gilrs::Event {
            id, event, time, ..
        }) = self.gilrs.next_event()
        {}

        // Update our own states
        if let Some(gamepad) = &mut *gamepad_one {
            gamepad.last_state = Gamepad::map_to_fsdb_gamepad_state(gamepad.id, &self.gilrs);
        }

        if let Some(gamepad) = &mut *gamepad_two {
            gamepad.last_state = Gamepad::map_to_fsdb_gamepad_state(gamepad.id, &self.gilrs);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Gamepad {
    pub id: GamepadId,
    pub last_state: GamepadState,
}

impl Gamepad {
    pub fn map_to_fsdb_gamepad_state(id: GamepadId, gilrs: &Gilrs) -> GamepadState {
        let gamepad = gilrs.gamepad(id);

        GamepadState {
            left_stick_x: gamepad.value(Axis::LeftStickX),
            left_stick_y: gamepad.value(Axis::LeftStickY),
            left_stick_button: gamepad.is_pressed(Button::LeftThumb),
            right_stick_x: gamepad.value(Axis::RightStickX),
            right_stick_y: gamepad.value(Axis::RightStickY),
            right_stick_button: gamepad.is_pressed(Button::RightThumb),
            dpad_up: gamepad.is_pressed(Button::DPadUp),
            dpad_down: gamepad.is_pressed(Button::DPadDown),
            dpad_left: gamepad.is_pressed(Button::DPadLeft),
            dpad_right: gamepad.is_pressed(Button::DPadRight),
            a: gamepad.is_pressed(Button::South),
            b: gamepad.is_pressed(Button::East),
            x: gamepad.is_pressed(Button::West),
            y: gamepad.is_pressed(Button::North),
            guide: gamepad.is_pressed(Button::Mode),
            start: gamepad.is_pressed(Button::Start),
            back: gamepad.is_pressed(Button::Select),
            left_bumper: gamepad.is_pressed(Button::LeftTrigger),
            right_bumper: gamepad.is_pressed(Button::RightTrigger),

            // Note: unideal, but we can't get the analog value from gilrs
            left_trigger: gamepad.is_pressed(Button::LeftTrigger2) as u8 as f32,
            right_trigger: gamepad.is_pressed(Button::RightTrigger2) as u8 as f32,
            touchpad: false,
        }
    }
}
