use std::{error::Error, io::Stdout};
mod app;
use app::App;
mod core;
mod ui;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{self, prelude::CrosstermBackend, Terminal};

fn enter_raw_mode() -> Result<CrosstermBackend<Stdout>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(CrosstermBackend::new(stdout))
}

fn leave_raw_mode(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let backend = enter_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    let app = App::new();
    app?.run(&mut terminal)?;
    leave_raw_mode(&mut terminal)?;
    Ok(())
}

// TODO
// handle args
//  create file, empty, explorer
