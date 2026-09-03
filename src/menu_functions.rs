use crate::{enums::window_type::WindowType, structs::app::App};

pub fn do_action(selection: usize, app: &mut App) {
    let action = &app.list_items[selection];
    log::info!(
        "User selected choice #{} which correspondend to {}",
        selection,
        action
    );
    log::info!("Here is the entire list: {:?}", app.list_items);
    if app.window == WindowType::Main {
        if action == "Fish" {
            app.set_window_type(WindowType::Fishing);
        }
    } else if app.window == WindowType::VictorySceen {
        if action == "Fish again" {
            app.set_window_type(WindowType::Fishing);
        } else if action == "Return to main menu" {
            app.set_window_type(WindowType::Main);
        }
    }
}
