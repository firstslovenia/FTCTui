use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Flex, Layout},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Clear, List, ListItem, ListState, Padding},
};

use crate::{
    app::{App, AppMode, get_timestamp_nanos},
    ftc_proto::command_packet::{CommandPacketData, REQUEST_CONFIGURATION, RobotConfigurationFile},
    network::send_command,
    renderers::styles::{
        MUTED_TEXT_COLOR, SELECTED_BACKGROUND, SUCCESS_COLOR, TEXT_COLOR, block_style,
        muted_text_style, text_style,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HardwareConfigurationUI {
    /// Choosing which configuration to use
    pub configurations_state: Option<ListState>,

    /// Configuration menu
    pub selected_configuration: Option<RobotConfigurationFile>,
    pub selected_configuration_data: Option<crate::ftc_proto::hardware::robot::Robot>,
    pub selected_configuration_state: Option<ListState>,

    /// Editing a configuration
    pub selected_configuration_edited_data: Option<crate::ftc_proto::hardware::robot::Robot>,
}

impl Default for HardwareConfigurationUI {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select_next();

        HardwareConfigurationUI {
            configurations_state: Some(list_state),
            selected_configuration: None,
            selected_configuration_state: None,
            selected_configuration_data: None,
            selected_configuration_edited_data: None,
        }
    }
}

impl HardwareConfigurationUI {
    /// Renders the hardware configuration UI, assuming it's open
    pub fn render(app: &mut App, frame: &mut Frame<'_>) {
        let AppMode::ConfigureHardware(state) = &mut app.mode else {
            return;
        };

        let block = Block::bordered()
            .title("Hardware Configurations")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let robot = futures::executor::block_on(app.robot.read());
        let hardware = futures::executor::block_on(robot.hardware.read());
        let mut render_configurations_list = None;
        let mut render_selected_list = None;
        let mut max_length = 0;
        let mut max_height = 0;

        // Selecting a configuration
        if let Some(config_list_state) = state.configurations_state {
            let mut items: Vec<ListItem> = Vec::new();

            items.push(ListItem::new(Span::styled("Back", muted_text_style())));
            items.push(ListItem::new(Span::styled(
                "Import from file",
                muted_text_style(),
            )));
            items.push(ListItem::new(Span::styled("New", muted_text_style())));

            if let Some(configurations) = &hardware.configurations {
                for i in 0..configurations.len() {
                    let mut selected_config = configurations[i].clone();

                    let mut style = text_style();

                    if let Some(active_config) = &hardware.active_configuration {
                        if active_config.name == selected_config.name {
                            selected_config.name = format!(">{}", selected_config.name);
                            style = style.fg(SUCCESS_COLOR);
                        }
                    }

                    items.push(ListItem::new(Span::styled(selected_config.name, style)));
                }
            }

            for (i, item) in items.iter_mut().enumerate() {
                let selected = config_list_state.selected().unwrap_or_default() == i;

                if selected {
                    *item = item.clone().bg(SELECTED_BACKGROUND);
                }
            }

            for item in &items {
                if item.width() > max_length {
                    max_length = item.width();
                }
            }
            max_height = items.len();

            let list = List::new(items);
            render_configurations_list = Some(list);
        }

        // Menu for one specific configuration
        if let Some(selected_config) = &state.selected_configuration {
            let list_state = state.selected_configuration_state.unwrap();
            let mut items: Vec<ListItem> = Vec::new();

            items.push(ListItem::new(Span::styled(
                &selected_config.name,
                text_style(),
            )));
            items.push(ListItem::new(Span::styled("Back", muted_text_style())));
            items.push(ListItem::new(Span::styled("Save", muted_text_style())));
            items.push(ListItem::new(Span::styled("Edit", muted_text_style())));
            items.push(ListItem::new(Span::styled(
                "Export to file",
                muted_text_style(),
            )));

            for (i, item) in items.iter_mut().enumerate() {
                let selected = list_state.selected().unwrap_or_default() == i;

                if selected {
                    *item = item.clone().bg(SELECTED_BACKGROUND);
                }
            }

            for item in &items {
                if item.width() > max_length {
                    max_length = item.width();
                }
            }
            max_height = items.len();

            let list = List::new(items);
            render_selected_list = Some(list);
        }

        let mut horizontal = Layout::horizontal([Constraint::Percentage(30)]).flex(Flex::Center);
        let mut vertical = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);

        // Build it to test the width..
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let block_inner_area = block.inner(area);

        let wanted_width = max_length as u16;
        if wanted_width > block_inner_area.width {
            // Too big, do 90%
            if wanted_width > frame.area().width {
                horizontal = Layout::horizontal([Constraint::Percentage(75)]).flex(Flex::Center);
            } else {
                horizontal =
                    Layout::horizontal([Constraint::Length(wanted_width + 2)]).flex(Flex::Center);
            }
        }

        let lines = max_height as u16;

        if lines > frame.area().height {
            vertical = Layout::vertical([Constraint::Percentage(75)]).flex(Flex::Center);
        } else {
            // 5: 4 to just fit
            let height = lines + 4;

            vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
        }

        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let block_inner_area = block.inner(area);

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        frame.render_widget(Clear, block_inner_area);

        if let Some(list) = render_configurations_list {
            frame.render_stateful_widget(
                list,
                block_inner_area,
                state.configurations_state.as_mut().unwrap(),
            );
        } else if let Some(list) = render_selected_list {
            frame.render_stateful_widget(
                list,
                block_inner_area,
                state.selected_configuration_state.as_mut().unwrap(),
            );
        }
    }

    /// Handles inputs when in the UI
    pub async fn on_key_event(app: &mut App, key: KeyEvent) {
        let AppMode::ConfigureHardware(state) = &mut app.mode else {
            return;
        };

        match (key.modifiers, key.code) {
            // Quit handler
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                app.quit().await;
                return;
            }
            // Go back to normal mode
            (_, KeyCode::Esc) => {
                app.mode = AppMode::Normal;
                return;
            }
            _ => {}
        }

        let robot = futures::executor::block_on(app.robot.read());
        let hardware = futures::executor::block_on(robot.hardware.read());

        if let Some(configurations_list_state) = state.configurations_state.as_mut() {
            match (key.modifiers, key.code) {
                (_, KeyCode::BackTab) | (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                    configurations_list_state.select_previous();
                }

                (_, KeyCode::Tab) | (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                    configurations_list_state.select_next();
                }
                (_, KeyCode::Enter) => {
                    let selected = configurations_list_state.selected().unwrap_or(0);

                    // Back button
                    if selected == 0 {
                        app.mode = AppMode::Normal;
                        return;
                    }

                    // Load from file
                    if selected == 1 {
                        // TODO
                        return;
                    }

                    // New
                    if selected == 2 {
                        // TODO
                        return;
                    }

                    let configuration_index = selected - 3;

                    if let Some(configurations) = &hardware.configurations {
                        let configuration = configurations[configuration_index].clone();
                        state.configurations_state = None;
                        state.selected_configuration = Some(configuration.clone());
                        state.selected_configuration_state = Some(ListState::default());
                        state
                            .selected_configuration_state
                            .as_mut()
                            .unwrap()
                            .select_next();

                        send_command(
                            &app.socket,
                            CommandPacketData {
                                acknowledged: false,
                                command: REQUEST_CONFIGURATION.to_string(),
                                data: configuration.name,
                                timestamp: get_timestamp_nanos(),
                            },
                            app.shared_network_data.clone(),
                        )
                        .await;
                    }
                }
                _ => {}
            }
        }

        if let Some(selected_list_state) = state.selected_configuration_state.as_mut() {
            match (key.modifiers, key.code) {
                (_, KeyCode::BackTab) | (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                    selected_list_state.select_previous();
                }

                (_, KeyCode::Tab) | (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                    selected_list_state.select_next();
                }
                (_, KeyCode::Enter) => {
                    let selected = selected_list_state.selected().unwrap_or(0);

                    match selected {
                        // Edit configuration name
                        0 => {}

                        // Back
                        1 => {
                            state.selected_configuration = None;
                            state.selected_configuration_state = None;
                            state.configurations_state = Some(ListState::default());
                            state.configurations_state.as_mut().unwrap().select_next();
                            return;
                        }

                        // Save
                        2 => {}

                        // Edit
                        3 => {}

                        // Export to file
                        4 => {}

                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
