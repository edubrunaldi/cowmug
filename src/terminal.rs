use std::io::{self, stdout, Write};
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

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

    pub fn clean_after_cursor() {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn reset_cursor_position(&self) {
        print!("{}", termion::cursor::Goto(self.x, self.y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn set_green_color() {
        print!("{}", color::Fg(color::Green));
    }
    pub fn reset_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
