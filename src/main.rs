/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

// Main menu
pub mod menu_functions;

pub mod directory_functions;

pub mod player;

use app::{App, WindowType};
use color_eyre::Result;
use directories::ProjectDirs;
use event::{Event, EventHandler};
use ftail::Ftail;
use log::LevelFilter;
use player::Player;
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let project_directory = ProjectDirs::from("", "Deranged Lunatic Apps", "rusty-fish").unwrap();
    directory_functions::make_project_directories(&project_directory);

    let log_directory = project_directory.data_dir().join("logs");
    Ftail::new()
        .daily_file(&log_directory, LevelFilter::Debug)
        .retention_days(7)
        .init()?;

    if directory_functions::should_download_words_list(&project_directory) {
        directory_functions::download_words_list(&project_directory);
    }

    // Create an application.
    let mut app = App::new();

    // Create a new player (ill change this later im just lazy)
    let mut player = Player::new();

    let menu_windows: Vec<WindowType> = vec![WindowType::Main, WindowType::VictorySceen];

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(25);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let words = menu_functions::get_random_words(&project_directory, &player);
    // Start the main loop.
    while !app.should_quit {
        // Render the main user interface.
        if app.window == WindowType::Main {
            let _ = tui.draw_main_menu(&mut app);
        } else if app.window == WindowType::Fishing {
            let _ = tui.draw_fishing_menu(&project_directory, &mut player, &mut app, words.clone());
        } else if app.window == WindowType::VictorySceen {
            app.clear_typed_text();
            let _ = tui.draw_victory_screen(&mut player, &mut app);
        }
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event, &menu_windows),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

// Cleanup stuff
// When the WindowType is Victory it should get a new set of words
// Structs directory
    // You won't believe what its gonna hold
// Enums directoy
    // You won't believe what its gonna hold
// Cleanup all the imports
    // Importing code I wrote should go together based off of what it is
        // Ie all struct imports together, all functions i made together, etc
// Cleanup functions to remove unused arguements