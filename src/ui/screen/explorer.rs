use ratatui::{
    layout::Position,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::App;
//pub struct Cursor;
//
//impl Cursor {
//    pub fn normal(app: &mut App, frame: &mut Frame) {}
//}
pub struct Explorer;

impl Explorer {
    pub fn new(app: &mut App, frame: &mut Frame) {
        //TODO ADD LINE NUMBER SYSTEM
        //BLINK CURSOR
        let mut text: Vec<Line> = vec![];
        for line in &app.lines {
            let line = match line.eq("../") {
                true => line.to_string(),
                false => line.replace("./", ""),
            };
            text.push(Line::from(Span::styled(line, Style::default())));
        }
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, frame.area());
        frame.set_cursor_position(Position::new(app.cursor.0, app.cursor.1));
    }
}
