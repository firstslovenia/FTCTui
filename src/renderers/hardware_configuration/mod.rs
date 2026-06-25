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
    renderers::{
        create_list_state,
        hardware_configuration::{
            HardwareConfigurationUIState::{Choosing, InMenu},
            editing::EditingData,
            in_menu::InMenuData,
        },
        styles::{
            MUTED_TEXT_COLOR, SELECTED_BACKGROUND, SUCCESS_COLOR, TEXT_COLOR, block_style,
            muted_text_style, text_style,
        },
    },
};

pub mod choosing;
pub mod editing;
pub mod in_menu;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HardwareConfigurationUIState {
    Choosing(ListState),
    Importing(ListState),
    InMenu(InMenuData),
    Editing(EditingData),
}

impl HardwareConfigurationUIState {
    pub fn new_choosing() -> HardwareConfigurationUIState {
        HardwareConfigurationUIState::Choosing(create_list_state())
    }

    pub fn new_in_menu(config: RobotConfigurationFile) -> HardwareConfigurationUIState {
        HardwareConfigurationUIState::InMenu(InMenuData {
            config,
            list_state: create_list_state(),
            config_data: None,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HardwareConfigurationUI {
    state: HardwareConfigurationUIState,
}

impl Default for HardwareConfigurationUI {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select_next();

        HardwareConfigurationUI {
            state: HardwareConfigurationUIState::new_choosing(),
        }
    }
}

impl HardwareConfigurationUI {
    /// Renders the hardware configuration UI, assuming it's open
    pub fn render(app: &mut App, frame: &mut Frame<'_>) {
        let AppMode::ConfigureHardware(ui) = &mut app.mode else {
            return;
        };

        let block = Block::bordered()
            .title("Hardware Configurations")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let robot = futures::executor::block_on(app.robot.read());
        let hardware = futures::executor::block_on(robot.hardware.read());
        let mut list_to_render = None;
        let mut max_length = 0;
        let mut max_height = 0;

        // Selecting a configuration
        if let HardwareConfigurationUIState::Choosing(config_list_state) = &ui.state {
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

            // This doesn't work?
            for item in &items {
                if item.width() > max_length {
                    max_length = item.width();
                }
            }
            max_height = items.len();

            let list = List::new(items);
            list_to_render = Some(list);
        }

        // Menu for one specific configuration
        if let HardwareConfigurationUIState::InMenu(state) = ui.state.clone() {
            let list_state = state.list_state;
            let mut items: Vec<ListItem> = Vec::new();

            items.push(ListItem::new(Span::styled(
                state.config.name,
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
            list_to_render = Some(list);
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

        match &mut ui.state {
            Choosing(state) => {
                frame.render_stateful_widget(
                    list_to_render.as_ref().unwrap(),
                    block_inner_area,
                    state,
                );
            }
            InMenu(data) => {
                frame.render_stateful_widget(
                    list_to_render.as_ref().unwrap(),
                    block_inner_area,
                    &mut data.list_state,
                );
            }
            _ => {}
        }
    }

    /// Handles inputs when in the UI
    pub async fn on_key_event(app: &mut App, key: KeyEvent) {
        let AppMode::ConfigureHardware(ui) = &mut app.mode else {
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

        if let HardwareConfigurationUIState::Choosing(state) = &mut ui.state {
            match (key.modifiers, key.code) {
                (_, KeyCode::BackTab) | (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                    state.select_previous();
                }

                (_, KeyCode::Tab) | (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                    state.select_next();
                }
                (_, KeyCode::Enter) => {
                    let selected = state.selected().unwrap_or(0);

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
                        ui.state = HardwareConfigurationUIState::new_in_menu(configuration.clone());

                        send_command(
                            &app.socket,
                            CommandPacketData::from_command_and_data(
                                REQUEST_CONFIGURATION,
                                configuration.name,
                            ),
                            app.shared_network_data.clone(),
                        )
                        .await;
                    }
                }
                _ => {}
            }
        }

        if let HardwareConfigurationUIState::InMenu(state) = &mut ui.state {
            let list_state = &mut state.list_state;

            match (key.modifiers, key.code) {
                (_, KeyCode::BackTab) | (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                    list_state.select_previous();
                }

                (_, KeyCode::Tab) | (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                    list_state.select_next();
                }
                (_, KeyCode::Enter) => {
                    let selected = list_state.selected().unwrap_or(0);

                    match selected {
                        // Edit configuration name
                        0 => {}

                        // Back
                        1 => {
                            ui.state = HardwareConfigurationUIState::new_choosing();
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
