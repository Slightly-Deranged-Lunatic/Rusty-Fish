use ratatui::{
    Frame,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders},
};

use crate::{App, ui::ui_styles};

pub fn render_border(frame: &mut Frame, app: &mut App, instructions: &str) {
    // Renders the border with instructions at the bottom
    let instructions = Line::from(instructions);
    let title = Line::from(format!("{} version {}", app.name, app.version));
    let area = frame.area();
    let border = Block::default()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .borders(Borders::ALL)
        .border_style(ui_styles::main_text_style())
        .border_type(BorderType::Thick);
    frame.render_widget(border, area);
}
