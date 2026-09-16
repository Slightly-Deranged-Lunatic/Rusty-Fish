use ratatui::style::{Style, Color, Modifier};

pub fn list_style() -> Style {
    const LIST_STYLE: Style  = Style::new()
    .fg(Color::Magenta);
    
    return LIST_STYLE;
}

pub fn main_text_style() -> Style {
    const MAIN_TEXT_STYLE: Style = Style::new()
    .fg(Color::Magenta);

    return MAIN_TEXT_STYLE;
}

pub fn correct_character_style() -> Style {
    const CORRECT_CHARACTER_STYLE: Style = Style::new()
    .fg(Color::Magenta)
    .underline_color(Color::Magenta)
    .add_modifier(Modifier::UNDERLINED);

    return CORRECT_CHARACTER_STYLE;
}

pub fn incorrect_character_style() -> Style {
    const INCORRECT_CHARACTER_STYLE: Style = Style::new()
    .fg(Color::Red)
    .underline_color(Color::Red)
    .add_modifier(Modifier::UNDERLINED);

    return INCORRECT_CHARACTER_STYLE;
}

pub fn untyped_character_style() -> Style {
    const UNTYPED_CHARACTER_STYLE: Style = Style::new()
    .fg(Color::Gray);

    return UNTYPED_CHARACTER_STYLE;
}

