use std::{error::Error, u16, usize};

use crossterm::event::{self, Event, KeyCode};

use crate::{app::App, ui::screen::command};

use super::{CurrentScreen, LineLen, Modes};

pub struct AppKeyHandler {
    pub buffered_input: Option<KeyCode>,
}

//TODO Handle vim key,
//Handle buffered_input
//
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

            // break if user do :q
            if self.handle_navigation(app, key.code) {
                return Ok(true);
            }
            self.handle_screen(app, key.code)?;
            self.buffer_input(key.code)
        }
        Ok(false)
    }

    fn handle_screen(&self, app: &mut App, key: KeyCode) -> Result<(), Box<dyn Error>> {
        match app.current_screen {
            CurrentScreen::Explorer => match key {
                KeyCode::Enter if app.modes != Modes::Command => {
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

    fn handle_navigation(&self, app: &mut App, key: KeyCode) -> bool {
        match app.modes {
            Modes::Normal | Modes::Visual => match key {
                // TODO! when up and down move the cursor.x to its max width if x > max_width
                KeyCode::Esc => app.modes = Modes::Normal,
                KeyCode::Char(':') => {
                    app.modes = Modes::Command;
                }
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
            Modes::Insert => match key {
                KeyCode::Esc => app.modes = Modes::Normal,
                _ => {}
            },
            Modes::Command => match key {
                KeyCode::Esc => {
                    app.modes = Modes::Normal;
                    app.command_input = String::new();
                }
                KeyCode::Char(char) => {
                    app.command_input.push(char);
                }
                KeyCode::Backspace => {
                    if app.command_input.len() == 0 {
                        app.modes = Modes::Normal;
                    } else {
                        app.command_input.pop().unwrap();
                    }
                }
                KeyCode::Enter => {
                    let is_break = self.handle_command_result(&app.command_input);
                    app.command_input = String::new();
                    return is_break;
                }
                _ => {}
            },
        }
        // to know if we need to break the loop
        false
    }

    fn buffer_input(&mut self, input: KeyCode) {
        self.buffered_input = Some(input);
    }

    fn handle_command_result(&self, command: &str) -> bool {
        match command {
            "q" => return true,
            _ => {}
        }
        false
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
