use ratatui::{
    text::Line,
    widgets::{Block, Borders, BorderType},
    style::{Color, Style,},
    Frame
};

use crate::App;

pub fn render_border(frame: &mut Frame, app: &mut App, instructions: &str) {
    // Renders the border with instructions at the bottom
    let instructions = Line::from(instructions);
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