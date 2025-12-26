use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, Padding},
    Frame,
};

use crate::{
    app::App,
    renderers::styles::{block_style, MUTED_TEXT_COLOR, SELECTED_BACKGROUND, TEXT_COLOR},
};

// Indexes of each quickmenu option
pub const QUICKMENU_OPTION_CLOSE_QUICKMENU: usize = 0;
pub const QUICKMENU_OPTION_RESTART_ROBOT: usize = 1;
pub const QUICKMENU_OPTION_TOGGLE_MATCH: usize = 2;
pub const QUICKMENU_OPTION_EXIT_APP: usize = 3;

/// How many options the quickmenu has
pub const QUICKMENU_OPTIONS_NUM: usize = 4;

// Text of each quickmenu option
pub const QUICKMENU_TEXT_CLOSE: &str = "Close Quickmenu";
pub const QUICKMENU_TEXT_RESTART: &str = "Restart Robot";
pub const QUICKMENU_TEXT_START_MATCH: &str = "Start Match";
pub const QUICKMENU_TEXT_STOP_MATCH: &str = "Stop Match";
pub const QUICKMENU_TEXT_EXIT_APP: &str = "Exit";

impl App {
    /// Renders a popup
    pub fn render_popup_if_any(&self, frame: &mut Frame<'_>) {
        let Some(popup) = self.active_popup.clone() else {
            return;
        };

        let popup = futures::executor::block_on(popup.lock());

        let block = Block::bordered()
            .title(popup.title())
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

        let should_render_options = popup.options().len() > 1;

        // Test the height..
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let block_inner_area = block.inner(area);

        let lines = popup
            .text()
            .line_count(block_inner_area.width.saturating_sub(4)) as u16;

        if lines > frame.area().height {
            vertical = Layout::vertical([Constraint::Percentage(75)]).flex(Flex::Center);
        } else {
            // 5: 4 to just fit, +1 for the options
            let mut height = lines + 4;

            if should_render_options {
                height += 1;
            }

            vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
        }

        // Build options
        let mut options = Vec::new();

        if should_render_options {
            for (i, option) in popup.options().into_iter().enumerate() {
                if i > 0 {
                    options.push(Span::raw(" "));
                }

                if popup.selected_option() == i as u8 {
                    options.push(Span::styled(
                        option,
                        Style::default().bg(SELECTED_BACKGROUND).fg(TEXT_COLOR),
                    ));
                } else {
                    options.push(Span::styled(option, Style::default().fg(MUTED_TEXT_COLOR)));
                }
            }
        }

        // Build it properly
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let block_inner_area = block.inner(area);

        let inner_layout = if should_render_options {
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(block_inner_area)
        } else {
            Layout::vertical([Constraint::Fill(1)]).split(block_inner_area)
        };

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        frame.render_widget(Clear, block_inner_area);
        frame.render_widget(popup.text(), inner_layout[0]);

        if should_render_options {
            frame.render_widget(Clear, inner_layout[1]);
            frame.render_widget(Line::from(options), inner_layout[1]);
        }
    }

    /// Renders a quick context menu, if it is open
    pub fn render_quickmenu_if_open(&mut self, frame: &mut Frame<'_>) {
        let Some(state) = self.quickmenu_state else {
            return;
        };

        let block = Block::bordered()
            .title("Quickmenu")
            .border_style(block_style())
            .padding(Padding::new(2, 2, 1, 1));

        let mut quickmenu_options = Vec::new();

        for i in 0..QUICKMENU_OPTIONS_NUM {
            match i {
                QUICKMENU_OPTION_CLOSE_QUICKMENU => quickmenu_options.push(QUICKMENU_TEXT_CLOSE),
                QUICKMENU_OPTION_RESTART_ROBOT => quickmenu_options.push(QUICKMENU_TEXT_RESTART),
                QUICKMENU_OPTION_TOGGLE_MATCH => {
                    // TODO: check if a match is active
                    quickmenu_options.push(QUICKMENU_TEXT_START_MATCH);
                }
                QUICKMENU_OPTION_EXIT_APP => quickmenu_options.push(QUICKMENU_TEXT_EXIT_APP),
                _ => {}
            }
        }

        let mut items: Vec<ListItem> = Vec::new();

        for i in 0..quickmenu_options.len() {
            let option = quickmenu_options[i];

            let selected = state.selected().unwrap_or_default() == i;

            let style = if selected {
                Style::new().fg(TEXT_COLOR).bg(SELECTED_BACKGROUND)
            } else {
                Style::new().fg(TEXT_COLOR)
            };

            items.push(ListItem::new(Span::styled(option, style)));
        }

        let lines_needed = quickmenu_options.len();
        let mut columns_needed = 0;

        for option in quickmenu_options.iter() {
            if option.len() > columns_needed {
                columns_needed = option.len();
            }
        }

        let vertical =
            Layout::vertical([Constraint::Length(lines_needed as u16 + 4)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Length(columns_needed as u16 + 6)]).flex(Flex::Center);

        // Build it properly
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(
            List::new(items).block(block),
            area,
            self.quickmenu_state.as_mut().unwrap(),
        );
    }
}
