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

pub mod structs;

pub mod enums;

pub mod logic;

use color_eyre::Result;
use directories::ProjectDirs;
use enums::{inventory_item::InventoryItem, window_type::WindowType};
use event::{Event, EventHandler};
use ftail::Ftail;
use log::LevelFilter;
use logic::fishing_logic;
use ratatui::{Terminal, backend::CrosstermBackend};
use structs::{app::App, player::Player, fish::Fish};
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

    // Create an application.
    let mut app = App::new();

    directory_functions::update_or_make_data(&project_directory, &app);

    // Create a new player (ill change this later im just lazy)
    let mut player = Player::new();

    let menu_windows: Vec<WindowType> = vec![WindowType::Main, WindowType::VictorySceen];

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(25);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut words = fishing_logic::get_random_words(&project_directory, &player);
    // Start the main loop.
    while !app.should_quit {
        // Render the main user interface.
        if app.window == WindowType::Main {
            let _ = tui.draw_main_menu(&mut app);
        } else if app.window == WindowType::Fishing {
            let _ = tui.draw_fishing_menu(&mut app, words.clone());
        } else if app.window == WindowType::VictorySceen {
            if app.has_window_changed {
                words = fishing_logic::get_random_words(&project_directory, &player);
                let catch = fishing_logic::get_random_fish(&project_directory, &app);
                app.clear_typed_text();
                player.add_to_inventory(InventoryItem::InvFish(catch.clone()
        ));
                player.last_caught_fish = catch;
                app.set_has_window_changed(false);
            }
            let _ = tui.draw_victory_screen(&mut app, &player);
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
