use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{
    app::App,
    network::NetworkStatus,
    r#match::Match,
    renderers::styles::{ERROR_COLOR, MUTED_TEXT_COLOR, SUCCESS_COLOR, TEXT_COLOR, WARNING_COLOR},
};

impl App {
    /// Creates the debug text
    pub fn create_debug_paragraph(&mut self) -> Paragraph {
        let mut debug_text: Vec<Line> = Vec::new();

        let shared_network_read = futures::executor::block_on(self.shared_network_data.read());

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

        debug_text.push(Line::from(""));
        self.add_active_match_to_lines(&mut debug_text);

        Paragraph::new(debug_text).wrap(Wrap { trim: false })
    }

    /// Adds text to display info about the active match to a vector of lines
    fn add_active_match_to_lines(&self, line_vec: &mut Vec<Line>) {
        let Some(active_match) = self.active_match else {
            return;
        };

        if active_match.start.elapsed() > Match::length() + std::time::Duration::from_secs(20) {
            return;
        }

        let phase = active_match.phase();
        let remaining_in_phase = active_match.remaining_in_phase();
        let elapsed = active_match.start.elapsed();

        line_vec.push(Line::from(vec![
            Span::styled(format!("{:?}", phase), Style::new().fg(SUCCESS_COLOR)),
            Span::styled(" - ", Style::new().fg(MUTED_TEXT_COLOR)),
            Span::styled(
                format!("{:.1?} left", remaining_in_phase),
                Style::new().fg(TEXT_COLOR),
            ),
            Span::styled(
                format!(" ({:.1?} elapsed)", elapsed),
                Style::new().fg(MUTED_TEXT_COLOR),
            ),
        ]));
        line_vec.push(Line::from(""));
    }
}
