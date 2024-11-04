use std::{error::Error, fs, io::Stdout, path::PathBuf, u16, usize};
mod ui;
use ui::screen::explorer::Explorer;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    self,
    prelude::{Backend, CrosstermBackend},
    Terminal,
};

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

#[derive(Debug)]
pub enum Modes {
    Normal,
    Visual,
    Insert,
    Command,
}

#[derive(Debug)]
pub enum CurrentScreen {
    File,
    Explorer,
    Empty,
}

#[derive(Debug)]
pub struct App {
    pub modes: Modes,
    pub current_screen: CurrentScreen,
    pub path: PathBuf,
    pub lines: Vec<String>,
    pub cursor: (u16, u16),
}

impl App {
    pub fn new() -> Result<App, Box<dyn Error>> {
        let (path, screen, lines) = App::get_path_data(std::env::args().nth(1))?;
        Ok(App {
            modes: Modes::Normal,
            current_screen: screen,
            cursor: (0, 0),
            lines,
            path,
        })
    }

    fn get_path_data(
        path: Option<String>,
    ) -> Result<(PathBuf, CurrentScreen, Vec<String>), Box<dyn Error>> {
        let path_info: (PathBuf, CurrentScreen, Vec<String>) = match path {
            Some(path) => {
                let path = PathBuf::from(path);
                if path.is_dir() {
                    let mut lines: Vec<String> = Vec::from(["../".into()]);
                    for entry in path.read_dir()? {
                        let path = entry?.path();
                        lines.push(path.to_str().unwrap().into());
                    }
                    (path, CurrentScreen::Explorer, lines)
                } else if path.is_file() {
                    (path, CurrentScreen::File, Vec::new())
                } else {
                    (path, CurrentScreen::Empty, Vec::new())
                }
            }
            None => (std::env::current_dir()?, CurrentScreen::Empty, Vec::new()),
        };
        Ok(path_info)
    }

    fn handle_new_path(&mut self, path: Option<String>) -> Result<(), Box<dyn Error>> {
        let (path, screen, lines) = App::get_path_data(path)?;
        self.path = path;
        self.lines = lines;
        self.current_screen = screen;
        self.cursor = (0, 0);
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|frame| {
                match self.current_screen {
                    CurrentScreen::Empty => {
                        //ui::screen::empty::Empty::new(frame, self);
                        todo!()
                    }
                    CurrentScreen::File => {
                        //ui::screen::empty::Empty::new(frame, self);
                        todo!()
                    }
                    CurrentScreen::Explorer => {
                        Explorer::new(self, frame);
                    }
                }
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                //DEV PURPOSE : q to leave
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                if let CurrentScreen::Explorer = self.current_screen {
                    match key.code {
                        KeyCode::Enter => {
                            let path = self.lines.iter().nth(self.cursor.1 as usize);
                            self.handle_new_path(path.cloned())?;
                        }
                        KeyCode::Right | KeyCode::Char('h') => {
                            if self.cursor.0 > 0 {
                                self.cursor.0 -= 1;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('j') => {
                            if self.cursor.1 < self.lines.len() as u16 {
                                self.cursor.1 += 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('k') => {
                            if self.cursor.1 > 0 {
                                self.cursor.1 -= 1;
                            }
                        }
                        KeyCode::Left | KeyCode::Char('l') => {
                            self.cursor.0 += 1; //TODO find how to stop the line by the string, size
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let backend = enter_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    app?.run(&mut terminal)?;
    leave_raw_mode(&mut terminal)?;
    Ok(())
}

// TODO
// handle args
//  create file, empty, explorer
