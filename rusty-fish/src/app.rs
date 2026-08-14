use ratatui::widgets::ListState;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub version: String,
    pub name: String,
    /// Here because keycodes shouldn't do their typical actions if the user is fishing
    pub is_fishing: bool,
    // Core menus list_state
    pub list_state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            version: "Super duper beta".to_owned(),
            name: "Rusty Fish".to_owned(),
            is_fishing: false,
            list_state: ListState::default().with_selected(Some(0))
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

}