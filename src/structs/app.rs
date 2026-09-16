use crate::enums::{biomes::Biome, window_type::WindowType};
use ratatui::widgets::ListState;

/// Application.
#[derive(Debug)]
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
            list_items: Vec::new(),
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
        self.list_items = vec!["Fish", "View Inventory", "Quit"]
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

    pub fn set_custom_vec(&mut self, mut vec: Vec<String>) {
        vec.push("Return to main menu".to_string());
        self.list_items = vec;
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
        log::info!("Set the current window to {:?}", self.window)
    }

    pub fn set_has_window_changed(&mut self, status: bool) {
        if status {
            self.has_window_changed = true
        } else {
            self.has_window_changed = false
        }
        // Always make sure the list state starts at the top
        self.list_state.select_first();
    }
}
