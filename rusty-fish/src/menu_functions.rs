use crate::app::App;
use crate::tui::Tui;

pub fn do_action(selection: usize, app: &mut App) {
    let action = & app.list_items[selection];
    if action == "Fish" {
        app.fish();
    }
}
