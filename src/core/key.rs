use std::{error::Error, u16, usize};

use crossterm::event::{self, Event, KeyCode};

use crate::app::App;

use super::{CurrentScreen, LineLen, Modes};

pub struct AppKeyHandler {
    pub buffered_input: Option<KeyCode>,
}

//-> Result<(), Box<dyn Error>>
impl AppKeyHandler {
    pub fn new() -> Self {
        Self {
            buffered_input: None,
        }
    }
    pub fn handle(&mut self, app: &mut App) -> Result<bool, Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(false);
            }
            if key.code == KeyCode::Char('q') && app.modes == Modes::Normal {
                return Ok(true);
            }

            self.handle_navigation(app, key.code);
            self.handle_screen(app, key.code)?;
            self.buffer_input(key.code)
        }
        Ok(false)
    }

    fn handle_screen(&self, app: &mut App, key: KeyCode) -> Result<(), Box<dyn Error>> {
        match app.current_screen {
            CurrentScreen::Explorer => match key {
                KeyCode::Enter => {
                    let path = app.lines.iter().nth(app.cursor.y as usize);
                    app.handle_new_path(path.cloned())?;
                }
                _ => {}
            },
            CurrentScreen::Editor => {}
            CurrentScreen::Empty => {}
        }
        Ok(())
    }

    fn handle_navigation(&self, app: &mut App, key: KeyCode) {
        match app.modes {
            Modes::Normal | Modes::Visual => match key {
                // TODO! when up and down move the cursor.x to its max width if x > max_width
                KeyCode::Up | KeyCode::Char('k') => {
                    app.cursor.up();
                    app.cursor.handle_max_width(app.get_line_len());
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.cursor.down(app.lines.len());
                    app.cursor.handle_max_width(app.get_line_len());
                }
                KeyCode::Left | KeyCode::Char('h') => app.cursor.left(),
                KeyCode::Right | KeyCode::Char('l') => app.cursor.right(app.get_line_len()),
                _ => {}
            },
            Modes::Insert => {}
            Modes::Command => {}
        }
    }

    fn buffer_input(&mut self, input: KeyCode) {
        self.buffered_input = Some(input);
    }
}

//                if key.code == KeyCode::Char('q') {
//                    return Ok(());
//                }
//
//                if let CurrentScreen::Explorer = self.current_screen {
//                    match key.code {
//                        KeyCode::Enter => {
//                            let path = self.lines.iter().nth(self.cursor.1 as usize);
//                            self.handle_new_path(path.cloned())?;
//                        }
//                        _ => {}
//                    }
//                }
//
//                if let Modes::Normal = self.modes {
//match key.code {
//TODO! Ajouter un buffer de la derniere touche appuyer pour handle
//                        //les commandes types gg
//                        //
//                        //
//                        //Vim input normal Mode
//                        KeyCode::Char('G') => {
//                            self.cursor.1 = self.lines.len() as u16 - 1;
//                        }
//                        _ => {}
//                    }
//                }
//            }
