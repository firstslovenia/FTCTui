use ratatui::style::{Color, Style};

// https://coolors.co/ff9147-ffa970-bdae93-ebdbb2-e02f29-ffba49-b8bb26
// The colors are heavily inspired by gruvbox: https://github.com/morhetz/gruvbox?tab=readme-ov-file

pub const PRIMARY_COLOR: Color = Color::from_u32(0xFFA970);
pub const PRIMARY_COLOR_LIGHTER: Color = Color::from_u32(0xFF9147);

pub const SELECTED_BACKGROUND: Color = Color::from_u32(0x665C54);

pub const TEXT_COLOR: Color = Color::from_u32(0xEBDBB2);

pub const MUTED_TEXT_COLOR: Color = Color::from_u32(0xBDAE93);

pub const ERROR_COLOR: Color = Color::from_u32(0xE02F29);

pub const WARNING_COLOR: Color = Color::from_u32(0xFFBA49);

pub const SUCCESS_COLOR: Color = Color::from_u32(0xB8BB26);

pub fn block_style() -> Style {
	Style::new().fg(PRIMARY_COLOR)
}

pub fn selected_block_style() -> Style {
	Style::new().fg(PRIMARY_COLOR_LIGHTER)
}
