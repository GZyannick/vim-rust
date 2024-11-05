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
        let mut buf = String::new();
        let mut file = File::open(&app.path)?;
        file.read_to_string(&mut buf)?;
        let lines: Vec<&str> = buf.split_inclusive("\n").collect();
        let mut content: Vec<Line> = vec![];
        for line in lines {
            content.push(Line::from(Span::styled(line, Style::default())));
        }

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, frame.area());
        Ok(())
    }
}
