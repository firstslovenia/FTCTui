use std::sync::Arc;

use async_lock::Mutex;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    App,
    app::{
        ACTIVE_OPMODE_BLOCK_ID, AUTO_BLOCK_ID, AppMode, GAMEPADS_BLOCK_ID, TELEOP_BLOCK_ID,
        get_timestamp_millis,
    },
    ftc_proto::{
        command_packet::OPMODE_STOP,
        gamepad_packet::{ButtonFlags, GamepadPacketData},
        time_packet::RobotOpmodeState,
    },
    popup::RestartRobotPopup,
};

use gilrs::{Axis, Button, GamepadId, Gilrs, MappingSource};

impl App {
    /// Handles the key events and updates the state of [`App`].
    pub async fn on_key_event(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::Normal => self.on_normal_mode_key_event(key).await,
            AppMode::InsertCommand => self.on_command_insert_mode_key_event(key).await,
        }
    }

    /// Handles the key events and updates the state of [`App`] when in normal mode.
    ///
    /// Normal mode is also the only which supports interacting with popups.
    pub async fn on_normal_mode_key_event(&mut self, key: KeyEvent) {
        // Universal, always active key handlers
        match (key.modifiers, key.code) {
            // Quit handler
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit().await,

            // Restart handler
            (_, KeyCode::Char('r')) => {
                self.active_popup = Some(Arc::new(Mutex::new(RestartRobotPopup {
                    selected_yes: false,
                })))
            }

            // Change modes into command mode
            (_, KeyCode::Char(':')) => {
                self.mode = AppMode::InsertCommand;
                self.current_command.clear();
            }

            _ => {}
        }

        // Popup key handlers
        if let Some(popup) = self.active_popup.clone() {
            match (key.modifiers, key.code) {
                // Submit
                (_, KeyCode::Enter) => {
                    popup.lock().await.submit(self);
                    self.active_popup = None;
                }

                // Move selected option forwards and backwards
                (_, KeyCode::Tab) | (_, KeyCode::Right) | (_, KeyCode::Char('l')) => {
                    popup.lock().await.select_next_option();
                }

                (_, KeyCode::BackTab) | (_, KeyCode::Left) | (_, KeyCode::Char('h')) => {
                    popup.lock().await.select_previous_option();
                }

                // Scrolls up and down
                (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                    popup.lock().await.scroll_up();
                }

                (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                    popup.lock().await.scroll_down();
                }

                _ => {}
            }

            return;
        }

        // Handlers when we don't have any popups
        match (key.modifiers, key.code) {
            // Main action button
            (_, KeyCode::Enter) => match self.selected_block {
                AUTO_BLOCK_ID | TELEOP_BLOCK_ID => {
                    if let Some(selected_op_mode) = self.get_selected_opmode().await {
                        let robot = self.robot.read().await;

                        if let Some(status) = &robot.active_opmode_state {
                            if robot.active_opmode == selected_op_mode.name {
                                match status {
                                    RobotOpmodeState::Initialized | RobotOpmodeState::Stopped => {
                                        self.start_opmode(selected_op_mode.name.clone()).await;
                                    }
                                    RobotOpmodeState::Running => {
                                        self.stop_opmode().await;
                                    }
                                    _ => {}
                                }
                            } else {
                                self.init_opmode(selected_op_mode.name.clone()).await;
                            }
                        }
                    }
                }
                _ => {}
            },

            // Stop / start button
            (_, KeyCode::Char(' ')) => {
                let robot = self.robot.read().await;

                if let Some(opmode_state) = &robot.active_opmode_state {
                    if robot.active_opmode != OPMODE_STOP {
                        match opmode_state {
                            RobotOpmodeState::Initialized
                            | RobotOpmodeState::Stopped
                            | RobotOpmodeState::NotStarted => {
                                self.start_opmode(robot.active_opmode.clone()).await;
                            }
                            RobotOpmodeState::Running => {
                                self.stop_opmode().await;
                            }
                            _ => {}
                        }
                    }
                }
            }

            // Move main selection forwards and backwards
            (_, KeyCode::Tab) | (_, KeyCode::Right) => {
                if self.selected_block == GAMEPADS_BLOCK_ID {
                    self.selected_block = 0;
                } else {
                    self.selected_block = self.selected_block + 1;
                }
            }

            // Move main selection forwards and backwards
            (_, KeyCode::BackTab) | (_, KeyCode::Left) => {
                if self.selected_block == 0 {
                    self.selected_block = GAMEPADS_BLOCK_ID;
                } else {
                    self.selected_block = self.selected_block - 1;
                }
            }

            // Move sub selection up and down
            (_, KeyCode::Up) | (_, KeyCode::Char('k')) => match self.selected_block {
                AUTO_BLOCK_ID => {
                    self.auto_list_state.select_previous();
                }
                TELEOP_BLOCK_ID => {
                    self.teleop_list_state.select_previous();
                }
                ACTIVE_OPMODE_BLOCK_ID => {
                    if self.telemetry_display_scroll != 0 {
                        self.telemetry_display_scroll -= 1;
                    }
                }
                _ => {}
            },

            (_, KeyCode::Down) | (_, KeyCode::Char('j')) => match self.selected_block {
                AUTO_BLOCK_ID => {
                    self.auto_list_state.select_next();
                }
                TELEOP_BLOCK_ID => {
                    self.teleop_list_state.select_next();
                }
                ACTIVE_OPMODE_BLOCK_ID => {
                    let mut max_index = 0;

                    let telemetry_lines_len = self.robot.read().await.telemetry_list.len();

                    if telemetry_lines_len != 0 {
                        max_index = telemetry_lines_len - 1;
                    }

                    if self.telemetry_display_scroll != max_index as u16 {
                        self.telemetry_display_scroll += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Handles the key events and updates the state of [`App`] when in command insertion mode.
    pub async fn on_command_insert_mode_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Return to normal mode
            (_, KeyCode::Esc) => {
                self.mode = AppMode::Normal;
                self.current_command.clear();
            }

            // Submit command
            (_, KeyCode::Enter) => {
                self.mode = AppMode::Normal;
                self.submit_command(self.current_command.clone()).await;
                self.current_command.clear();
            }

            // Delete one character
            (_, KeyCode::Backspace) => {
                self.current_command.pop();
            }

            (_, KeyCode::Char(char)) => {
                self.current_command.push(char);
            }
            _ => {}
        }
    }

    /// Checks if there are any new gamepads / if any have disconnected and updates self if there are
    pub async fn update_gamepads(&mut self) {
        let gamepads: Vec<(GamepadId, gilrs::Gamepad)> = self.gilrs.0.gamepads().collect();
        let gamepad_ids: Vec<GamepadId> = gamepads.clone().iter().map(|x| x.0).collect();

        let mut gamepad_one = self.gamepad_one.write().await;
        let mut gamepad_two = self.gamepad_two.write().await;

        // Check if any gamepads disconnected and if we want to unbind them
        if let Some(gamepad) = &*gamepad_one {
            if !gamepad_ids.contains(&gamepad.id) {
                *gamepad_one = None;
            } else {
                let gamepad = self.gilrs.0.gamepad(gamepad.id);

                if gamepad.is_pressed(Button::Start) && gamepad.is_pressed(Button::West) {
                    *gamepad_one = None;
                }
            }
        }

        if let Some(gamepad) = &*gamepad_two {
            if !gamepad_ids.contains(&gamepad.id) {
                *gamepad_two = None;
            } else {
                let gamepad = self.gilrs.0.gamepad(gamepad.id);

                if gamepad.is_pressed(Button::Start) && gamepad.is_pressed(Button::West) {
                    *gamepad_two = None;
                }
            }
        }

        // Check for assigning new gamepads
        for (id, gamepad) in self.gilrs.0.gamepads() {
            if gamepad.mapping_source() == MappingSource::None {
                continue;
            }

            // Not a gamepad
            if gamepad
                .name()
                .contains("Framework Laptop 16 Keyboard Module")
            {
                continue;
            }

            // Bind for gamepad 1
            if gamepad.is_pressed(Button::Start) && gamepad.is_pressed(Button::South) {
                *gamepad_one = Some(Gamepad {
                    id,
                    last_state: Gamepad::map_to_gamepad_packet_data(id, 1, &self.gilrs.0),
                });

                if let Some(gp_two) = &*gamepad_two {
                    if gp_two.id == gamepad.id() {
                        *gamepad_two = None;
                    }
                }
            }
            // Bind for gamepad 2
            else if gamepad.is_pressed(Button::Start) && gamepad.is_pressed(Button::East) {
                *gamepad_two = Some(Gamepad {
                    id,
                    last_state: Gamepad::map_to_gamepad_packet_data(id, 2, &self.gilrs.0),
                });

                if let Some(gp_one) = &*gamepad_one {
                    if gp_one.id == gamepad.id() {
                        *gamepad_one = None;
                    }
                }
            }
        }

        // Update our own states
        if let Some(gamepad) = &mut *gamepad_one {
            gamepad.last_state = Gamepad::map_to_gamepad_packet_data(gamepad.id, 1, &self.gilrs.0);
        }

        if let Some(gamepad) = &mut *gamepad_two {
            gamepad.last_state = Gamepad::map_to_gamepad_packet_data(gamepad.id, 2, &self.gilrs.0);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Gamepad {
    pub id: GamepadId,
    pub last_state: GamepadPacketData,
}

impl Gamepad {
    pub fn map_to_gamepad_packet_data(id: GamepadId, user: u8, gilrs: &Gilrs) -> GamepadPacketData {
        let gamepad = gilrs.gamepad(id);

        let timestamp = get_timestamp_millis();

        let mut flags = ButtonFlags::empty();

        if gamepad.is_pressed(Button::LeftThumb) {
            flags = flags | ButtonFlags::LEFT_STICK_BUTTON;
        }

        if gamepad.is_pressed(Button::RightThumb) {
            flags = flags | ButtonFlags::RIGHT_STICK_BUTTON;
        }

        if gamepad.is_pressed(Button::DPadUp) {
            flags = flags | ButtonFlags::DPAD_UP;
        }

        if gamepad.is_pressed(Button::DPadDown) {
            flags = flags | ButtonFlags::DPAD_DOWN;
        }

        if gamepad.is_pressed(Button::DPadLeft) {
            flags = flags | ButtonFlags::DPAD_LEFT;
        }

        if gamepad.is_pressed(Button::DPadRight) {
            flags = flags | ButtonFlags::DPAD_RIGHT;
        }

        if gamepad.is_pressed(Button::South) {
            flags = flags | ButtonFlags::A;
        }

        if gamepad.is_pressed(Button::East) {
            flags = flags | ButtonFlags::B;
        }

        if gamepad.is_pressed(Button::West) {
            flags = flags | ButtonFlags::X;
        }

        if gamepad.is_pressed(Button::North) {
            flags = flags | ButtonFlags::Y;
        }

        if gamepad.is_pressed(Button::Mode) {
            flags = flags | ButtonFlags::GUIDE;
        }

        if gamepad.is_pressed(Button::Start) {
            flags = flags | ButtonFlags::START;
        }

        if gamepad.is_pressed(Button::Select) {
            flags = flags | ButtonFlags::BACK;
        }

        if gamepad.is_pressed(Button::LeftTrigger) {
            flags = flags | ButtonFlags::LEFT_BUMPER;
        }

        if gamepad.is_pressed(Button::RightTrigger) {
            flags = flags | ButtonFlags::RIGHT_BUMPER;
        }

        GamepadPacketData {
            gamepad_id: usize::from(gamepad.id()) as i32,
            left_stick_x: gamepad.value(Axis::LeftStickX),
            // Note: up is negative, different than the convention
            left_stick_y: -gamepad.value(Axis::LeftStickY),
            right_stick_x: gamepad.value(Axis::RightStickX),
            // Note: up is negative, different than the convention
            right_stick_y: -gamepad.value(Axis::RightStickY),
            timestamp,
            // FIXME: unideal, but we can't? get the analog value from gilrs
            //
            // Some have Left + Right Z, but not all are mapped
            left_trigger: gamepad.is_pressed(Button::LeftTrigger2) as u8 as f32,
            right_trigger: gamepad.is_pressed(Button::RightTrigger2) as u8 as f32,
            button_flags: flags.bits(),
            ..GamepadPacketData::default_for_user(user)
        }
    }
}
