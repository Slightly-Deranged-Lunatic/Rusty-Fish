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

use app::App;
use color_eyre::Result;
use directories::ProjectDirs;
use event::{Event, EventHandler};
use ftail::Ftail;
use log::LevelFilter;
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

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        log::info!(
            "App.window is currently {}, selecting the respective action",
            { &app.window }
        );
        // Render the main user interface.
        if app.window == "Main" {
            let _ = tui.draw_main_menu(&mut app);
        } else if app.window == "Fishing" {
            let _ = tui.draw_fishing_menu(&mut app);
        }
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
