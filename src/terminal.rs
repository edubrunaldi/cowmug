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
    stdout: Option<RawTerminal<io::Stdout>>,
    x: u16,
    y: u16,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        Ok(Self {
            x: 0,
            y: 0,
            stdout: None,
        })
    }

    pub fn hold_stdout(&mut self) {
        self.stdout = Some(stdout().into_raw_mode().unwrap());
        let (x, y) = stdout().into_raw_mode().unwrap().cursor_pos().unwrap();
        self.x = x;
        self.y = y;
    }

    pub fn free_stdout(&mut self) {
        self.stdout = None;
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn writeln(&mut self, msg: &str) -> Result<(), std::io::Error> {
        if self.stdout.is_none() {
            return Ok(());
        }
        writeln!(self.stdout.as_mut().unwrap(), "{}\r", msg)
    }

    pub fn clean_after_cursor(&self) {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn reset_cursor_position(&self) {
        print!("{}", termion::cursor::Goto(self.x, self.y));
    }

    pub fn set_cursor_first_choice(&self) {
        let p = self.y.saturating_add(1);
        print!("{}", termion::cursor::Goto(self.x, p));
    }

    pub fn move_cursor_up(&self, position: usize) {
        let y = (self.y + 1 + position as u16).saturating_sub(1);
        print!("{}", termion::cursor::Goto(self.x, y));
    }

    pub fn move_cursor_down(&self, position: usize) {
        let y = (self.y + position as u16).saturating_add(1);
        print!("{}", termion::cursor::Goto(self.x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn read_line() -> String {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Cannot read from terminal");
        line
    }
}
