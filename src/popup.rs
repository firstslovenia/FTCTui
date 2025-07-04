use ratatui::widgets::Paragraph;

use crate::app::App;

/// A modal / popup to be shown on the ui
pub trait Popup: Send + Sync + std::fmt::Debug {
    /// Returns the main text of the popup to show
    fn text(&self) -> Paragraph;

    /// Returns the options the user can submit to the popup
    fn options(&self) -> Vec<String>;

    /// Submits the current selected option and closes the popup
    fn submit(&mut self, app: &mut App);

    /// Selects the next option, wrapping around
    fn select_next_option(&mut self);

    /// Selects the previous option, wrapping around
    fn select_previous_option(&mut self);

    /// Moves the paragraph's scroll 1 more down
    fn scroll_down(&mut self);
    fn scroll_up(&mut self);
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// A popup which shows some text to the user, with only the Ok / Close option
pub struct InfoPopup<'a> {
    pub text: Paragraph<'a>,
    pub scroll: u16,
}

impl<'a> Popup for InfoPopup<'a> {
    fn text(&self) -> Paragraph {
        self.text.clone().scroll((self.scroll, 0))
    }

    fn options(&self) -> Vec<String> {
        vec!["Ok".to_string()]
    }

    fn submit(&mut self, _app: &mut App) {
        // Do nothing special, the calling app will close us
    }

    fn select_next_option(&mut self) {
        // Only one option
    }

    fn select_previous_option(&mut self) {
        // Only one option
    }

    fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
}
