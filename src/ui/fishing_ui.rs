use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, ListItem, Paragraph, Wrap},
};

use crate::{
    App, WindowType,
    logic::fishing_logic,
    structs::fishing_minigame::FishingMinigame,
    ui::{general_ui, menu_ui, ui_styles},
};

fn render_fishing_text(frame: &mut Frame, app: &mut App, fishing_minigame: &mut FishingMinigame) {
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

    let test_width = (horizontal_layout[1].width - 2) as usize; // the 2 is used up by the border
    let mut lines_vec: Vec<Line> = Vec::new();
    let mut current_line: Line = Line::default();
    let mut character_span: Span = Span::default();
    let mut current_word: Vec<Span> = Vec::new();

    // Make the lines
    for (index, character) in fishing_minigame.words.iter().enumerate() {
        // Assign spans to the characters
        if fishing_minigame.typed_text.get(index).is_none() {
            character_span =
                Span::styled(character.to_string(), ui_styles::untyped_character_style());
        } else if fishing_minigame.typed_text[index] == *character {
            character_span =
                Span::styled(character.to_string(), ui_styles::correct_character_style());
        } else if fishing_minigame.typed_text[index] != *character {
            character_span = Span::styled(
                character.to_string(),
                ui_styles::incorrect_character_style(),
            );
        }

        current_word.push(character_span.clone());

        let is_last_char = index + 1 == fishing_minigame.words.len(); // Used to prevent the last word from getting cut off

        // Space represents the end of a word
        if *character != ' ' && !is_last_char {
            continue;
        }

        if current_line.iter().len() + current_word.len() > test_width {
            lines_vec.push(current_line.clone());
            current_line = Line::default();
            continue;
        }
        for span in &current_word {
            current_line.push_span(span.to_owned());
        }
        if is_last_char {
            lines_vec.push(current_line.clone());
        }
        current_word.clear();
    }

    if fishing_minigame.words.len() == fishing_minigame.typed_text.len() {
        fishing_logic::calculate_statistics(fishing_minigame);
        app.set_window_type(WindowType::VictorySceen);
        return;
    }

    let text = Text::from(lines_vec.clone());

    let text = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: (true) });

    // Line control stuff
    if fishing_minigame.position_in_line as usize
        == lines_vec
            .get(fishing_minigame.current_line)
            .unwrap()
            .iter()
            .len()
    {
        fishing_minigame.current_line += 1;
        fishing_minigame.position_in_line = 0;
    }
    // If user is trying to go back a line
    else if fishing_minigame.position_in_line < 0
        && !fishing_minigame.current_line.checked_sub(1).is_none()
    {
        fishing_minigame.current_line -= 1;
        fishing_minigame.position_in_line = lines_vec
            .get(fishing_minigame.current_line)
            .unwrap()
            .iter()
            .len() as i32
            - 1
        // -1 because .len() and fishing_minigame.position_in_line need to be the same, .len doesnt start at 0 basically
    }

    // makes the position in line 0 becuase you would get a negative position if you backspace at the start and it would mess up auto scroll
    if fishing_minigame.position_in_line < 0 {
        fishing_minigame.position_in_line = 0;
    }

    frame.render_widget(
        text.scroll((fishing_minigame.current_line as u16, 0)),
        horizontal_layout[1],
    );
}

pub fn render_victory_screen(app: &mut App, fishing_minigame: &FishingMinigame, frame: &mut Frame) {
    let list_items = app
        .list_items
        .clone()
        .into_iter()
        .map(ListItem::new)
        .collect();

    let catch_text = vec![
        Line::from(format!("You caught a {}", fishing_minigame.catch.name)),
        Line::from(format!("Words per minute: {:.0}", fishing_minigame.wpm)),
        Line::from(format!("Accuracy: {:.2}", fishing_minigame.accuracy)),
    ];
    let catch_text = Paragraph::new(catch_text)
        .style(ui_styles::main_text_style())
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
    frame.render_widget(catch_text, horizontal_layout[1]);
}

pub fn render_fishing_ui(app: &mut App, frame: &mut Frame, fishing_minigame: &mut FishingMinigame) {
    let instructions = "Type the text on screen";
    general_ui::render_border(frame, app, instructions);
    render_fishing_text(frame, app, fishing_minigame);
}
