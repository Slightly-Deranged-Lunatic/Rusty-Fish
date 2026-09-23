use ratatui::style::{Color, Modifier, Style};
use std::env::consts::OS;

pub fn list_style() -> Style {
    let style: Style = Style::new().fg(Color::Magenta);

    style
}

pub fn main_text_style() -> Style {
    let style: Style = Style::new().fg(Color::Magenta);

    style
}

pub fn correct_character_style() -> Style {
    let mut style: Style = Style::reset()
        .fg(Color::Magenta)
        .add_modifier(Modifier::UNDERLINED);

    // Here because KDE plasma's Konsole worked better like this, without it then the underlined colors would be all wrong
    if OS == "linux" {
        style = style.underline_color(Color::Magenta);
    }

    style
}

pub fn incorrect_character_style() -> Style {
    let mut style: Style = Style::reset()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);

    // Here because KDE plasma's Konsole worked better like this, without it then the underlined colors would be all wrong
    if OS == "linux" {
        style = style.underline_color(Color::Red);
    }

    style
}

pub fn untyped_character_style() -> Style {
    let style: Style = Style::new().fg(Color::DarkGray);

    style
}
