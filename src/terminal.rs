use std::io::{self, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub enum CursorMove {
    Up,
    Down,
}

// #[derive(Clone)]
pub struct Terminal {
    x: u16,
    y: u16,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let (x, y) = stdout().into_raw_mode().unwrap().cursor_pos().unwrap();
        Ok(Self { x, y })
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn write(&mut self, msg: &str) {
        print!("{}\r", msg);
    }

    pub fn show(&mut self) {
        print!("{}", termion::cursor::Show);
    }

    pub fn clean(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1),);
    }

    pub fn clean_line(&mut self) {
        print!("{}", termion::cursor::Goto(1, 1),);
    }

    pub fn clean_after_cursor(&mut self) {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn reset_cursor_position(&mut self) {
        print!("{}", termion::cursor::Goto(self.x, self.y));
    }

    pub fn cursor_position(&mut self, position: usize) {
        let p = position.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(0, p));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn read_line() -> String {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Cannot read from terminal");
        line
    }

    pub fn save_postion(&mut self) {}
}
