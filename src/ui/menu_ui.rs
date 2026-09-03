use ratatui::{
    Frame,
    style::Color,
    widgets::{Block, List, ListItem, Padding},
};

use crate::{App, ui::general_ui::render_border};
pub fn render_standard_menu(app: &mut App, frame: &mut Frame, list_items: Vec<ListItem>) {
    let instructions = "Use up or down to change selection, press enter to select";
    render_border(frame, app, instructions);
    render_list(frame, list_items, app);
}

fn render_list(frame: &mut Frame, list_items: Vec<ListItem>, app: &mut App) {
    // Render the list
    let padding_block = Block::new().padding(Padding::new(2, 2, 2, 2));
    let list = List::new(list_items)
        .style(Color::Magenta)
        .highlight_symbol(">")
        .block(padding_block);
    frame.render_stateful_widget(list, frame.area(), &mut app.list_state);
}
