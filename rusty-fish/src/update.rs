use ratatui::crossterm::event::{KeyCode, KeyEvent,};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        // Match keycode::eventtype here to specific functions, ie
        KeyCode::Char('q') => app.quit(),

        // Do nothing if the key is not in the above list
        _ => {}
    };
}