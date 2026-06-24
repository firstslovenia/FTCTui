
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Flex, Layout},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Clear, List, ListItem, ListState, Padding},
};

use crate::{
    app::{App, AppMode},
    renderers::styles::{
        MUTED_TEXT_COLOR, SELECTED_BACKGROUND, SUCCESS_COLOR, TEXT_COLOR, block_style,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HardwareConfigurationUI {
    pub configurations_state: Option<ListState>,
}

impl Default for HardwareConfigurationUI {
    fn default() -> Self {
        HardwareConfigurationUI {
            configurations_state: Some(ListState::default()),
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

        let mut render_list = None;
        let mut max_length = 0;
        let mut max_height = 0;

        // Selecting a configuration
        if let Some(config_list_state) = state.configurations_state {
            let mut items: Vec<ListItem> = Vec::new();
            let robot = futures::executor::block_on(app.robot.read());
            let hardware = futures::executor::block_on(robot.hardware.read());

            items.push(ListItem::new(Span::styled(
                "Back",
                Style::new().fg(MUTED_TEXT_COLOR),
            )));
            items.push(ListItem::new(Span::styled(
                "Load from file",
                Style::new().fg(MUTED_TEXT_COLOR),
            )));

            if let Some(configurations) = &hardware.configurations {
                for i in 0..configurations.len() {
                    let mut selected_config = configurations[i].clone();

                    let mut style = Style::new().fg(TEXT_COLOR);

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

            if items.len() == 0 {
                items.push(ListItem::new(Span::styled(
                    "<No configurations available>",
                    Style::new().fg(TEXT_COLOR),
                )));
            }

            for item in &items {
                if item.width() > max_length {
                    max_length = item.width();
                }
            }
            max_height = items.len();

            let list = List::new(items);
            render_list = Some(list);
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
        if let Some(list) = render_list {
            frame.render_stateful_widget(
                list,
                block_inner_area,
                state.configurations_state.as_mut().unwrap(),
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

        if let Some(configurations_list_state) = state.configurations_state.as_mut() {
            match (key.modifiers, key.code) {
                (_, KeyCode::BackTab) | (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                    configurations_list_state.select_previous();
                }

                (_, KeyCode::Tab) | (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                    configurations_list_state.select_next();
                }
                _ => {}
            }
        }
    }
}
