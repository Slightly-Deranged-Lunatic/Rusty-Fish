use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{enums::window_type::WindowType, menu_functions, structs::app::App};

pub fn update(app: &mut App, key_event: KeyEvent, menu_windows: &Vec<WindowType>) {
    if menu_windows.contains(&app.window) {
        match key_event.code {
            // Match keycode::eventtype here to specific functions, ie
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('w') | KeyCode::Up => app.list_state.select_previous(),
            KeyCode::Char('s') | KeyCode::Down => app.list_state.select_next(),
            KeyCode::Enter => {
                let selection = app.list_state.selected().unwrap();
                menu_functions::do_action(selection, app);
            }
            // Do nothing if the key is not in the above list
            _ => {}
        }
    } else if app.window == WindowType::Fishing {
        match key_event.code {
            KeyCode::Backspace => app.pop_typed_text(),
            _ => match key_event.code.as_char() {
                Some(character) => app.insert_text(character),
                None => log::info!("Failed to convert {} to a char", key_event.code),
            },
        }
    }
}
