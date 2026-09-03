use crate::enums::{biomes::Biome, window_type::WindowType};
use ratatui::widgets::ListState;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    pub version: String,
    pub name: String,
    pub window: WindowType,
    pub has_window_changed: bool,
    // Core menus list_state
    pub list_state: ListState,
    // Menus to display in the List
    pub list_items: Vec<String>,
    // Typed text for the fishing game
    pub typed_text: Vec<char>,
    pub current_biome: Biome,
}
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            version: "0.9.0 ALPHA".to_owned(),
            name: "Rusty Fish".to_owned(),
            window: WindowType::Main,
            has_window_changed: false,
            list_state: ListState::default().with_selected(Some(0)),
            list_items: vec!["Fish", "Some other button", "Another button"].into_iter().map(String::from).collect(), // Default main menu vec
            typed_text: Vec::new(),
            current_biome: Biome::BackyardPond,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        log::info!("Setting app.should_quit to true");
        self.should_quit = true;
    }

    fn set_main_menu_vec(&mut self) {
        // Sets the vector items to be the menu for the main menu
        self.list_items = vec!["Fish", "Some other button", "Another button"]
            .into_iter()
            .map(String::from)
            .collect();
    }

    fn set_victory_screen_vec(&mut self) {
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

    pub fn clear_typed_text(&mut self) {
        self.typed_text.clear();
    }

    pub fn set_window_type(&mut self, window: WindowType) {
        log::info!("Setting the current window to {:?}", window);
        if window == WindowType::Main {
            self.set_main_menu_vec();
        } else if window == WindowType::VictorySceen {
            self.set_victory_screen_vec();
        }
        self.set_has_window_changed(true);
        self.window = window;
    }

    pub fn set_has_window_changed(&mut self, status: bool) {
        if status {
            self.has_window_changed = true
        } else  {
            self.has_window_changed = false
        }
    }
}
