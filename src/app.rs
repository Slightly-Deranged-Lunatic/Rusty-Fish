use ratatui::widgets::ListState;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub version: String,
    pub name: String,
    pub window: String,
    // Core menus list_state
    pub list_state: ListState,
    // Menus to display in the List
    pub list_items: Vec<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            version: "Super duper beta".to_owned(),
            name: "Rusty Fish".to_owned(),
            window: "Main".to_owned(),
            list_state: ListState::default().with_selected(Some(0)),
            list_items: Vec::new(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn fish(&mut self) {
        self.window = "Fishing".to_owned();
    }
    pub fn set_main_menu_vec(&mut self) {
        // Sets the vector items to be the menu for the main menu
        self.list_items = vec!["Fish", "Some other button", "Another button"]
            .into_iter()
            .map(String::from)
            .collect();
    }
}
