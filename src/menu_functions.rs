use crate::{
    enums::window_type::WindowType,
    structs::{app::App, player::Player},
};

pub fn do_action(selection: usize, app: &mut App, player: &Player) {
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
        } else if action == "Quit" {
            app.quit();
        } else if action == "View Inventory" {
            app.set_custom_vec(player.get_inventory_as_vec());
            app.set_window_type(WindowType::StandardMenu);
        }
    } else if app.window == WindowType::VictorySceen {
        if action == "Fish again" {
            app.set_window_type(WindowType::Fishing);
        } else if action == "Return to main menu" {
            app.set_window_type(WindowType::Main);
        }
    } else if app.window == WindowType::StandardMenu {
        if action == "Return to main menu" {
            app.set_window_type(WindowType::Main);
        }
    }
}
