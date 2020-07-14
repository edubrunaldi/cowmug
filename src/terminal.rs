use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub enum CursorMove {
    Up,
    Down,
}

pub struct Terminal {
    stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        Ok(Self {
            stdout: stdout().into_raw_mode()?,
        })
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

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
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
}
