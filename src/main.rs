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
use enums::window_type::WindowType;
use event::{Event, EventHandler};
use ftail::Ftail;
use log::LevelFilter;
use ratatui::{Terminal, backend::CrosstermBackend};
use structs::{app::App, fishing_minigame::FishingMinigame, player::Player};
use tui::Tui;
use update::{update, update_fishing_game};

fn main() -> Result<()> {
    let project_directory = ProjectDirs::from("", "deranged-lunatic-apps", "rusty-fish").unwrap();
    directory_functions::make_project_directories(&project_directory);

    let log_directory = project_directory.data_dir().join("logs");
    Ftail::new()
        .daily_file(&log_directory, LevelFilter::Debug)
        .retention_days(7)
        .init()?;

    // Create an application.
    let mut app = App::new();
    app.set_window_type(WindowType::Main);

    directory_functions::update_or_make_data(&project_directory, &app);

    // Create a new player (ill change this later im just lazy)
    let mut player = Player::new();
    let mut fishing_minigame = FishingMinigame::new(&project_directory, &app, &player);
    let menu_windows: Vec<WindowType> = vec![
        WindowType::Main,
        WindowType::VictorySceen,
        WindowType::StandardMenu,
    ];

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(25);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {

        let _ = match app.window {
            WindowType::Main => tui.draw_main_menu(&mut app),
            WindowType::VictorySceen => tui.draw_victory_screen(&mut app, &fishing_minigame, &mut player),
            WindowType::Fishing => {
                let _: () = while app.window == WindowType::Fishing {
                    if app.has_window_changed {
                        fishing_minigame = FishingMinigame::new(&project_directory, &app, &player);
                        let _ = tui.draw_fishing_menu(&mut app, &mut fishing_minigame); // Here so it displays initally, without it the user needs to press a key.
                        app.set_has_window_changed(false);
                    }
                    match tui.events.next()? {
                        Event::Tick => {}
                        Event::Key(key_event) => {
                            update_fishing_game(&mut fishing_minigame, key_event, &mut app);
                            let _ = tui.draw_fishing_menu(&mut app, &mut fishing_minigame);
                        }
                        Event::Mouse(_) => {}
                        Event::Resize(_, _) => {}
                    } 
                };
                Ok(())
            },
            WindowType::StandardMenu => tui.draw_standard_menu(&mut app, &player),
            _ => unimplemented!()
        };

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, &player, key_event, &menu_windows),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
