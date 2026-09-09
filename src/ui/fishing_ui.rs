use ratatui::{
    Frame,
    layout::{Constraint, Layout, Alignment},
    style::{Color, Style, Modifier, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, ListItem, Paragraph, Wrap, Scrollbar},
};

use crate::{App, WindowType, structs::{fishing_minigame::FishingMinigame, player::Player}, ui::{general_ui, menu_ui}, logic::fishing_logic};

fn make_span(character: &char, color: Color) -> Span<'static> {
    return Span::styled(character.to_string(), Style::default().fg(color));
}

fn render_fishing_text(frame: &mut Frame, app: &mut App, fishing_minigame: &mut FishingMinigame) {
    let mut character_span_vec: Vec<Span> = Vec::new();
    let untyped_color = Color::DarkGray;
    let correct_color = Color::Magenta;
    let incorrect_color = Color::Red;

    for (index, character) in fishing_minigame.words.iter().enumerate() {
        if fishing_minigame.typed_text.get(index).is_none() {
            character_span_vec.push(make_span(character, untyped_color));
        } else if fishing_minigame.typed_text[index] == *character {
            character_span_vec.push(make_span(character, correct_color).add_modifier(Modifier::UNDERLINED).light_magenta());
        } else if fishing_minigame.typed_text[index] != *character {
            character_span_vec.push(make_span(character, incorrect_color).add_modifier(Modifier::UNDERLINED).light_red());
        }
    }

    if fishing_minigame.words.len() == fishing_minigame.typed_text.len() {
        fishing_logic::calculate_statistics(fishing_minigame);
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

pub fn render_victory_screen(app: &mut App, fishing_minigame: &FishingMinigame, player: &Player, frame: &mut Frame) {
    let list_items = app
        .list_items
        .clone()
        .into_iter()
        .map(ListItem::new)
        .collect();
    let catch_text = vec![
        Line::from(format!("You caught a {}", player.last_caught_fish.name)),
        Line::from(format!("Words per minute: {:.2}", fishing_minigame.wpm)),
        Line::from(format!("Accuracy: {:.2}", fishing_minigame.accuracy))
        ];
    let catch_text = Paragraph::new(catch_text)
        .alignment(Alignment::Center);

    let vertical_layout = Layout::vertical([
        Constraint::Percentage(5),
        Constraint::Percentage(100),
        Constraint::Percentage(80),
    ])
    .split(frame.area());

    let horizontal_layout = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
        Constraint::Percentage(20),
    ])
    .split(vertical_layout[1]);

    menu_ui::render_standard_menu(app, frame, list_items);
    frame.render_widget(
            catch_text,
        horizontal_layout[1],
    );
}

pub fn render_fishing_ui(app: &mut App, frame: &mut Frame, fishing_minigame: &mut FishingMinigame) {
    let instructions = "Type the text on screen";
    general_ui::render_border(frame, app, instructions);
    render_fishing_text(frame,  app, fishing_minigame);
}
