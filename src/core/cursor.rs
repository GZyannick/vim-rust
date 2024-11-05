use std::{u16, usize};

// TODO! See if when max width we let always the cursor at the end of the line or
// we move it one time and after it will stay at the same position
#[derive(Debug)]
pub struct VimCursor {
    pub x: u16,
    pub y: u16,
}

impl VimCursor {
    pub fn new() -> VimCursor {
        VimCursor { x: 0, y: 0 }
    }

    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    pub fn down(&mut self, max_height: usize) {
        if max_height > self.y as usize {
            self.y += 1;
        }
    }
    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn right(&mut self, max_width: usize) {
        if max_width > self.x as usize + 1 {
            self.x += 1;
        }
    }

    pub fn handle_max_width(&mut self, max_width: usize) {
        let max_width = max_width as u16 - 1;
        if self.x > max_width {
            self.x = max_width;
        }
    }
}
