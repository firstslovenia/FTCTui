use std::sync::Arc;

use ratatui::widgets::Paragraph;
use tokio::sync::Mutex;

use crate::{app::App, popup::InfoPopup};

impl App {
    /// Executes a command buffer
    pub async fn submit_command(&mut self, command_buffer: String) {
        let mut args = Vec::new();

        let command_string = command_buffer.trim();

        let mut argument_buffer = String::new();
        let mut literal_start: Option<char> = None;

        for char in command_string.chars() {
            // We are in a literal
            if let Some(literal_starting_char) = literal_start {
                if char != literal_starting_char {
                    argument_buffer.push(char);
                    continue;
                }

                literal_start = None;
                args.push(argument_buffer.clone());
                argument_buffer.clear();
                continue;
            }

            // Seperate args by whitespace
            if char.is_whitespace() {
                if !argument_buffer.is_empty() {
                    args.push(argument_buffer.clone());
                    argument_buffer.clear();
                }
                continue;
            }

            if char == '\'' || char == '"' {
                if !argument_buffer.is_empty() {
                    args.push(argument_buffer.clone());
                    argument_buffer.clear();
                }

                literal_start = Some(char);
                continue;
            }

            argument_buffer.push(char);
        }

        if !argument_buffer.is_empty() {
            args.push(argument_buffer);
        }

        let command = args[0].to_lowercase();

        match command.as_str() {
            "quit" | "q" | "q!" | "wq" | "wqa" => {
                self.quit().await;
            }
            "showtoast" | "echo" => {
                let mut string = String::new();

                for arg in args.into_iter().enumerate() {
                    // Skip the name of the command
                    if arg.0 == 0 {
                        continue;
                    }

                    string.push_str(&arg.1);
                    string.push(' ');
                }

                self.show_toast(string);
            }
            "initopmode" => {
                if args.len() < 2 {
                    self.show_toast("Expected name of opmode to initialize".to_string());
                    return;
                }

                self.init_opmode(args[1].clone()).await;
            }
            "startopmode" => {
                if args.len() < 2 {
                    self.show_toast("Expected name of opmode to start".to_string());
                    return;
                }

                self.start_opmode(args[1].clone()).await;
            }
            "stopopmode" | "stop" => {
                self.stop_opmode().await;
            }
            _ => {
                self.show_toast(format!("Invalid or incomplete command {}", command));
                return;
            }
        }
    }

    /// Shows some text as a popup
    pub fn show_toast(&mut self, text: String) {
        self.active_popup = Some(Arc::new(Mutex::new(InfoPopup {
            text: Paragraph::new(text).wrap(ratatui::widgets::Wrap { trim: false }),
            scroll: 0,
        })));
    }
}
