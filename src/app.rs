use ratatui::{layout::Position, prelude::Backend, Terminal};
use std::{
    error::Error,
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
};

use crate::{
    core::{AppKeyHandler, AppLines, CurrentScreen, LineLen, Modes, VimCursor},
    ui::screen::{editor::Editor, explorer::Explorer},
};

#[derive(Debug)]
pub struct App {
    pub modes: Modes,
    pub current_screen: CurrentScreen,
    pub path: PathBuf,
    pub lines: Vec<AppLines>,
    pub cursor: VimCursor,
}

impl App {
    pub fn new() -> Result<App, Box<dyn Error>> {
        let (path, screen, lines) = match std::env::args().nth(1) {
            Some(path) => {
                let path = PathBuf::from(path);
                App::get_path_data(path)?
            }
            None => (std::env::current_dir()?, CurrentScreen::Empty, Vec::new()),
        };
        Ok(App {
            modes: Modes::Normal,
            current_screen: screen,
            cursor: VimCursor::new(),
            lines,
            path,
        })
    }

    pub fn get_path_data(
        path: PathBuf,
    ) -> Result<(PathBuf, CurrentScreen, Vec<AppLines>), Box<dyn Error>> {
        //TODO transform vec of string and pathbuf to enum to handle it in file tooo
        let mut lines: Vec<AppLines> = vec![];
        if path.is_dir() {
            if let Some(parent) = path.parent() {
                lines.push(AppLines::Explorer((
                    "../".to_string(),
                    PathBuf::from(parent),
                )));
            }
            for entry in path.read_dir()? {
                let path = entry?.path();
                let mut line = path.to_str().unwrap().to_string();
                match line.split("/").last() {
                    Some(str) => line = format!("./{str}"),
                    None => {}
                };

                lines.push(AppLines::Explorer((line, path)));
            }

            return Ok((path, CurrentScreen::Explorer, lines));
        } else if path.is_file() {
            let mut buffer = String::new();
            let mut file = File::open(path.clone())?;
            file.read_to_string(&mut buffer)?;
            let split_lines: Vec<&str> = buffer.split_inclusive("\n").collect();

            for split_line in split_lines {
                lines.push(AppLines::File(split_line.to_string()));
            }

            return Ok((path, CurrentScreen::Editor, lines));
        } else {
            return Ok((path, CurrentScreen::Empty, vec![]));
        }
    }

    pub fn handle_new_path(&mut self, path: Option<AppLines>) -> Result<(), Box<dyn Error>> {
        if let Some(app_path) = path {
            match app_path {
                AppLines::Explorer((_, pathbuf)) => {
                    let (path, screen, lines) = App::get_path_data(pathbuf)?;
                    self.path = path;
                    self.lines = lines;
                    self.current_screen = screen;
                    self.cursor = VimCursor::new();
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        let mut input = AppKeyHandler::new();
        loop {
            terminal.draw(|frame| {
                match self.current_screen {
                    CurrentScreen::Empty => {
                        //ui::screen::empty::Empty::new(frame, self);
                        todo!()
                    }
                    CurrentScreen::Editor => match Editor::render(self, frame) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{e}");
                        }
                    },
                    CurrentScreen::Explorer => {
                        Explorer::render(self, frame);
                    }
                }
                frame.set_cursor_position(Position::new(self.cursor.x, self.cursor.y));
            })?;

            if input.handle(self)? == true {
                return Ok(());
            }
        }
    }

    //TODO! see if its preferable to put it at 0 or return Option<usize>
    pub fn get_line_len(&self) -> usize {
        match self.lines.iter().nth(self.cursor.y as usize) {
            Some(line) => line.len(),
            None => 0,
        }
    }
}
