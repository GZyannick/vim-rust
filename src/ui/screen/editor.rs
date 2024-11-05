use std::{error::Error, fs::File, io::Read};

use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::App;

pub struct Editor {}

impl Editor {
    pub fn render(app: &mut App, frame: &mut Frame) -> Result<(), Box<dyn Error>> {
        let mut content: Vec<Line> = vec![];
        for line in &app.lines {
            content.push(Line::from(Span::styled(line.to_string(), Style::default())));
        }

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, frame.area());
        Ok(())
    }
}
