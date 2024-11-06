use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub struct Command;

impl Command {
    pub fn new(frame: &mut Frame, app: &mut App) {
        let area = Command::centered_rect(30, 8, frame.area());

        let block = Block::default()
            .title("Command")
            .borders(Borders::all())
            .style(Style::default().fg(ratatui::style::Color::LightBlue));

        let input = Paragraph::new(app.command_input.clone()).block(block);
        frame.render_widget(input, area);
    }

    fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(rect);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
