use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Clear, Padding},
};

use crate::{
    app::App,
    renderers::styles::{MUTED_TEXT_COLOR, SELECTED_BACKGROUND, TEXT_COLOR, block_style},
};

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
                    Layout::horizontal([Constraint::Length(wanted_width + 4)]).flex(Flex::Center);
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
}
