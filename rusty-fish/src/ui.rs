use ratatui::{
    Frame, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, text::Line, widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
};

use crate::app::App;

fn render_border(frame: &mut Frame, app: &mut App, instructions: Line) {
    // Renders the border with instructions at the bottom
    let title = Line::from(format!("{} version {}", app.name, app.version));
    let area = frame.area();
    let border = Block::default()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .border_type(BorderType::Thick);
    frame.render_widget(border, area);
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

fn render_fishing_text(frame: &mut Frame) {
    let text: Vec<Line<'_>> = vec![
        "This is some example text!".into(),
        "Look! It's even got a new line!".into(),
    ];
    let vertical_layout = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(100),
        Constraint::Percentage(20),
    ])
    .split(frame.area());

    let horizontal_layout = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(100),
        Constraint::Percentage(20),
    ])
    .split(vertical_layout[1]);
    frame.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL)), horizontal_layout[1]);
}

pub fn render_standard_menu(app: &mut App, frame: &mut Frame, list_items: Vec<ListItem>) {
    let instructions = Line::from("Use up or down to change selection, press enter to select");
    render_border(frame, app, instructions);
    render_list(frame, list_items, app);
}

pub fn render_fishing_ui(app: &mut App, frame: &mut Frame) {
    let instructions = Line::from("Type the text on screen");
    render_border(frame, app, instructions);
    render_fishing_text(frame);
}