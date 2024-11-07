use super::{CurrentScreen, Modes};
use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

pub struct AppKeyHandler {
    pub buffered_input: Option<KeyCode>,
}

//-> Result<(), Box<dyn Error>>

// !!! TODO LIST !!!
//  Handle specific command in normal, insert, command, visual
//  Ajouter un buffer de la derniere touche appuyer pour handle
//  find a way to handle general behavior like moving in each modes
//  Error! the movement key can crash because the cursor go more line than possible

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
            match app.modes {
                // handle ESC here because it will be on each Modes
                Modes::Visual | Modes::Insert | Modes::Command if key.code == KeyCode::Esc => {
                    app.modes = Modes::Normal;
                    if app.command_input.len() > 0 {
                        app.command_input = String::new();
                    }
                }

                Modes::Visual | Modes::Normal if key.code == KeyCode::Char(':') => {
                    app.modes = Modes::Command;
                }
                Modes::Normal => self.handle_normal(key.code, app),
                Modes::Visual => self.handle_visual(key.code, app),
                Modes::Insert => self.handle_insert(key.code, app),
                Modes::Command => {
                    let is_quit = self.handle_command(key.code, app);
                    if is_quit {
                        return Ok(is_quit);
                    }
                }
            };
            if app.modes != Modes::Command {
                self.handle_screen(app, key.code)?;
            }
            self.buffer_input(key.code)
        }
        Ok(false)
    }

    fn handle_normal(&mut self, key: KeyCode, app: &mut App) {
        self.hjkl_navigation(app, key);
        //self.arrows_navigation(app, key);
        //match key {
        //    _ => {}
        //}
    }

    fn handle_insert(&mut self, key: KeyCode, app: &mut App) {
        self.arrows_navigation(app, key);
        //match key {
        //    _ => {}
        //}
    }
    fn handle_visual(&mut self, key: KeyCode, app: &mut App) {
        self.hjkl_navigation(app, key);
        //self.arrows_navigation(app, key);
        //match key {
        //    _ => {}
        //}
    }

    fn handle_command(&mut self, key: KeyCode, app: &mut App) -> bool {
        match key {
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
                let is_break = self.command_result(&app.command_input);
                app.command_input = String::new();
                return is_break;
            }
            _ => {}
        }
        false
    }

    fn hjkl_navigation(&self, app: &mut App, key: KeyCode) {
        // TODO! Error cursor can go more than the number of line and crash the app
        match key {
            KeyCode::Char('k') => {
                app.cursor.up();
                app.cursor.handle_max_width(app.get_line_len());
            }
            KeyCode::Char('j') => {
                app.cursor.down(app.lines.len());
                app.cursor.handle_max_width(app.get_line_len());
            }
            KeyCode::Char('h') => app.cursor.left(),
            KeyCode::Char('l') => app.cursor.right(app.get_line_len()),
            _ => {}
        }
    }

    fn arrows_navigation(&self, app: &mut App, key: KeyCode) {
        match key {
            KeyCode::Up => {
                app.cursor.up();
                app.cursor.handle_max_width(app.get_line_len());
            }
            KeyCode::Down => {
                app.cursor.down(app.lines.len());
                app.cursor.handle_max_width(app.get_line_len());
            }
            KeyCode::Left => app.cursor.left(),
            KeyCode::Right => app.cursor.right(app.get_line_len()),
            _ => {}
        }
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

    fn buffer_input(&mut self, input: KeyCode) {
        self.buffered_input = Some(input);
    }

    fn command_result(&self, command: &str) -> bool {
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
