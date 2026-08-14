use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line,},
    widgets::{Block, BorderType, Borders, List, Padding},
};

use crate::app::App;

fn render_border(frame: &mut Frame, app: &mut App) {
    // Renders the border with instructions at the bottom
    let title = Line::from(format!("{} version {}", app.name, app.version));
    let instructions = Line::from("Use up or down to change selection, press enter to select");
    let area = frame.area();
    let border = Block::default()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .border_type(BorderType::Thick);
    frame.render_widget(border, area);
}

fn render_list(frame: &mut Frame, list_items: Vec<&str>, app: &mut App) {
    // Render the list
    let padding_block = Block::new().padding(Padding::new(2, 2, 2, 2));
    let list = List::new(list_items)
        .style(Color::Magenta)
        .highlight_symbol(">")
        .block(padding_block);
    frame.render_stateful_widget(list, frame.area(), &mut app.list_state);
}

pub fn render_standard_menu(app: &mut App, frame: &mut Frame, list_items: Vec<&str>) {
    render_border(frame, app);
    render_list(frame, list_items, app);
}