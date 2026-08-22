use crate::app::App;

pub fn do_action(selection: usize, app: &mut App) {
    let action = &app.list_items[selection];
    log::info!(
        "User selected choice #{} which correspondend to {}",
        selection,
        action
    );
    log::info!("Here is the entire list: {:?}", app.list_items);
    if action == "Fish" {
        app.fish();
    }
}
