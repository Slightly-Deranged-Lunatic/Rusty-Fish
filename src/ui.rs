use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap},
};

use crate::{app::App, app::WindowType, player::Player};

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

fn make_span(character: &char, color: Color) -> Span<'static> {
    return Span::styled(
        character.to_string(),
        Style::default().fg(color));
}

fn render_fishing_text(frame: &mut Frame, words: Vec<char>, app: &mut App) {
    let mut character_span_vec: Vec<Span> = Vec::new();
    let untyped_color = Color::DarkGray;
    let correct_color = Color::Magenta;
    let incorrect_color = Color::Red;


    for (index, character) in words.iter().enumerate() {
        if app.typed_text.get(index).is_none() {
            character_span_vec.push(make_span(character, untyped_color));
        } else if app.typed_text[index] == *character {
            character_span_vec.push(make_span(character, correct_color));
        } else if app.typed_text[index] != *character {
            character_span_vec.push(make_span(character, incorrect_color));
        }
    }

    if words.len() == app.typed_text.len() {
        app.set_window_type(WindowType::VictorySceen);
    }

    let text = Text::from(Line::from(character_span_vec));

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

    frame.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: (true) }),
        horizontal_layout[1],
    );
}

pub fn render_victory_screen(app: &mut App, frame: &mut Frame, player: &mut Player) {
    let instructions = Line::from("Use up or down to change selection, press enter to select");
    render_border(frame, app, instructions);
    app.set_victory_screen_vec();
    let list_items = app
        .list_items
        .clone()
        .into_iter()
        .map(ListItem::new)
        .collect();
    render_list(frame, list_items, app);
}

pub fn render_standard_menu(app: &mut App, frame: &mut Frame, list_items: Vec<ListItem>) {
    let instructions = Line::from("Use up or down to change selection, press enter to select");
    render_border(frame, app, instructions);
    render_list(frame, list_items, app);
}

pub fn render_fishing_ui(app: &mut App, frame: &mut Frame, words: Vec<char>, player: &mut Player) {
    let instructions = Line::from("Type the text on screen");
    render_border(frame, app, instructions);
    render_fishing_text(frame, words.clone(), app);
}
