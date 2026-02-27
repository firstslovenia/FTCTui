use ratatui::widgets::Paragraph;

use crate::app::App;

/// A modal / popup to be shown on the ui
pub trait Popup: Send + Sync + std::fmt::Debug {
    /// Returns the title of the popup to show
    fn title(&self) -> String;

    /// Returns the main text of the popup to show
    fn text(&self) -> Paragraph<'_>;

    /// Returns the options the user can submit to the popup
    fn options(&self) -> Vec<String>;

    /// Submits the current selected option and closes the popup
    fn submit(&mut self, app: &mut App);

    /// Returns which option is selected currently
    fn selected_option(&self) -> u8;

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
    fn title(&self) -> String {
        String::from("Alert (press enter to close)")
    }

    fn text(&self) -> Paragraph<'_> {
        self.text.clone().scroll((self.scroll, 0))
    }

    fn options(&self) -> Vec<String> {
        vec!["Ok".to_string()]
    }

    fn submit(&mut self, _app: &mut App) {
        // Do nothing special, the calling app will close us
    }

    fn selected_option(&self) -> u8 {
        0
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

#[derive(Clone, PartialEq, Eq, Debug)]
/// A popup which asks the user if they want to restart the robot
pub struct RestartRobotPopup {
    pub selected_yes: bool,
}

impl Popup for RestartRobotPopup {
    fn title(&self) -> String {
        String::from("Restart Robot?")
    }

    fn text(&self) -> Paragraph<'_> {
        Paragraph::new("Are you sure you want to restart the robot?")
    }

    fn options(&self) -> Vec<String> {
        vec!["Yes".to_string(), "No".to_string()]
    }

    fn submit(&mut self, app: &mut App) {
        if self.selected_yes {
            // Ehhhhhhh
            futures::executor::block_on(app.restart_robot());
        }
    }

    fn selected_option(&self) -> u8 {
        if self.selected_yes { 0 } else { 1 }
    }

    fn select_next_option(&mut self) {
        self.selected_yes = !self.selected_yes;
    }

    fn select_previous_option(&mut self) {
        self.selected_yes = !self.selected_yes;
    }

    // No scroll, we just have a little bit of text
    fn scroll_up(&mut self) {}
    fn scroll_down(&mut self) {}
}
