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
    postion: usize,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        Ok(Self {
            stdout: stdout().into_raw_mode()?,
            postion: 4,
        })
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn write(&mut self, msg: &str) -> Result<(), std::io::Error> {
        writeln!(self.stdout, "{}\r", msg)
    }

    pub fn show(&mut self) -> Result<(), std::io::Error> {
        write!(self.stdout, "{}", termion::cursor::Show)
    }

    pub fn clean(&mut self) -> Result<(), std::io::Error> {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            // termion::cursor::Hide
        )
    }

    pub fn clean_line(&mut self) -> Result<(), std::io::Error> {
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(1, 1),
            // termion::clear::CurrentLine
        )
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_position(&mut self, cursor_move: CursorMove) {
        match cursor_move {
            CursorMove::Up => self.postion -= 1,
            CursorMove::Down => self.postion += 1,
        }
        let position = self.postion.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(0, position));
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
