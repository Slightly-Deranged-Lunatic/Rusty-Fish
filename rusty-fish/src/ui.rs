use ratatui::{
    layout::Alignment,
    style::{Stylize},
    widgets::{Block, BorderType, Borders, List, ListDirection},
    text::Line,
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let list_items = ["Fish", "View shop", "Other option 1", "Other option 2",];
    let title = Line::from("Rusty Fish");
    frame.render_widget(
        List::new(list_items)
            .block(
                Block::default()
                    .title(title.bold())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_symbol(">")
            .direction(ListDirection::TopToBottom),
        frame.area(),
    )
}