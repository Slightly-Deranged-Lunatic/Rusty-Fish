use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    enums::window_type::WindowType,
    menu_functions,
    structs::{
        app::App,
        fishing_minigame::FishingMinigame,
        player::{self, Player},
    },
};

pub fn update(
    app: &mut App,
    fishing_minigame: &mut FishingMinigame,
    player: &Player,
    key_event: KeyEvent,
    menu_windows: &Vec<WindowType>,
) {
    if menu_windows.contains(&app.window) {
        match key_event.code {
            // Match keycode::eventtype here to specific functions, ie
            KeyCode::Char('w') | KeyCode::Up => app.list_state.select_previous(),
            KeyCode::Char('s') | KeyCode::Down => app.list_state.select_next(),
            KeyCode::Enter => {
                let selection = app.list_state.selected().unwrap();
                log::info!("{}", selection);
                menu_functions::do_action(selection, app, player);
            }
            // Do nothing if the key is not in the above list
            _ => {}
        }
    } else if app.window == WindowType::Fishing {
        match key_event.code {
            KeyCode::Backspace => fishing_minigame.pop_typed_text(),
            _ => match key_event.code.as_char() {
                Some(character) => fishing_minigame.insert_text(character),
                None => log::info!("Failed to convert {} to a char", key_event.code),
            },
        }
    }
}
