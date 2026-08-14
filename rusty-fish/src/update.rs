use ratatui::crossterm::event::{KeyCode, KeyEvent,};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        // Match keycode::eventtype here to specific functions, ie
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('w') | KeyCode::Up => app.list_state.select_previous(),
        KeyCode::Char('s') | KeyCode::Down => app.list_state.select_next(),
        // Do nothing if the key is not in the above list
        _ => {}
    };
}