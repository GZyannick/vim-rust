use ratatui::{
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
    pub fn render(app: &mut App, frame: &mut Frame) {
        //TODO ADD LINE NUMBER SYSTEM
        let mut text: Vec<Line> = vec![];
        for (line, _) in &app.lines {
            //let line = match line.eq("../") {
            //    true => line,
            //    false => line.split("/").last().unwrap(),
            //};
            text.push(Line::from(Span::styled(line, Style::default())));
        }
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, frame.area());
    }
}
