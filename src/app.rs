use ratatui::widgets::ListState;

#[derive(Debug, Default, PartialEq)]
pub enum WindowType {
    #[default] Main,
    Fishing
}

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub version: String,
    pub name: String,
    pub window: WindowType,
    // Core menus list_state
    pub list_state: ListState,
    // Menus to display in the List
    pub list_items: Vec<String>,
    // Typed text for the fishing game
    pub typed_text: Vec<char>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            version: "Super duper beta".to_owned(),
            name: "Rusty Fish".to_owned(),
            window: WindowType::Main,
            list_state: ListState::default().with_selected(Some(0)),
            list_items: Vec::new(),
            typed_text: Vec::new(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        log::info!("Setting app.should_quit to true");
        self.should_quit = true;
    }

    pub fn fish(&mut self) {
        log::info!("Setting the current window to 'Fishing'");
        self.window = WindowType::Fishing;
    }
    pub fn set_main_menu_vec(&mut self) {
        // Sets the vector items to be the menu for the main menu
        self.list_items = vec!["Fish", "Some other button", "Another button"]
            .into_iter()
            .map(String::from)
            .collect();
    }
    pub fn set_victory_screen_vec(&mut self) {
        self.list_items = vec!["Fish again", "Return to main menu"]
            .into_iter()
            .map(String::from)
            .collect();
    }
    pub fn insert_text(&mut self, character: char) {
        self.typed_text.push(character);
    }
    pub fn pop_typed_text(&mut self) {
        self.typed_text.pop();
    }
}
