use ratatui::{
    Frame, crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, layout::{Constraint, Flex, Layout}, widgets::{Block, ListState, Padding}
};

use crate::{
    app::{App, AppMode},
    renderers::styles::block_style,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HardwareConfigurationUI {
    pub configurations_state: Option<ListState>,
}

impl Default for HardwareConfigurationUI {
    pub fn default() -> Self {
        HardwareConfigurationUI {
            configurations_state: Some(ListState::default()),
        }
    }
}

impl HardwareConfigurationUI {
    /// Renders the hardware configuration UI, assuming it's open
    pub fn render(app: &mut App, frame: &mut Frame<'_>) {
        let AppMode::ConfigureHardware(state) = app.mode else {
            return;
        };

        let block = Block::bordered()
            .title("Hardware Configurations")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let mut horizontal = Layout::horizontal([Constraint::Percentage(30)]).flex(Flex::Center);
        let mut vertical = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);

        // Build it to test the width..
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let block_inner_area = block.inner(area);

        let wanted_width = popup.text().line_width() as u16;
        if wanted_width > block_inner_area.width {
            // Too big, do 90%
            if wanted_width > frame.area().width {
                horizontal = Layout::horizontal([Constraint::Percentage(75)]).flex(Flex::Center);
            } else {
                horizontal =
                    Layout::horizontal([Constraint::Length(wanted_width + 6)]).flex(Flex::Center);
            }
        }
    }

    /// Handles inputs when in the UI
    pub async fn on_key_event(app: &mut App, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Quit handler
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit().await,
            // Go back to normal mode
            (_, KeyCode::Esc) => app.mode = AppMode::Normal,
            _ => {}
        }
    }
}
